//! This example is almost the same with chapter01.

use cookbook::scene::{Scene, SceneData};
use cookbook::error::{GLResult, GLError, GLErrorKind};
use cookbook::utils;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::Surface;


#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Vertex {
    position : [f32; 3],
    color    : [f32; 3],
}

const TRIANGLE: [Vertex; 3] = [
    Vertex { position: [-0.8, -0.8, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [ 0.8, -0.8, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [ 0.0,  0.8, 0.0], color: [0.0, 0.0, 1.0] },
];


#[derive(Debug)]
pub struct SceneBasicAttrib {
    scene_data: SceneData,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    program: glium::Program,
}


impl Scene for SceneBasicAttrib {

    fn new(display: &impl Facade) -> GLResult<SceneBasicAttrib> {

        let program = SceneBasicAttrib::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;

        glium::implement_vertex!(Vertex, position, color);
        let vertex_buffer = glium::VertexBuffer::new(display, &TRIANGLE)
            .map_err(GLErrorKind::CreateBuffer)?;

        utils::print_active_attribs(&program);

        let scene_data: SceneData = Default::default();

        let scene = SceneBasicAttrib { scene_data, vertex_buffer, program };
        Ok(scene)
    }

    fn update(&mut self, _delta_time: f32) {
        // nothing to do, just keep it empty
    }

    fn render(&self, display: &glium::Display) -> GLResult<()> {

        let no_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let draw_params = glium::draw_parameters::DrawParameters {
            viewport: Some(self.scene_data.viewport()),
            ..Default::default()
        };

        let mut target = display.draw();
        target.clear_color(0.5, 0.5, 0.5, 1.0);
        target.draw(&self.vertex_buffer, &no_indices, &self.program, &glium::uniforms::EmptyUniforms, &draw_params)
            .map_err(GLErrorKind::DrawError)?;

        target.finish()
            .map_err(|_| GLError::device("Something wrong when swapping framebuffers."))
    }

    #[inline(always)]
    fn scene_data(&self) -> &SceneData { &self.scene_data }
    #[inline(always)]
    fn scene_data_mut(&mut self) -> &mut SceneData { &mut self.scene_data }
}


impl SceneBasicAttrib {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

    	// Load vertex shader contents of file.
        let vertex_shader_code = include_str!("shaders/basic.vert.glsl");

    	// Load fragment shader contents of file.
        let fragment_shader_code = include_str!("shaders/basic.frag.glsl");

        // use the wrapper function provided by glium to create program directly.
        glium::Program::from_source(display, vertex_shader_code, fragment_shader_code, None)
    }
}
