
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
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
    vertex_buffer: glium::VertexBuffer<Vertex>,
    program: glium::Program,

    angle: f32,
    is_animate: bool,
}

impl Scene for SceneBasicUniform {

    fn new(display: &impl Facade) -> GLResult<SceneBasicUniform> {

        let program = SceneBasicUniform::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;

        glium::implement_vertex!(Vertex, VertexPosition, VertexColor);
        let vertex_buffer = glium::VertexBuffer::immutable(display, &TRIANGLE)
            .map_err(BufferCreationErrorKind::Vertex)?;

        utils::print_active_uniforms(&program);

        let scene = SceneBasicUniform {
            vertex_buffer, program,
            angle: 0.0,
            is_animate: true, // Enalbe animation
        };
        Ok(scene)
    }

    fn update(&mut self, delta_time: f32) {

        if self.is_animating() {
            self.angle = (self.angle + 50.0 * delta_time) % 360.0;
        }
    }

    fn render(&mut self, frame: &mut glium::Frame) -> GLResult<()> {

        let no_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let uniforms = uniform! {
            RotationMatrix: Mat4F::rotation_z(self.angle.to_radians()).into_col_arrays(),
        };

        frame.clear_color_srgb(0.5, 0.5, 0.5, 1.0);
        frame.draw(&self.vertex_buffer, &no_indices, &self.program, &uniforms, &Default::default())
            .map_err(GLErrorKind::DrawError)?;

        Ok(())
    }

    fn resize(&mut self, _width: u32, _height: u32) {}

    #[inline]
    fn is_animating(&self) -> bool { self.is_animate }
    #[inline]
    fn toggle_animation(&mut self) { self.is_animate = !self.is_animate; }
}


impl SceneBasicUniform {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/basic_uniform.vert.glsl");
        let fragment_shader_code = include_str!("shaders/basic_uniform.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
