
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::utils;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::{uniform, implement_uniform_block};
use glium::uniforms::UniformBuffer;
use glium::Surface;


#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Vertex {
    VertexPosition: [f32; 3], // Vertex Position
    VertexTexCoord: [f32; 2], // Vertex texture coordinates
}

const TRIANGLE: [Vertex; 6] = [
    Vertex { VertexPosition: [-0.8, -0.8, 0.0], VertexTexCoord: [0.0, 0.0] },
    Vertex { VertexPosition: [ 0.8, -0.8, 0.0], VertexTexCoord: [1.0, 0.0] },
    Vertex { VertexPosition: [ 0.8,  0.8, 0.0], VertexTexCoord: [1.0, 1.0] },
    Vertex { VertexPosition: [-0.8, -0.8, 0.0], VertexTexCoord: [0.0, 0.0] },
    Vertex { VertexPosition: [ 0.8,  0.8, 0.0], VertexTexCoord: [1.0, 1.0] },
    Vertex { VertexPosition: [-0.8,  0.8, 0.0], VertexTexCoord: [0.0, 1.0] },
];

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct BlobSettings {
    InnerColor: [f32; 4],
    OuterColor: [f32; 4],
    RadiusInner: f32,
    RadiusOuter: f32,
}

#[derive(Debug)]
pub struct SceneBasicUniformBlock {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    program: glium::Program,

    uniform_block: UniformBuffer<BlobSettings>,
}

impl Scene for SceneBasicUniformBlock {

    fn new(display: &impl Facade) -> GLResult<SceneBasicUniformBlock> {

        let program = SceneBasicUniformBlock::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;

        // -------------------------- Vertex Buffer ---------------------------------------
        glium::implement_vertex!(Vertex, VertexPosition, VertexTexCoord);
        let vertex_buffer = glium::VertexBuffer::immutable(display, &TRIANGLE)
            .map_err(BufferCreationErrorKind::Vertex)?;
        // --------------------------------------------------------------------------------


        // -------------------------- Uniform Buffer Block --------------------------------
        // let glium help to create uniform blocks.
        glium::implement_uniform_block!(BlobSettings, InnerColor, OuterColor, RadiusInner, RadiusOuter);

        let uniform_block = UniformBuffer::immutable(display, BlobSettings {
            InnerColor: [1.0, 1.0, 0.75, 1.0],
            OuterColor: [0.0, 0.0, 0.0, 0.0],
            RadiusInner: 0.25,
            RadiusOuter: 0.45,
        }).map_err(BufferCreationErrorKind::UniformBlock)?;

        utils::print_active_uniform_blocks(&program);
        // println!("{:?}", scene.uniform_block.read());
        // --------------------------------------------------------------------------------


        let scene = SceneBasicUniformBlock { vertex_buffer, program, uniform_block };
        Ok(scene)
    }

    fn update(&mut self, _delta_time: f32) {
        // nothing to do
    }

    fn render(&self, frame: &mut glium::Frame) -> GLResult<()> {

        let no_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let uniforms = uniform! {
            BlobSettings: &self.uniform_block,
        };

        frame.clear_color_srgb(0.5, 0.5, 0.5, 1.0);
        frame.draw(&self.vertex_buffer, &no_indices, &self.program, &uniforms, &Default::default())
            .map_err(GLErrorKind::DrawError)?;

        Ok(())
    }

    fn resize(&mut self, _width: u32, _height: u32) {}

    fn is_animating(&self) -> bool { false }
    fn toggle_animation(&mut self) {}
}


impl SceneBasicUniformBlock {

    // TODO: The following code has not been test yet!
    #[cfg(not(target_os = "macos"))]
    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/basic_uniformblock.vert.glsl");
        let fragment_shader_code = include_str!("shaders/basic_uniformblock.frag.glsl");

        glium::Program::from_source(display, vertex_shader_code, fragment_shader_code, None)
    }

    #[cfg(target_os = "macos")]
    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/basic_uniformblock_v41.vert.glsl");
        let fragment_shader_code = include_str!("shaders/basic_uniformblock_v41.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
