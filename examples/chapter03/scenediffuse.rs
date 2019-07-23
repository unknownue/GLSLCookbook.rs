
use cookbook::scene::{Scene, SceneData};
use cookbook::error::{GLResult, GLErrorKind};
use cookbook::objects::torus::Torus;
use cookbook::{Mat4F, Mat3F, Vec3F, Vec4F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::{Surface, uniform};


#[derive(Debug)]
pub struct SceneDiffuse {
    scene_data: SceneData,
    program: glium::Program,
    torus: Torus,
}

impl Scene for SceneDiffuse {

    fn new(display: &impl Facade) -> GLResult<SceneDiffuse> {

        let program = SceneDiffuse::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // cookbook::utils::print_active_uniforms(&program);

        let torus = Torus::new(display, 0.7, 0.3, 30, 30)?;

        let model = Mat4F::identity()
            .rotated_x(-35.0_f32.to_radians())
            .rotated_y( 35.0_f32.to_radians());
        let view = Mat4F::look_at_rh(Vec3F::new(0.0, 0.0, 2.0), Vec3F::zero(), Vec3F::unit_y());
        let projection = Mat4F::identity();

        let scene_data = SceneData::new_detail(false, projection, view, model);

        let scene = SceneDiffuse { scene_data, program, torus };
        Ok(scene)
    }

    fn update(&mut self, _delta_time: f32) {
        // nothing to do, just keep it empty
    }

    fn render(&self, frame: &mut glium::Frame) -> GLResult<()> {

        let draw_params = glium::draw_parameters::DrawParameters {
            viewport: Some(self.scene_data.viewport()),
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let mv: Mat4F = self.scene_data.view * self.scene_data.model;
        let uniforms = uniform! {
            LightPosition: (self.scene_data.view * Vec4F::new(5.0, 5.0, 2.0, 1.0)).into_array(),
            Kd: [0.9_f32, 0.5, 0.3],
            Ld: [1.0_f32, 1.0, 1.0],
            ModelViewMatrix: mv.clone().into_col_arrays(),

            // If your model-view matrix does not include any nonuniform scaling,
            // then one can use the upper-left 3 x 3 of the model-view matrix in place of the normal matrix to transform your normal vectors.
            // However, if your model-view matrix does include (uniform) scaling,
            // you'll still need to (re)normalize your normal vectors after transforming them.
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.scene_data.projection * mv).into_col_arrays(),
        };

        frame.clear_color(0.5, 0.5, 0.5, 1.0);
        frame.clear_depth(1.0);

        self.torus.render(frame, &self.program, &draw_params, &uniforms)
    }

    fn resize(&mut self, width: u32, height: u32) {

        self.scene_data.set_dimension(width, height);
        self.scene_data.projection = Mat4F::perspective_rh_zo(70.0_f32.to_radians(), self.scene_data.sceen_aspect_ratio(), 0.3, 100.0);
    }

    #[inline(always)]
    fn scene_data(&self) -> &SceneData { &self.scene_data }
    #[inline(always)]
    fn scene_data_mut(&mut self) -> &mut SceneData { &mut self.scene_data }
}


impl SceneDiffuse {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/diffuse.vert.glsl");
        let fragment_shader_code = include_str!("shaders/diffuse.frag.glsl");

        glium::Program::from_source(display, vertex_shader_code, fragment_shader_code, None)
    }
}
