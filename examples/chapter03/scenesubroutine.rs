
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::Teapot;
use cookbook::{Mat4F, Mat3F, Vec3F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::{Surface, uniform, implement_uniform_block};


#[derive(Debug)]
pub struct SceneSubroutine {

    program: glium::Program,

    teapot: Teapot,
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


impl Scene for SceneSubroutine {

    fn new(display: &impl Facade) -> GLResult<SceneSubroutine> {

        // Shader Program ------------------------------------------------------------
        let program = SceneSubroutine::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let teapot = Teapot::new(display, 13, Mat4F::identity())?;
        // ----------------------------------------------------------------------------


        // Initialize MVP -------------------------------------------------------------
        let model = Mat4F::identity();
        let view = Mat4F::look_at_rh(Vec3F::new(0.0, 3.5, 9.0), Vec3F::zero(), Vec3F::unit_y());
        let projection = Mat4F::identity();
        // ----------------------------------------------------------------------------


        // Initialize Uniforms --------------------------------------------------------
        glium::implement_uniform_block!(LightInfo, LightPosition, La, Ld, Ls);
        let lights = UniformBuffer::immutable(display, LightInfo {
            LightPosition: [0.0, 0.0, 0.0, 1.0],
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
        // ----------------------------------------------------------------------------


        let scene = SceneSubroutine { program, teapot, materials, lights, view, model, projection };
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

        frame.clear_color_srgb(0.5, 0.5, 0.5, 1.0);
        frame.clear_depth(1.0);


        // Render teapot with Phong shading. --------------------------
        let model = Mat4F::translation_3d(Vec3F::new(-3.0, -1.5, 0.0))
            .rotated_x(-90.0_f32.to_radians());
        let mv: Mat4F = self.view * model;

        let uniforms = uniform! {
            LightInfo: &self.lights,
            MaterialInfo: &self.materials,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
            // Set Subroutine
            shadeModel: ("phongModel", glium::program::ShaderStage::Vertex),
        };

        self.teapot.render(frame, &self.program, &draw_params, &uniforms)?;
        // -------------------------------------------------------------


        // Render teapot with Diffuse shading. --------------------------
        let model = Mat4F::translation_3d(Vec3F::new(3.0, -1.5, 0.0))
            .rotated_x(-90.0_f32.to_radians());
        let mv: Mat4F = self.view * model;

        let uniforms = uniform! {
            LightInfo: &self.lights,
            MaterialInfo: &self.materials,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
            // Set Subroutine
            shadeModel: ("diffuseOnly", glium::program::ShaderStage::Vertex),
        };

        self.teapot.render(frame, &self.program, &draw_params, &uniforms)
        // -------------------------------------------------------------
    }

    fn resize(&mut self, width: u32, height: u32) {

        self.projection = Mat4F::perspective_rh_zo(50.0_f32.to_radians(), width as f32 / height as f32, 0.3, 100.0);
    }

    fn is_animating(&self) -> bool { false }
    fn toggle_animation(&mut self) {}
}

impl SceneSubroutine {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/subroutine.vert.glsl");
        let fragment_shader_code = include_str!("shaders/subroutine.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}