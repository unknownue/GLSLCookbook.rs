//! This example is almost the same with chapter01.

use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::utils;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
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
pub struct SceneBasicAttrib {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    program: glium::Program,
}


impl Scene for SceneBasicAttrib {

    fn new(display: &impl Facade) -> GLResult<SceneBasicAttrib> {

        let program = SceneBasicAttrib::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;

        glium::implement_vertex!(Vertex, VertexPosition, VertexColor);
        let vertex_buffer = glium::VertexBuffer::immutable(display, &TRIANGLE)
            .map_err(BufferCreationErrorKind::Vertex)?;

        utils::print_active_attribs(&program);

        let scene = SceneBasicAttrib { vertex_buffer, program };
        Ok(scene)
    }

    fn update(&mut self, _delta_time: f32) {
        // nothing to do, just keep it empty
    }

    fn render(&mut self, frame: &mut glium::Frame) -> GLResult<()> {

        let no_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        frame.clear_color_srgb(0.5, 0.5, 0.5, 1.0);
        frame.draw(&self.vertex_buffer, &no_indices, &self.program, &glium::uniforms::EmptyUniforms, &Default::default())
            .map_err(GLErrorKind::DrawError)?;

        Ok(())
    }

    fn resize(&mut self, _display: &impl Facade, _width: u32, _height: u32) {}

    fn is_animating(&self) -> bool { false }
    fn toggle_animation(&mut self) {}
}


impl SceneBasicAttrib {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

    	// Load vertex shader contents of file.
        let vertex_shader_code = include_str!("shaders/basic.vert.glsl");

    	// Load fragment shader contents of file.
        let fragment_shader_code = include_str!("shaders/basic.frag.glsl");

        // use the wrapper function provided by glium to create program directly.
        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
