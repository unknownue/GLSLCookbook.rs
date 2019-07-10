
use glium::glutin;
use crate::scene::Scene;
use crate::utils;
use crate::error::{GLResult, GLError};

use std::collections::HashMap;


pub struct SceneRunner {

    display: glium::backend::glutin::Display, // display manage the surface window.
    events_loop: glutin::EventsLoop,

    fb_width  : u32,
    fb_height : u32,
}

impl SceneRunner {

    pub fn new(title: impl Into<String>, width: u32, height: u32, samples: u16) -> SceneRunner {

        let events_loop = glutin::EventsLoop::new();

        // TODO: unwrap() is not handle here.
        let wb = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions((width, height).into())
            .with_resizable(true);
        let cb = glutin::ContextBuilder::new()
            .with_gl_profile(glutin::GlProfile::Core)
            .with_multisampling(samples);
        let cb = SceneRunner::with_context_gl_request(cb);

        // TODO: handle expect().
        let display = glium::Display::new(wb, cb, &events_loop)
            .expect("Unable to create OpenGL context.");

        // Print dump info about current OpenGL context.
        utils::dump_gl_info(&display, false);

        // TODO: Get Framebuffer size.
        let (fb_width, fb_height) = display.get_framebuffer_dimensions();

        // Initialization
        // TODO: Set up debug calllback


        SceneRunner { display, events_loop, fb_width, fb_height }
    }

    #[cfg(not(target_os = "macos"))]
    fn with_context_gl_request(builder: glium::ContextBuilder) -> glutin::ContextBuilder {
        // Select OpenGL 4.6 for windows and linux.
        builder.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl), (4, 6))
    }


    #[cfg(target_os = "macos")]
    fn with_context_gl_request(builder: glutin::ContextBuilder) -> glutin::ContextBuilder {
        // Select OpenGL 4.1 for windows and linux.
        builder.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 1)))
    }

    // TODO: Return a Result indicating the Running status.
    pub fn run(&mut self, scene: &mut impl Scene) -> GLResult<()> {

        scene.set_dimension(self.fb_width, self.fb_height);
        scene.resize(self.fb_width, self.fb_height);

        // Enter the main loop
        self.main_loop(scene)

        // TODO: Insert End Debug Messager.

    }

    fn main_loop(&mut self, scene: &mut impl Scene) -> GLResult<()> {

        let mut should_close = false;

        while !should_close {

            // Find equivalent call to glfwGetTime()
            let time = 0.0;

            scene.update(time);
            scene.render(&self.display)?;

            self.events_loop.poll_events(|ev| {
                match ev {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        | glutin::WindowEvent::CloseRequested => should_close = true,
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
        }

        Ok(())
    }

    pub fn print_help_info(program_name: &str, candidate_scenes: &HashMap<String, String>) {
        println!("Usage: {} recipe-name", program_name);
        println!("Recipe names: ");

        for scene in candidate_scenes {
            println!("  {}: {}", scene.0, scene.1);
        }
    }

    pub fn parse_command_line_args(candidate_scenes: &HashMap<String, String>) -> GLResult<String> {

        let args: Vec<String> = std::env::args().collect();

        if args.len() < 2 {
            SceneRunner::print_help_info(&args[0], candidate_scenes);
            // TODO: Return Error type.
            let help_message = "You must provide at least 2 arguments;\nFor example: $ cargo r --example chapter01 basic\n";
            Err(GLError::args(help_message))
        } else {
            if candidate_scenes.iter().any(|s| s.0 == &args[1]) {
                Ok(args[1].clone())
            } else {
                println!("Unknown recipe: {}\n", args[1]);
                SceneRunner::print_help_info(&args[0], candidate_scenes);

                let help_message = format!("Unknown recipe: {}\n", args[1]);
                Err(GLError::args(help_message))
            }
        }
    }

    pub fn display_backend(&self) -> &glium::backend::glutin::Display {
        &self.display
    }
}
