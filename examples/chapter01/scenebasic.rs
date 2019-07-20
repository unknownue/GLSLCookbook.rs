
use cookbook::scene::{Scene, SceneData};
use cookbook::error::{GLResult, GLError, GLErrorKind};

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
    scene_data: SceneData,
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
        let vertex_buffer = glium::VertexBuffer::new(display, &TRIANGLE)
            .map_err(GLErrorKind::CreateBuffer)?;

        let scene_data: SceneData = Default::default();

        // All the initialization work has done.
        let scene = SceneBasic { scene_data, vertex_buffer, program };
        Ok(scene)
    }

    /// This is called prior to every frame. Use this to update your animation.
    fn update(&mut self, _delta_time: f32) {
        // nothing to do, just keep it empty
    }

    /// Draw your scene.
    fn render(&self, display: &glium::Display) -> GLResult<()> {

        // For simplicity, we do not use index buffer.
        let no_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let draw_params = glium::draw_parameters::DrawParameters {
            viewport: Some(self.scene_data.viewport()),
            ..Default::default()
        };

        let mut target = display.draw();
        target.clear_color(0.5, 0.5, 0.5, 1.0);
        target.draw(&self.vertex_buffer, &no_indices, &self.program, &glium::uniforms::EmptyUniforms, &draw_params)
            .map_err(GLErrorKind::DrawError)?;

        // Glium swap the buffer in swapchain for you.
        target.finish()
            .map_err(|_| GLError::device("Something wrong when swapping framebuffers."))
    }

    #[inline(always)]
    fn scene_data(&self) -> &SceneData { &self.scene_data }
    #[inline(always)]
    fn scene_data_mut(&mut self) -> &mut SceneData { &mut self.scene_data }
}


impl SceneBasic {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        println!("Compiling Shader Program");

    	// Load vertex shader contents of file.
        let vertex_shader_code = include_str!("shaders/basic.vert.glsl");

    	// Load fragment shader contents of file.
        let fragment_shader_code = include_str!("shaders/basic.frag.glsl");

        // use the wrapper function provided by glium to create program directly.
        let program = glium::Program::from_source(display, vertex_shader_code, fragment_shader_code, None);

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
