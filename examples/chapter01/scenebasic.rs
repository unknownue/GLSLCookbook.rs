
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};

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
pub struct SceneBasic {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    program: glium::Program,
}


impl Scene for SceneBasic {

    /// Load textures, initialize shaders, etc.
    fn new(display: &impl Facade) -> GLResult<SceneBasic> {

        // **************************************************************************************
        // Choose one of the following options for the shader program.
        //  1)  Compile the shader program normally
        //  2)  Load a binary (pre-compiled) shader program.  (file: "shader/program.bin")
        //  3)  Load a SPIR-V shader program. (files: "shader/vert.spv" and "shader/frag.spv")
        //
        // Optionally, you may attempt to write out the shader program binary using the function writeShaderBinary().
        // **************************************************************************************

        // (1) Use this to load and compile the shader program.
        let program = SceneBasic::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;

        // (2) Use this to load a binary shader.  Use the format provided when the binary was written.
        // let shaderFormat: i32 = 36385;
        // let program = load_shader_binary(shaderFormat);

        // (3) Load a SPIR-V shader
        // let program = load_spriv_shader();

        // Optional: use this to write the shader binary out to a file.
        // let program = write_shader_binary();


        /////////////////// Create the VertexBuffer ////////////////////
        glium::implement_vertex!(Vertex, VertexPosition, VertexColor);
        let vertex_buffer = glium::VertexBuffer::immutable(display, &TRIANGLE)
            .map_err(BufferCreationErrorKind::Vertex)?;

        // All the initialization work has done.
        let scene = SceneBasic { vertex_buffer, program };
        Ok(scene)
    }

    /// This is called prior to every frame. Use this to update your animation.
    fn update(&mut self, _delta_time: f32) {
        // nothing to do, just keep it empty
    }

    /// Draw your scene.
    fn render(&mut self, frame: &mut glium::Frame) -> GLResult<()> {

        // For simplicity, we do not use index buffer.
        let no_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        frame.clear_color_srgb(0.5, 0.5, 0.5, 1.0);
        frame.draw(&self.vertex_buffer, &no_indices, &self.program, &glium::uniforms::EmptyUniforms, &Default::default())
            .map_err(GLErrorKind::DrawError)?;

        Ok(())
    }

    fn resize(&mut self, _display: &impl Facade, _width: u32, _height: u32) -> GLResult<()> {
        Ok(())
    }

    fn is_animating(&self) -> bool { false }
    fn toggle_animation(&mut self) {}
}


impl SceneBasic {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        println!("Compiling Shader Program");

    	// Load vertex shader contents of file.
        let vertex_shader_code = include_str!("shaders/basic.vert.glsl");

    	// Load fragment shader contents of file.
        let fragment_shader_code = include_str!("shaders/basic.frag.glsl");

        // use the wrapper function provided by glium to create program directly.
        // glium does not provide a method to set srgb output directly, so a custom GLSourceCode is introduced here.
        // GLSourceCode is very similar to glium::program::SourceCode, but provides some customization to its members.
        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        let program = glium::Program::new(display, sources);

        println!("Finish Shader Compiling");

        program
    }

    fn _load_shader_binary(_format: glium::program::Binary) -> Result<Program, ProgramCreationError> {
        println!("Loading shader binary: shader/program.bin (format = %d)", );
        unimplemented!()
    }

    fn _load_spriv_shader() -> Result<Program, ProgramCreationError>  {
        unimplemented!()
    }

    fn _write_shader_binary() -> Result<Program, ProgramCreationError>  {
        unimplemented!()
    }
}
