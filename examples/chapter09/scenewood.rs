
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind};
use cookbook::objects::Quad;
use cookbook::noise;
use cookbook::{Mat4F, Vec3F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::texture::texture2d::Texture2d;
use glium::texture::MipmapsOption;
use glium::{Surface, uniform};


#[derive(Debug)]
pub struct SceneWood {

    program: glium::Program,

    quad: Quad,
    noise_tex: Texture2d,

    slice: Mat4F,
}


impl Scene for SceneWood {

    fn new(display: &impl Facade) -> GLResult<SceneWood> {

        // Shader Program ------------------------------------------------------------
        let program = SceneWood::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------

        // Initialize Mesh ------------------------------------------------------------
        let quad = Quad::new(display)?;
        // ----------------------------------------------------------------------------

        // Initialize Textures --------------------------------------------------------
        let noise_tex = noise::generate_2d_texture(display, 4.0, 0.5, 128, 128, MipmapsOption::NoMipmap)?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let slice = Mat4F::translation_3d(Vec3F::new(-0.35, -0.5, 2.0))
            .scaled_3d(Vec3F::new(40.0, 40.0, 1.0))
            .rotated_z(-20.0_f32.to_radians())
            .rotated_x(10.0_f32.to_radians());
        // let slice = Mat4F::rotation_x(10.0_f32.to_radians())
        //     .rotated_z(-20.0_f32.to_radians())
        //     .scaled_3d(Vec3F::new(40.0, 40.0, 1.0))
        //     .translated_3d(Vec3F::new(-0.35, -0.5, 2.0));
        // ----------------------------------------------------------------------------

        let scene = SceneWood {
            program,
            quad, noise_tex,
            slice,
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
            Slice: self.slice.into_col_arrays(),
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


impl SceneWood {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/wood.vert.glsl");
        let fragment_shader_code = include_str!("shaders/wood.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
