
use glium::glutin;
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;

use crate::scene::Scene;
use crate::utils;
use crate::error::{GLResult, GLError};
use crate::timer::Timer;

use std::collections::HashMap;


pub struct SceneRunner;

#[derive(Debug, Clone)]
pub struct SceneParams {

    title: String,
    width: u32,
    height: u32,
    samples: u16,
    
    is_debug: bool, // Set true to enable debug messages
}

impl From<(String, u32, u32, u16, bool)> for SceneParams {

    fn from(v: (String, u32, u32, u16, bool)) -> SceneParams {
        SceneParams { title: v.0, width: v.1, height: v.2, samples: v.3, is_debug: v.4 }
    }
}

impl SceneRunner {

    #[cfg(not(target_os = "macos"))]
    fn with_context_gl_request<T>(builder: glutin::ContextBuilder<T>) -> glutin::ContextBuilder<T>
        where T: glutin::ContextCurrentState {
        // Select OpenGL 4.6 on Windows and Linux.
        builder.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 6)))
    }

    #[cfg(target_os = "macos")]
    fn with_context_gl_request<T>(builder: glutin::ContextBuilder<T>) -> glutin::ContextBuilder<T>
        where T: glutin::ContextCurrentState {
        // Select OpenGL 4.1 on macOS.
        builder.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 1)))
    }

    pub fn run<S: 'static + Scene>(params: SceneParams) -> GLResult<()> {

        let params = params.into();

        let event_loop = EventLoop::new();
        let display = SceneRunner::build_display(&params, &event_loop)?;
        let mut scene = S::new(&display)?;

        if params.is_debug {
            // Ignore debug marker error if backend is not support.
            display.insert_debug_marker("Start debugging")
                .or_else(|_| { eprintln!("Current backend does not support Debug Marker"); Err(()) }).ok();
        }

        SceneRunner::resize_window(&display, &mut scene)?;

        // Enter the main loop
        SceneRunner::main_loop(event_loop, display, scene, params)
    }

    fn build_display(params: &SceneParams, event_loop: &EventLoop<()>) -> GLResult<glium::Display> {

        let wb = WindowBuilder::new() // Window Builder
            .with_title(params.title.clone())
            .with_inner_size((params.width, params.height).into())
            .with_resizable(true);
        let cb = glutin::ContextBuilder::new() // Context Builder
            .with_gl_profile(glutin::GlProfile::Core)
            .with_multisampling(params.samples);

        let display: glium::Display = if params.is_debug {
            let wc = SceneRunner::with_context_gl_request(cb) // Windows Context
                .build_windowed(wb, event_loop)
                .map_err(|_| GLError::window("Unable to create Windows context."))?;

            // Initializtion, set up debug callback
            glium::Display::with_debug(wc, glium::debug::DebugCallbackBehavior::Custom {
                callback: Box::new(utils::debug_callback),
                synchronous: false,
            }).map_err(|_| GLError::device("Unable to create OpenGL context."))?
        } else {
            let cb = SceneRunner::with_context_gl_request(cb);
            glium::Display::new(wb, cb, event_loop)
                .map_err(|_| GLError::device("Unable to create OpenGL context."))?
        };

        // Print dump info about current OpenGL context.
        utils::dump_gl_info(&display, false);

        Ok(display)
    }

    fn main_loop<S: 'static + Scene>(event_loop: EventLoop<()>, display: glium::Display, mut scene: S, params: SceneParams) -> GLResult<()> {

        use glium::glutin::event_loop::ControlFlow;
        use glium::glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent };

        // display manage the surface window.
        let mut timer = Timer::new();

        event_loop.run(move |event, _, control_flow| {

            scene.update(timer.delta_time());

            let mut frame = display.draw();
            match scene.render2(&display, &mut frame) {
                | Ok(()) => try_ops(frame.finish().map_err(GLError::rendering_finish)),
                | Err(e) => {
                    // frame.finish() must be called no matter if any error occurred.
                     try_ops(frame.finish().map_err(GLError::rendering_finish));
                     try_ops(Err(e));
                }
            }

            match event {
                | Event::WindowEvent { event, .. } => {
                    match event {
                        | WindowEvent::CloseRequested => {
                            if params.is_debug { display.insert_debug_marker("End debug").ok(); }
                            *control_flow = ControlFlow::Exit
                        },
                        | WindowEvent::KeyboardInput { input, .. } => {
                            match input {
                                | KeyboardInput { virtual_keycode, state, .. } => {
                                    match (virtual_keycode, state) {
                                        | (Some(VirtualKeyCode::Space), ElementState::Released) => {
                                            scene.toggle_animation();
                                        },
                                        | (Some(VirtualKeyCode::Escape), ElementState::Released) => {
                                            if params.is_debug { display.insert_debug_marker("End debug").ok(); }
                                            *control_flow = ControlFlow::Exit
                                        },
                                        | _ => {},
                                    }
                                },
                            }
                        },
                        | WindowEvent::Resized(_new_size) => {
                            try_ops(SceneRunner::resize_window(&display, &mut scene));
                        },
                        | _ => {},
                    }
                },
                _ => (),
            }

            timer.tick_frame();
        })
    }

    fn resize_window(display: &glium::Display, scene: &mut impl Scene) -> GLResult<()> {
        let (new_width, new_height) = display.get_framebuffer_dimensions();
        scene.resize(display, new_width, new_height)
    }

    pub fn print_help_info(program_name: &str, candidate_scenes: &HashMap<String, String>) {
        println!("-------------------------------------------------------------");
        println!("Usage: {} recipe-name", program_name);
        println!("Candidate recipe names: ");

        let max_recipe_length: usize = candidate_scenes.iter()
            .map(|s| s.0.len()).max().unwrap_or(10);

        for scene in candidate_scenes {
            println!("\t{:width$}: {}", scene.0, scene.1, width = max_recipe_length + 1);
        }
        println!("\n-------------------------------------------------------------");
    }

    pub fn parse_command_line_args(candidate_scenes: &HashMap<String, String>) -> GLResult<(String, String)> {

        let args: Vec<String> = std::env::args().collect();

        if args.len() < 2 {
            SceneRunner::print_help_info(&args[0], candidate_scenes);
            let help_message = "You must provide at least 2 arguments;\nFor example: $ cargo r --example chapter01 basic\n";
            Err(GLError::args(help_message))
        } else {
            if candidate_scenes.iter().any(|s| s.0 == &args[1]) {
                Ok((args[1].clone(), candidate_scenes[&args[1]].clone()))
            } else {
                SceneRunner::print_help_info(&args[0], candidate_scenes);

                let help_message = format!("Unknown recipe: {}\n", args[1]);
                Err(GLError::args(help_message))
            }
        }
    }

}


#[inline]
fn try_ops(result: GLResult<()>) {
    match result {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{}", e);
            panic!()
        }
    }
}
