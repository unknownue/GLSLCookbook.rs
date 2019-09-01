
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind};
use cookbook::{Mat4F, Vec3F, Mat3F};
use cookbook::objects::TeapotPatch;
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::{Surface, uniform};


#[derive(Debug)]
pub struct SceneTessTeapotDepth {

    program: glium::Program,
    
    teapot: TeapotPatch,

    viewport: Mat4F,
    projection: Mat4F,
}

impl Scene for SceneTessTeapotDepth {

    fn new(display: &impl Facade) -> GLResult<SceneTessTeapotDepth> {

        // Shader Program ------------------------------------------------------------
        let program = SceneTessTeapotDepth::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------

        // Initialize Mesh ---------------------------------------------------------
        let teapot = TeapotPatch::new(display)?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let projection = Mat4F::identity();
        let viewport = Mat4F::identity();
        // ----------------------------------------------------------------------------


        let scene = SceneTessTeapotDepth {
            program, teapot,
            projection, viewport,
        };
        Ok(scene)
    }

    fn update(&mut self, _delta_time: f32) {}

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

        let camera_pos = Vec3F::new(0.0, 1.0, 6.25);
        let view = Mat4F::look_at_rh(camera_pos, Vec3F::zero(), Vec3F::unit_y());

        // Draw teapot 1 ------------------------------------------------------------
        let model = Mat4F::rotation_x(-90.0_f32.to_radians())
            .translated_3d(Vec3F::new(-2.0, -1.5, 0.0));
        self.render_teapot(frame, &draw_params, view, model)?;
        // ------------------------------------------------------------------------- 

        // Draw teapot 2 ------------------------------------------------------------
        let model = Mat4F::rotation_x(-90.0_f32.to_radians())
            .translated_3d(Vec3F::new(2.0, -1.5, -5.0));
        self.render_teapot(frame, &draw_params, view, model)?;
        // ------------------------------------------------------------------------- 

        // Draw teapot 3 ------------------------------------------------------------
        let model = Mat4F::rotation_x(-90.0_f32.to_radians())
            .translated_3d(Vec3F::new(7.0, -1.5, -10.0));
        self.render_teapot(frame, &draw_params, view, model)?;
        // ------------------------------------------------------------------------- 

        // Draw teapot 4 ------------------------------------------------------------
        let model = Mat4F::rotation_x(-90.0_f32.to_radians())
            .translated_3d(Vec3F::new(17.0, -1.5, -20.0));
        self.render_teapot(frame, &draw_params, view, model)
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

    fn is_animating(&self) -> bool { false }
    fn toggle_animation(&mut self) {}
}


impl SceneTessTeapotDepth {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code          = include_str!("shaders/tessteapot.vert.glsl");
        let geometry_shader_code        = include_str!("shaders/tessteapot.geom.glsl");
        let tess_control_shader_code    = include_str!("shaders/tessteapotdepth.tesc.glsl");
        let tess_evaluation_shader_code = include_str!("shaders/tessteapotdepth.tese.glsl");
        let fragment_shader_code        = include_str!("shaders/tessteapot.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_geometry_shader(geometry_shader_code)
            .with_tessellation_control_shader(tess_control_shader_code)
            .with_tessellation_evaluation_shader(tess_evaluation_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }

    fn render_teapot(&mut self, frame: &mut glium::Frame, draw_params: &glium::DrawParameters, view: Mat4F, model: Mat4F) -> GLResult<()> {

        let mv = view * model;

        let uniforms = uniform! {
            MinTessLevel  : 2_i32,
            MaxTessLevel  : 15_i32,
            MaxDepth      : 20.0_f32,
            MinDepth      : 2.0_f32,
            LineWidth     : 0.8_f32,
            LineColor     : [0.05_f32, 0.0, 0.05, 1.0],
            LightPosition : [0.0_f32, 0.0, 0.0, 1.0],
            LightIntensity: [1.0_f32, 1.0, 1.0],
            Kd            : [0.9_f32, 0.9, 1.0],
            ViewportMatrix: self.viewport.into_col_arrays(),
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.teapot.render(frame, &self.program, &draw_params, &uniforms)
    }
}
