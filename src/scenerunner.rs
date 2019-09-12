
use glium::glutin;
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;

use crate::scene::Scene;
use crate::utils;
use crate::error::{GLResult, GLError};
use crate::timer::Timer;

use std::collections::HashMap;


pub struct SceneRunner {

    event_loop: EventLoop<()>,

    title: String,
    initial_width : u32,
    initial_height: u32,
    samples: u16,

    is_debug: bool, // Set true to enable debug messages
}

impl SceneRunner {

    pub fn new(title: impl Into<String>, width: u32, height: u32, is_debug: bool, samples: u16) -> GLResult<SceneRunner> {

        let event_loop = EventLoop::new();

        let title = title.into();
        let initial_width  = width;
        let initial_height = height;

        let runner = SceneRunner { event_loop, title, initial_width, initial_height, samples, is_debug };
        Ok(runner)
    }

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

    pub fn run<S: 'static + Scene>(self) -> GLResult<()> {

        let display = self.build_display()?;
        let mut scene = S::new(&display)?;

        if self.is_debug {
            // Ignore debug marker error if backend is not support.
            display.insert_debug_marker("Start debugging")
                .or_else(|_| { eprintln!("Current backend does not support Debug Marker"); Err(()) }).ok();
        }

        SceneRunner::resize_window(&display, &mut scene)?;

        // Enter the main loop
        SceneRunner::main_loop(self, display, scene)
    }

    fn build_display(&self) -> GLResult<glium::Display> {

        let wb = WindowBuilder::new() // Window Builder
            .with_title(self.title.clone())
            .with_inner_size((self.initial_width, self.initial_height).into())
            .with_resizable(true);
        let cb = glutin::ContextBuilder::new() // Context Builder
            .with_gl_profile(glutin::GlProfile::Core)
            .with_multisampling(self.samples);

        let display: glium::Display = if self.is_debug {
            let wc = SceneRunner::with_context_gl_request(cb) // Windows Context
                .build_windowed(wb, &self.event_loop)
                .map_err(|_| GLError::window("Unable to create Windows context."))?;

            // Initializtion, set up debug callback
            glium::Display::with_debug(wc, glium::debug::DebugCallbackBehavior::Custom {
                callback: Box::new(utils::debug_callback),
                synchronous: false,
            }).map_err(|_| GLError::device("Unable to create OpenGL context."))?
        } else {
            let cb = SceneRunner::with_context_gl_request(cb);
            glium::Display::new(wb, cb, &self.event_loop)
                .map_err(|_| GLError::device("Unable to create OpenGL context."))?
        };

        // Print dump info about current OpenGL context.
        utils::dump_gl_info(&display, false);

        Ok(display)
    }

    fn main_loop<S: 'static + Scene>(runner: SceneRunner, display: glium::Display, mut scene: S) -> GLResult<()> {

        use glium::glutin::event_loop::ControlFlow;
        use glium::glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent };

        // display manage the surface window.
        let mut timer = Timer::new();
        let is_debug = runner.is_debug;
        let event_loop = runner.event_loop;

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
                            if is_debug { display.insert_debug_marker("End debug").ok(); }
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
                                            if is_debug { display.insert_debug_marker("End debug").ok(); }
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
