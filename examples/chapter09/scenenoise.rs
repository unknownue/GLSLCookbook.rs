
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind};
use cookbook::objects::Quad;
use cookbook::noise;
use cookbook::Mat4F;
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::texture::texture2d::Texture2d;
use glium::texture::MipmapsOption;
use glium::{Surface, uniform};


#[derive(Debug)]
pub struct SceneNoise {

    program: glium::Program,

    quad: Quad,
    noise_tex: Texture2d,

    projection : Mat4F,
}


impl Scene for SceneNoise {

    fn new(display: &impl Facade) -> GLResult<SceneNoise> {

        // Shader Program ------------------------------------------------------------
        let program = SceneNoise::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------

        // Initialize Mesh ------------------------------------------------------------
        let quad = Quad::new_with_texcoord_scale(display, 2.0)?;
        // ----------------------------------------------------------------------------

        // Initialize Textures --------------------------------------------------------
        let noise_tex = noise::generate_periodic_2d_texture(display, 4.0, 0.5, 128, 128, MipmapsOption::NoMipmap)?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let projection = Mat4F::identity();
        // ----------------------------------------------------------------------------

        let scene = SceneNoise {
            program,
            quad, noise_tex,
            projection,
        };
        Ok(scene)
    }

    fn update(&mut self, _delta_time: f32) {}

    fn render(&mut self, frame: &mut glium::Frame) -> GLResult<()> {

        frame.clear_color(1.0, 1.0, 1.0, 1.0);
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
            MVP: (self.projection * Mat4F::identity()).into_col_arrays(),
        };

        self.quad.render(frame, &self.program, &draw_params, &uniforms)
        // -------------------------------------------------------------------------
    }

    fn resize(&mut self, _display: &impl Facade, _width: u32, _height: u32) -> GLResult<()> {

        const C: f32 = 0.75;
        self.projection = Mat4F::orthographic_rh_zo(vek::FrustumPlanes {
            left: -2.0 * C, right: 2.0 * C, bottom: -1.5 * C, top: 1.5 * C,
            near: 0.1, far: 100.0,
        });
        Ok(())
    }

    fn is_animating(&self) -> bool { false }
    fn toggle_animation(&mut self) {}
}


impl SceneNoise {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/noisetex.vert.glsl");
        let fragment_shader_code = include_str!("shaders/noisetex.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
