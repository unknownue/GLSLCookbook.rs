
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::Plane;
use cookbook::texture::load_bytes_to_texture;
use cookbook::{Mat4F, Mat3F, Vec3F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::texture::texture2d::Texture2d;
use glium::{Surface, uniform, implement_uniform_block};


#[derive(Debug)]
pub struct SceneSamplerObj {

    program: glium::Program,

    plane: Plane,
    checkerboard_tex: Texture2d,

    material_buffer: UniformBuffer<MaterialInfo>,
    light_buffer   : UniformBuffer<LightInfo>,

    view       : Mat4F,
    projection : Mat4F,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct LightInfo {
    LightPosition: [f32; 4],
    L : [f32; 3], _padding1: f32,
    La: [f32; 3],
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct MaterialInfo {
    Ks: [f32; 3],
    Shininess: f32,
}


impl Scene for SceneSamplerObj {

    fn new(display: &impl Facade) -> GLResult<SceneSamplerObj> {

        // Shader Program ------------------------------------------------------------
        let program = SceneSamplerObj::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let plane = Plane::new(display, 10.0, 10.0, 1, 1, 1.0, 1.0)?;
        // ----------------------------------------------------------------------------

        // Initialize Textures --------------------------------------------------------
        // A simple 128x128 checkerboard texture
        const W: usize = 128;
        const H: usize = 128;
        const CHECK_SIZE: usize = 4;

        let mut checkerboard: Vec<u8> = Vec::with_capacity(W * H * CHECK_SIZE);
        for r in 0..H {
            for c in 0..W {
                let color = if ((c / CHECK_SIZE) + (r / CHECK_SIZE)) % 2 == 0 { 0 } else { 255 };
                checkerboard.extend(&[color, color, color, 255]);
            }
        }
        let checkerboard_tex = load_bytes_to_texture(display, checkerboard, W, H)?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let view = Mat4F::look_at_rh(Vec3F::new(0.0, 0.1, 6.0), Vec3F::zero(), Vec3F::unit_y());
        let projection = Mat4F::identity();
        // ----------------------------------------------------------------------------


        // Initialize Uniforms --------------------------------------------------------
        glium::implement_uniform_block!(LightInfo, LightPosition, L, La);
        let light_buffer = UniformBuffer::immutable(display, LightInfo {
            LightPosition: [0.0_f32, 20.0, 0.0, 1.0],
            L: [1.0_f32, 1.0, 1.0],
            La: [0.2_f32, 0.2, 0.2], ..Default::default()
        }).map_err(BufferCreationErrorKind::UniformBlock)?;

        glium::implement_uniform_block!(MaterialInfo, Ks, Shininess);
        let material_buffer = UniformBuffer::immutable(display, MaterialInfo {
            Ks: [0.95, 0.95, 0.95],
            Shininess: 100.0,
        }).map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------

        let scene = SceneSamplerObj {
            program,
            plane, checkerboard_tex,
            material_buffer, light_buffer,
            view, projection,
        };
        Ok(scene)
    }

    fn update(&mut self, _delta_time: f32) {
        // nothing to do, just keep it empty
    }

    fn render(&mut self, frame: &mut glium::Frame) -> GLResult<()> {

        frame.clear_color_srgb(0.9, 0.9, 0.9, 1.0);
        frame.clear_depth(1.0);

        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        // Render Plane ----------------------------------------------------------
        let model = Mat4F::translation_3d(Vec3F::new(-5.01, 0.0, 0.0))
            .rotated_x(10.0_f32.to_radians());
        let mv: Mat4F = self.view * model;

        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            Tex1: self.checkerboard_tex.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.plane.render(frame, &self.program, &draw_params, &uniforms)?;
        // -------------------------------------------------------------------------

        // Render Plane Again ------------------------------------------------------
        let model = Mat4F::translation_3d(Vec3F::new(5.01, 0.0, 0.0))
            .rotated_x(10.0_f32.to_radians());
        let mv: Mat4F = self.view * model;

        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            Tex1: self.checkerboard_tex.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Linear)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.plane.render(frame, &self.program, &draw_params, &uniforms)
        // -------------------------------------------------------------------------
    }

    fn resize(&mut self, _display: &impl Facade, width: u32, height: u32) {

        self.projection = Mat4F::perspective_rh_zo(60.0_f32.to_radians(), width as f32 / height as f32, 0.3, 100.0);
    }

    fn is_animating(&self) -> bool { false }
    fn toggle_animation(&mut self) {}
}


impl SceneSamplerObj {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/texture.vert.glsl");
        let fragment_shader_code = include_str!("shaders/texture.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
