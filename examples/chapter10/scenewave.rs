
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::Plane;
use cookbook::{Mat4F, Mat3F, Vec3F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::{Surface, uniform, implement_uniform_block};


#[derive(Debug)]
pub struct SceneWave {

    program: glium::Program,

    plane: Plane,

    material_buffer: UniformBuffer<MaterialInfo>,
    light_buffer   : UniformBuffer<LightInfo>,

    time: f32,
    angle: f32,
    is_animate: bool,

    projection : Mat4F,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct LightInfo {
    LightPosition: [f32; 4],
    Intensity: [f32; 3], _padding1: f32,
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


impl Scene for SceneWave {

    fn new(display: &impl Facade) -> GLResult<SceneWave> {

        // Shader Program ------------------------------------------------------------
        let program = SceneWave::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let plane = Plane::new(display, 13.0, 10.0, 200, 2, 1.0, 1.0)?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let projection = Mat4F::identity();
        let angle = std::f32::consts::PI / 2.0;
        let is_animate = true;
        let time = 0.0;
        // ----------------------------------------------------------------------------

        // Initialize Uniforms --------------------------------------------------------
        glium::implement_uniform_block!(LightInfo, LightPosition, Intensity);
        let light_buffer = UniformBuffer::immutable(display, LightInfo {
            LightPosition: [0.0_f32, 0.0, 0.0, 1.0],
            Intensity: [1.0_f32, 1.0, 1.0], ..Default::default()
        }).map_err(BufferCreationErrorKind::UniformBlock)?;

        glium::implement_uniform_block!(MaterialInfo, Ka, Kd, Ks, Shininess);
        let material_buffer = UniformBuffer::immutable(display, MaterialInfo {
            Ka: [0.2, 0.2, 0.2],
            Kd: [0.9, 0.5, 0.3],
            Ks: [0.8, 0.8, 0.8],
            Shininess: 100.0, ..Default::default()
        }).map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------

        let scene = SceneWave {
            program, plane,
            material_buffer, light_buffer,
            projection, angle, is_animate, time,
        };
        Ok(scene)
    }

    fn update(&mut self, delta_time: f32) {
        
        if self.is_animate {
            self.time += delta_time;
        }
    }

    fn render(&mut self, _display: &impl Facade, frame: &mut glium::Frame) -> GLResult<()> {

        frame.clear_color_srgb(0.5, 0.5, 0.5, 1.0);
        frame.clear_depth(1.0);

        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        // Render Cube ----------------------------------------------------------
        let view = Mat4F::look_at_rh(Vec3F::new(10.0 * self.angle.cos(), 4.0, 10.0 * self.angle.sin()), Vec3F::zero(), Vec3F::unit_y());
        let model = Mat4F::rotation_x(50.0_f32.to_radians())
            .rotated_z(-10.0_f32.to_radians());
        let mv: Mat4F = view * model;

        let uniforms = uniform! {
            Time: self.time,
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.plane.render(frame, &self.program, &draw_params, &uniforms)
        // -------------------------------------------------------------------------
    }

    fn resize(&mut self, _display: &impl Facade, width: u32, height: u32) -> GLResult<()> {

        self.projection = Mat4F::perspective_rh_zo(60.0_f32.to_radians(), width as f32 / height as f32, 0.3, 100.0);
        Ok(())
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }
    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate;
    }
}


impl SceneWave {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/wave.vert.glsl");
        let fragment_shader_code = include_str!("shaders/wave.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
