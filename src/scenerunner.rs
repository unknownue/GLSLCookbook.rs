
use glium::glutin;
use crate::scene::Scene;
use crate::utils;
use crate::error::{GLResult, GLError};
use crate::timer::Timer;

use std::collections::HashMap;


pub struct SceneRunner {

    display: glium::Display, // display manage the surface window.
    events_loop: glutin::EventsLoop,

    fb_width  : u32,
    fb_height : u32,

    is_debug: bool, // Set true to enable debug messages
}

impl SceneRunner {

    pub fn new(title: impl Into<String>, width: u32, height: u32, is_debug: bool, samples: u16) -> GLResult<SceneRunner> {

        let events_loop = glutin::EventsLoop::new();

        let wb = glutin::WindowBuilder::new() // Window Builder
            .with_title(title)
            .with_dimensions((width, height).into())
            .with_resizable(true);
        let cb = glutin::ContextBuilder::new() // Context Builder
            .with_gl_profile(glutin::GlProfile::Core)
            .with_multisampling(samples);

        let display: glium::Display = if is_debug {
            let wc = SceneRunner::with_context_gl_request(cb) // Windows Context
                .build_windowed(wb, &events_loop)
                .map_err(|_| GLError::window("Unable to create Windows context."))?;

            // Initializtion, set up debug callback
            glium::Display::with_debug(wc, glium::debug::DebugCallbackBehavior::Custom {
                callback: Box::new(utils::debug_callback),
                synchronous: false,
            }).map_err(|_| GLError::device("Unable to create OpenGL context."))?
        } else {
            let cb = SceneRunner::with_context_gl_request(cb);
            glium::Display::new(wb, cb, &events_loop)
                .map_err(|_| GLError::device("Unable to create OpenGL context."))?
        };

        // Print dump info about current OpenGL context.
        utils::dump_gl_info(&display, false);

        // Get Framebuffer size.
        let (fb_width, fb_height) = display.get_framebuffer_dimensions();

        if is_debug {
            // Ignore debug marker error if backend is not support.
            display.insert_debug_marker("Start debugging")
                .or_else(|_| { eprintln!("Current backend does not support Debug Marker"); Err(()) }).ok();
        }

        let runner = SceneRunner { display, events_loop, fb_width, fb_height, is_debug };
        Ok(runner)
    }

    #[cfg(not(target_os = "macos"))]
    fn with_context_gl_request(builder: glium::ContextBuilder) -> glutin::ContextBuilder {
        // Select OpenGL 4.6 on Windows and Linux.
        builder.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl), (4, 6))
    }

    #[cfg(target_os = "macos")]
    fn with_context_gl_request(builder: glutin::ContextBuilder) -> glutin::ContextBuilder {
        // Select OpenGL 4.1 on macOS.
        builder.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 1)))
    }

    pub fn run(&mut self, scene: &mut impl Scene) -> GLResult<()> {

        scene.resize(self.fb_width, self.fb_height);

        // Enter the main loop
        self.main_loop(scene)?;

        if self.is_debug {
            self.display.insert_debug_marker("End debug").ok();
        }

        Ok(())
    }

    fn main_loop(&mut self, scene: &mut impl Scene) -> GLResult<()> {

        let mut should_close  = false;
        let mut should_resize = false;

        let mut timer = Timer::new();

        while !should_close {

            scene.update(timer.delta_time());

            let mut frame = self.display.draw();
            match scene.render(&mut frame) {
                | Ok(()) => frame.finish().map_err(GLError::rendering_finish)?,
                | Err(e) => {
                    // frame.finish() must be called no matter if any error occurred.
                    frame.finish().map_err(GLError::rendering_finish)?;
                    return Err(e)
                }
            }

            self.events_loop.poll_events(|ev| {
                match ev {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        | glutin::WindowEvent::CloseRequested => should_close  = true,
                        | glutin::WindowEvent::Resized(_)     => should_resize = true,
                        | glutin::WindowEvent::KeyboardInput { input, .. } => {
                            if let Some(code) = input.virtual_keycode {
                                match code {
                                    | glium::glutin::VirtualKeyCode::Space => {
                                        scene.set_animate(true);
                                    },
                                    | glium::glutin::VirtualKeyCode::Escape => {
                                        should_close = true;
                                    },
                                    | _ => {},
                                }
                            }
                        },

                        _ => (),
                    },
                    _ => (),
                }
            });

            if should_resize {
                should_resize = false;
                self.resize_window(scene);
            }

            timer.tick_frame();
        }

        Ok(())
    }

    fn resize_window(&mut self, scene: &mut impl Scene) {
        let (new_width, new_height) = self.display.get_framebuffer_dimensions();
        self.fb_width  = new_width;
        self.fb_height = new_height;
        scene.resize(self.fb_width, self.fb_height);
    }

    pub fn print_help_info(program_name: &str, candidate_scenes: &HashMap<String, String>) {
        println!("-------------------------------------------------------------");
        println!("Usage: {} recipe-name", program_name);
        print!("Candidate recipe names: ");

        for scene in candidate_scenes {
            print!(" {}", scene.0);
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

    pub fn display_backend(&self) -> &glium::Display {
        &self.display
    }
}
