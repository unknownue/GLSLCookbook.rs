
use glium::glutin;
use crate::scene::Scene;


pub struct SceneRunner {

    display: glium::backend::glutin::Display, // display manage the surface window.
    events_loop : glutin::EventsLoop,

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

        // TODO: Print dump info about current OpenGL context.
        // GLUtils::dumpGLInfo();

        // TODO: handle expect().
        let display = glium::Display::new(wb, cb, &events_loop)
            .expect("Unable to create OpenGL context.");

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
    pub fn run(&mut self, scene: &mut impl Scene) {

        scene.set_dimension(self.fb_width, self.fb_height);
        scene.resize(self.fb_width, self.fb_height);

        // Enter the main loop
        self.main_loop(scene);

        // TODO: Insert End Debug Messager.

    }

    fn main_loop(&mut self, scene: &mut impl Scene) {

        let mut should_close = false;

        while !should_close {

            // Find equivalent call to glfwGetTime()
            let time = 0.0;

            scene.update(time);
            scene.render(&self.display);

            // TODO: Vertify if the function swap_buffer is necessary.
            // TODO: handle unwrap().
            // self.display.swap_buffers().unwrap();

            self.events_loop.poll_events(|ev| {
                match ev {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::CloseRequested => should_close = true,

                        // TODO: Check for Escape key event.
                        // TODO: Check for Space Key event to toggle animation.
                        _ => (),
                    },
                    _ => (),
                }
            });
        }

    }

    // TODO: printHelpInfo function is not implemented yet.
    pub fn print_help_info(&self) {
        unimplemented!()
    }
    // TODO: parseCLArgs function is not implemented yet.
    pub fn parse_command_line_args() -> String {
        unimplemented!()
    }

    pub fn display_backend(&self) -> &glium::backend::glutin::Display {
        &self.display
    }
}
