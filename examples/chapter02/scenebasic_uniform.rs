
use cookbook::scene::{Scene, SceneData};
use cookbook::error::{GLResult, GLError, GLErrorKind};
use cookbook::utils;
use cookbook::Mat4F;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniform;
use glium::Surface;


#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Vertex {
    VertexPosition: [f32; 3],
    VertexColor   : [f32; 3],
}

const TRIANGLE: [Vertex; 3] = [
    Vertex { VertexPosition: [-0.8, -0.8, 0.0], VertexColor: [1.0, 0.0, 0.0] },
    Vertex { VertexPosition: [ 0.8, -0.8, 0.0], VertexColor: [0.0, 1.0, 0.0] },
    Vertex { VertexPosition: [ 0.0,  0.8, 0.0], VertexColor: [0.0, 0.0, 1.0] },
];


#[derive(Debug)]
pub struct SceneBasicUniform {
    scene_data: SceneData,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    program: glium::Program,

    angle: f32,
}

impl Scene for SceneBasicUniform {

    fn new(display: &impl Facade) -> GLResult<SceneBasicUniform> {

        let program = SceneBasicUniform::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;

        glium::implement_vertex!(Vertex, VertexPosition, VertexColor);
        let vertex_buffer = glium::VertexBuffer::new(display, &TRIANGLE)
            .map_err(GLErrorKind::CreateBuffer)?;

        utils::print_active_uniforms(&program);

        // set true to enable animation.
        let scene_data = SceneData::new(true);

        let scene = SceneBasicUniform {
            scene_data, vertex_buffer, program,
            angle: 0.0,
        };
        Ok(scene)
    }

    fn update(&mut self, delta_time: f32) {

        if self.is_animating() {
            self.angle = (self.angle + 50.0 * delta_time) % 360.0;
        }
    }

    fn render(&self, display: &glium::Display) -> GLResult<()> {

        let no_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let draw_params = glium::draw_parameters::DrawParameters {
            viewport: Some(self.scene_data.viewport()),
            ..Default::default()
        };

        let uniforms = uniform! {
            RotationMatrix: Mat4F::identity().rotated_z(self.angle.to_radians()).into_col_arrays(),
        };

        let mut target = display.draw();
        target.clear_color(0.5, 0.5, 0.5, 1.0);
        target.draw(&self.vertex_buffer, &no_indices, &self.program, &uniforms, &draw_params)
            .map_err(GLErrorKind::DrawError)?;

        target.finish()
            .map_err(|_| GLError::device("Something wrong when swapping framebuffers."))
    }

    #[inline(always)]
    fn scene_data(&self) -> &SceneData { &self.scene_data }
    #[inline(always)]
    fn scene_data_mut(&mut self) -> &mut SceneData { &mut self.scene_data }
}


impl SceneBasicUniform {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/basic_uniform.vert.glsl");
        let fragment_shader_code = include_str!("shaders/basic_uniform.frag.glsl");

        glium::Program::from_source(display, vertex_shader_code, fragment_shader_code, None)
    }
}
