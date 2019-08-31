
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind};
use cookbook::{Mat4F, Vec3F, Mat3F};
use cookbook::objects::TeapotPatch;
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::{Surface, uniform};


#[derive(Debug)]
pub struct SceneTessTeapot {

    program: glium::Program,
    
    teapot: TeapotPatch,

    viewport: Mat4F,
    projection: Mat4F,

    angle: f32,
    is_animate: bool,
}


impl Scene for SceneTessTeapot {

    fn new(display: &impl Facade) -> GLResult<SceneTessTeapot> {

        // Shader Program ------------------------------------------------------------
        let program = SceneTessTeapot::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ---------------------------------------------------------
        let teapot = TeapotPatch::new(display)?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let projection = Mat4F::identity();
        let viewport = Mat4F::identity();
        let is_animate = true;
        let angle = std::f32::consts::PI / 3.0;
        // ----------------------------------------------------------------------------


        let scene = SceneTessTeapot {
            program, teapot,
            projection, viewport, is_animate, angle,
        };
        Ok(scene)
    }

    fn update(&mut self, delta_time: f32) {
        const TWO_PI: f32 = std::f32::consts::PI * 2.0;
        const ROTATE_SPEED: f32 = std::f32::consts::PI / 8.0;

        if self.is_animating() {
            self.angle = (self.angle + delta_time * ROTATE_SPEED) % TWO_PI;
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


        // Draw teapot ------------------------------------------------------------
        let camera_pos = Vec3F::new(4.25 * self.angle.cos(), 3.0, 4.25 * self.angle.sin());
        let view = Mat4F::look_at_rh(camera_pos, Vec3F::zero(), Vec3F::unit_y());
        let model = Mat4F::rotation_x(-90.0_f32.to_radians())
            .translated_3d(Vec3F::new(0.0, -1.5, 0.0));
        let mv = view * model;

        let uniforms = uniform! {
            TessLevel: 4_i32,
            ViewportMatrix: self.viewport.into_col_arrays(),
            LineWidth: 0.8_f32,
            LineColor: [0.05_f32, 0.0, 0.05, 1.0],
            LightPosition: [0.0_f32, 0.0, 0.0, 1.0],
            LightIntensity: [1.0_f32, 1.0, 1.0],
            Kd: [0.9_f32, 0.9, 1.0],
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.teapot.render(frame, &self.program, &draw_params, &uniforms)
        // ------------------------------------------------------------------------- 
    }

    fn resize(&mut self, _display: &impl Facade, width: u32, height: u32) -> GLResult<()> {

        self.projection = Mat4F::perspective_rh_zo(60.0_f32.to_radians(), width as f32 / height as f32, 0.3, 100.0);

        self.viewport = Mat4F::new(
            width as f32 / 2.0, 0.0, 0.0, 0.0,
            0.0, height as f32 / 2.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            width as f32 / 2.0, height as f32 / 2.0, 0.0, 1.0,
        );
        Ok(())
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }

    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate
    }
}


impl SceneTessTeapot {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code          = include_str!("shaders/tessteapot.vert.glsl");
        let geometry_shader_code        = include_str!("shaders/tessteapot.geom.glsl");
        let tess_control_shader_code    = include_str!("shaders/tessteapot.tesc.glsl");
        let tess_evaluation_shader_code = include_str!("shaders/tessteapot.tese.glsl");
        let fragment_shader_code        = include_str!("shaders/tessteapot.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_geometry_shader(geometry_shader_code)
            .with_tessellation_control_shader(tess_control_shader_code)
            .with_tessellation_evaluation_shader(tess_evaluation_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
