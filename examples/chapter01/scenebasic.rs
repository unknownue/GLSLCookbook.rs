
use cookbook::scene::{Scene, SceneData};
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
pub struct SceneBasic {
    scene_data: SceneData,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    program: glium::Program,
}

impl SceneBasic {

    /// Load textures, initialize shaders, etc.
    pub fn new<F: glium::backend::Facade>(display: &F, scene_data: SceneData) -> SceneBasic {

        // **************************************************************************************
        // Choose one of the following options for the shader program.
        //  1)  Compile the shader program normally
        //  2)  Load a binary (pre-compiled) shader program.  (file: "shader/program.bin")
        //  3)  Load a SPIR-V shader program. (files: "shader/vert.spv" and "shader/frag.spv")
        //
        // Optionally, you may attempt to write out the shader program binary using the function writeShaderBinary().
        // **************************************************************************************

        // (1) Use this to load and compile the shader program.
        let program = SceneBasic::compile_shader_program(display).unwrap();

        // (2) Use this to load a binary shader.  Use the format provided when the binary was written.
        // let shaderFormat: i32 = 36385;
        // let program = load_shader_binary(shaderFormat);

        // (3) Load a SPIR-V shader
        // let program = load_spriv_shader();

        // Optional: use this to write the shader binary out to a file.
        // let program = write_shader_binary();


        /////////////////// Create the VertexBuffer ////////////////////
        glium::implement_vertex!(Vertex, position, color);
        let vertex_buffer = glium::VertexBuffer::new(display, &TRIANGLE).unwrap();

        // All the initialization work has done.
        SceneBasic { scene_data, vertex_buffer, program }
    }

    fn compile_shader_program<F: glium::backend::Facade>(display: &F) -> Result<glium::Program, glium::program::ProgramCreationError> {
        println!("Compiling Shader Program");

    	// Load vertex shader contents of file.
        let vertex_shader_code = include_str!("shaders/basic.vert.glsl");

    	// Load fragment shader contents of file.
        let fragment_shader_code = include_str!("shaders/basic.frag.glsl");

        // use the wrapper function provided by glium to create program directly.
        glium::Program::from_source(display, vertex_shader_code, fragment_shader_code, None)
    }

    fn _load_shader_binary(_format: i32) -> glium::Program {
        println!("Loading shader binary: shader/program.bin");

        unimplemented!()
    }

    fn _load_spriv_shader() -> glium::Program {
        unimplemented!()
    }

    fn _write_shader_binary() -> glium::Program {
        unimplemented!()
    }
}

impl Scene for SceneBasic {

    /// This is called prior to every frame. Use this to update your animation.
    fn update(&mut self, _t: f32) {
        // nothing to do, just keep it empty
    }

    /// Draw your scene.
    fn render(&self, display: &glium::Display) {

        // For simplicity, we do not use index buffer.
        let no_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&self.vertex_buffer, &no_indices, &self.program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    }

    /// Called when screen is resized.
    fn resize(&mut self, width: usize, height: usize) {

        self.scene_data.width = width;
        self.scene_data.height = height;

        // Find equivalent way to set viewport(glViewport).
        unimplemented!()
    }

    #[inline(always)]
    fn scene_data(&self) -> &SceneData { &self.scene_data }
    #[inline(always)]
    fn scene_data_mut(&mut self) -> &mut SceneData { &mut self.scene_data }
}
