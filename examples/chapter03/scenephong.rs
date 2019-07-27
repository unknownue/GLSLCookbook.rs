
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::Torus;
use cookbook::{Mat4F, Mat3F, Vec3F, Vec4F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::{Surface, uniform, implement_uniform_block};


#[derive(Debug)]
pub struct ScenePhong {

    program: glium::Program,

    torus: Torus,
    materials: UniformBuffer<MaterialInfo>,
    lights   : UniformBuffer<LightInfo>,

    view       : Mat4F,
    model      : Mat4F,
    projection : Mat4F,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct LightInfo {
    LightPosition: [f32; 4],
    // This padding trick is needed here. See https://github.com/glium/glium/issues/1203 for detail.
    La: [f32; 3], _padding1: f32,
    Ld: [f32; 3], _padding2: f32,
    Ls: [f32; 3],
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct MaterialInfo {
    Ka: [f32; 3], _padding1: f32,
    Kd: [f32; 3], _padding2: f32,
    Ks: [f32; 3],
    Shininess: f32,
}


impl Scene for ScenePhong {

    fn new(display: &impl Facade) -> GLResult<ScenePhong> {

        // Shader Program ------------------------------------------------------------
        let program = ScenePhong::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // cookbook::utils::print_active_uniforms(&program);
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let torus = Torus::new(display, 0.7, 0.3, 50, 50)?;
        // ----------------------------------------------------------------------------


        // Initialize MVP -------------------------------------------------------------
        let model = Mat4F::identity()
            .rotated_x(-35.0_f32.to_radians())
            .rotated_y( 35.0_f32.to_radians());
        let view = Mat4F::look_at_rh(Vec3F::new(0.0, 0.0, 2.0), Vec3F::zero(), Vec3F::unit_y());
        let projection = Mat4F::identity();
        // ----------------------------------------------------------------------------


        // Initialize Uniforms --------------------------------------------------------
        let world_light = Vec4F::new(5.0, 5.0, 2.0, 1.0);

        glium::implement_uniform_block!(LightInfo, LightPosition, La, Ld, Ls);
        let lights = UniformBuffer::immutable(display, LightInfo {
            LightPosition: (view * world_light).into_array(),
            La: [0.4_f32, 0.4, 0.4],
            Ld: [1.0_f32, 1.0, 1.0],
            Ls: [1.0_f32, 1.0, 1.0], ..Default::default()
        }).map_err(BufferCreationErrorKind::UniformBlock)?;

        glium::implement_uniform_block!(MaterialInfo, Ka, Kd, Ks, Shininess);
        let materials = UniformBuffer::immutable(display, MaterialInfo {
            Ka: [0.9_f32, 0.5, 0.3],
            Kd: [0.9_f32, 0.5, 0.3],
            Ks: [0.8_f32, 0.8, 0.8],
            Shininess: 100.0_f32, ..Default::default()
        }).map_err(BufferCreationErrorKind::UniformBlock)?;

        // cookbook::utils::print_active_uniform_blocks(&program);
        // ----------------------------------------------------------------------------


        let scene = ScenePhong { program, torus, materials, lights, view, model, projection };
        Ok(scene)
    }

    fn update(&mut self, _delta_time: f32) {
        // nothing to do, just keep it empty
    }

    fn render(&self, frame: &mut glium::Frame) -> GLResult<()> {

        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let mv: Mat4F = self.view * self.model;
        let uniforms = uniform! {
            LightInfo: &self.lights,
            MaterialInfo: &self.materials,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        frame.clear_color_srgb(0.5, 0.5, 0.5, 1.0);
        frame.clear_depth(1.0);

        self.torus.render(frame, &self.program, &draw_params, &uniforms)
    }

    fn resize(&mut self, width: u32, height: u32) {

        self.projection = Mat4F::perspective_rh_zo(70.0_f32.to_radians(), width as f32 / height as f32, 0.3, 100.0);
    }

    fn is_animating(&self) -> bool { false }
    fn toggle_animation(&mut self) {}
}


impl ScenePhong {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/phong.vert.glsl");
        let fragment_shader_code = include_str!("shaders/phong.frag.glsl");

        // let vertex_shader_code   = include_str!("shaders/function.vert.glsl");
        // let fragment_shader_code = include_str!("shaders/function.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
