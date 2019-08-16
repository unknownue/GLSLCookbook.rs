
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{Teapot, Plane};
use cookbook::{Mat4F, Mat3F, Vec3F, Vec4F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::{Surface, uniform, implement_uniform_block};


#[derive(Debug)]
pub struct ScenePerfragment {

    program: glium::Program,

    teapot: Teapot,
    teapot_material: MaterialInfo,
    plane: Plane,
    plane_material: MaterialInfo,

    material_buffer: UniformBuffer<MaterialInfo>,
    light_buffer   : UniformBuffer<LightInfo>,

    view       : Mat4F,
    projection : Mat4F,

    angle: f32,
    is_animate: bool,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct LightInfo {
    LightPosition: [f32; 4],
    La: [f32; 3], _padding1: f32,
    L: [f32; 3],
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


impl Scene for ScenePerfragment {

    fn new(display: &impl Facade) -> GLResult<ScenePerfragment> {

        // Shader Program ------------------------------------------------------------
        let program = ScenePerfragment::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let teapot = Teapot::new(display, 14, Mat4F::identity())?;
        let teapot_material = MaterialInfo {
            Ka: [0.1_f32, 0.1, 0.1],
            Kd: [0.9_f32, 0.5, 0.3],
            Ks: [0.95_f32, 0.95, 0.95],
            Shininess: 100.0_f32, ..Default::default()
        };
        let plane = Plane::new(display, 50.0, 50.0, 1, 1, 1.0, 1.0)?;
        let plane_material = MaterialInfo {
            Ka: [0.1_f32, 0.1, 0.1],
            Kd: [0.7_f32, 0.7, 0.7],
            Ks: [0.9_f32, 0.9, 0.9],
            Shininess: 180.0_f32, ..Default::default()
        };
        // ----------------------------------------------------------------------------


        // Initialize MVP -------------------------------------------------------------
        let view = Mat4F::look_at_rh(Vec3F::new(0.0, 3.0, 5.0), Vec3F::new(0.0, 0.75, 0.0), Vec3F::unit_y());
        let projection = Mat4F::identity();
        let angle = 0.957283;
        let is_animate = true;
        // ----------------------------------------------------------------------------


        // Initialize Uniforms --------------------------------------------------------
        glium::implement_uniform_block!(LightInfo, LightPosition, La, L);
        let light_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;

        glium::implement_uniform_block!(MaterialInfo, Ka, Kd, Ks, Shininess);
        let material_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------


        let scene = ScenePerfragment {
            program,
            teapot, teapot_material, plane, plane_material,
            material_buffer, light_buffer,
            view, projection, angle, is_animate,
        };
        Ok(scene)
    }

    fn update(&mut self, delta_time: f32) {

        const TWO_PI: f32 = std::f32::consts::PI * 2.0;

        if self.is_animating() {
            self.angle = (self.angle + delta_time) % TWO_PI;
        }
    }

    fn render(&mut self, frame: &mut glium::Frame) -> GLResult<()> {

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

        self.light_buffer.write(&LightInfo {
            LightPosition: (self.view * Vec4F::new(10.0 * self.angle.cos(), 3.0, 10.0 * self.angle.sin(), 1.0)).into_array(),
            La: [0.1, 0.1, 0.1],
            L: [0.9, 0.9, 0.9], ..Default::default()
        });

        // Render Teapot ----------------------------------------------------------
        self.material_buffer.write(&self.teapot_material);

        let model = Mat4F::rotation_x(-90.0_f32.to_radians());
        let mv: Mat4F = self.view * model;
        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.teapot.render(frame, &self.program, &draw_params, &uniforms)?;
        // -------------------------------------------------------------------------

        // Render Plane ----------------------------------------------------------
        self.material_buffer.write(&self.plane_material);

        let model = Mat4F::translation_3d(Vec3F::new(0.0, -0.45, 0.0));
        let mv: Mat4F = self.view * model;
        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.plane.render(frame, &self.program, &draw_params, &uniforms)
        // -------------------------------------------------------------------------
    }

    fn resize(&mut self, width: u32, height: u32) {

        self.projection = Mat4F::perspective_rh_zo(60.0_f32.to_radians(), width as f32 / height as f32, 0.3, 100.0);
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }
    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate;
    }
}


impl ScenePerfragment {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/perfrag.vert.glsl");
        let fragment_shader_code = include_str!("shaders/perfrag.frag.glsl");

        // let vertex_shader_code   = include_str!("shaders/blinnphong.vert.glsl");
        // let fragment_shader_code = include_str!("shaders/blinnphong.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
