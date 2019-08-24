
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind};
use cookbook::objects::Quad;
use cookbook::{Mat4F, Vec3F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::{Surface, uniform};


#[derive(Debug)]
pub struct SceneMsaa {

    program: glium::Program,

    quad: Quad,

    projection: Mat4F,
    angle: f32,
    is_animate: bool,
}



impl Scene for SceneMsaa {

    fn new(display: &impl Facade) -> GLResult<SceneMsaa> {

        // Shader Program ------------------------------------------------------------
        let program = SceneMsaa::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------

        // Initialize Mesh ------------------------------------------------------------
        let quad = Quad::new(display)?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let projection = Mat4F::identity();
        let angle = std::f32::consts::PI / 2.0;
        let is_animate = true;
        // ----------------------------------------------------------------------------

        let scene = SceneMsaa {
            program, quad,
            projection, angle, is_animate,
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

        frame.clear_color(0.5, 0.5, 0.5, 1.0);
        frame.clear_depth(1.0);

        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            multisampling: self.is_animating(),
            ..Default::default()
        };

        let model = Mat4F::rotation_z(30.0_f32.to_radians());
        let view = Mat4F::look_at_rh(Vec3F::new(3.0 * self.angle.cos(), 0.0, 3.0 * self.angle.sin()), Vec3F::zero(), Vec3F::unit_y());
        let mv: Mat4F = view * model;

        let uniforms = uniform! {
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.quad.render(frame, &self.program, &draw_params, &uniforms)
    }

    fn resize(&mut self, _display: &impl Facade, _width: u32, _height: u32) {

        const C: f32 = 5.0;
        self.projection = Mat4F::orthographic_rh_zo(vek::FrustumPlanes {
            left: -0.4 * C, right: 0.4 * C, bottom: -0.3 * C, top: 0.3 * C,
            near: 0.1, far: 100.0,
        });;
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }
    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate;
    }
}


impl SceneMsaa {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/centroid.vert.glsl");
        let fragment_shader_code = include_str!("shaders/centroid.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
