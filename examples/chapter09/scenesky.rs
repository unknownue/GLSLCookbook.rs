
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind};
use cookbook::objects::Quad;
use cookbook::noise;
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::texture::texture2d::Texture2d;
use glium::texture::MipmapsOption;
use glium::{Surface, uniform};


#[derive(Debug)]
pub struct SceneSky {

    program: glium::Program,

    quad: Quad,
    noise_tex: Texture2d,
}


impl Scene for SceneSky {

    fn new(display: &impl Facade) -> GLResult<SceneSky> {

        // Shader Program ------------------------------------------------------------
        let program = SceneSky::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------

        // Initialize Mesh ------------------------------------------------------------
        let quad = Quad::new(display)?;
        // ----------------------------------------------------------------------------

        // Initialize Textures --------------------------------------------------------
        let noise_tex = noise::generate_2d_texture(display, 6.0, 0.5, 128, 128, MipmapsOption::NoMipmap)?;
        // ----------------------------------------------------------------------------

        let scene = SceneSky {
            program,
            quad, noise_tex,
        };
        Ok(scene)
    }

    fn update(&mut self, _delta_time: f32) {}

    fn render(&mut self, frame: &mut glium::Frame) -> GLResult<()> {

        frame.clear_color(0.5, 0.5, 0.5, 1.0);
        frame.clear_depth(1.0);

        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let uniforms = uniform! {
            NoiseTex: self.noise_tex.sampled()
                .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat)
                .minify_filter(glium::uniforms::MinifySamplerFilter::Linear)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
        };

        self.quad.render(frame, &self.program, &draw_params, &uniforms)
        // -------------------------------------------------------------------------
    }

    fn resize(&mut self, _display: &impl Facade, _width: u32, _height: u32) -> GLResult<()> {
        Ok(())
    }

    fn is_animating(&self) -> bool { false }
    fn toggle_animation(&mut self) {}
}


impl SceneSky {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/sky.vert.glsl");
        let fragment_shader_code = include_str!("shaders/sky.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
