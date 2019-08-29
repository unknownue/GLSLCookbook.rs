
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::{Mat4F, Vec3F};

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::{Surface, uniform};



#[derive(Debug)]
pub struct SceneBezCurve {

    program: glium::Program,
    solid_program: glium::Program,

    vertex_buffer: glium::VertexBuffer<Vertex>,
    
    projection: Mat4F,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct Vertex {
    VertexPosition: [f32; 2],
}


impl Scene for SceneBezCurve {

    fn new(display: &impl Facade) -> GLResult<SceneBezCurve> {

        // Shader Program ------------------------------------------------------------
        let program = SceneBezCurve::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        let solid_program = SceneBezCurve::compile_solid_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // initialize Vertices ---------------------------------------------------------
        let points: Vec<Vertex> = [[-1.0_f32, -1.0], [-0.5, 1.0], [0.5, -1.0], [1.0, 1.0]]
            .into_iter().map(|&p| Vertex { VertexPosition: p }).collect();

        glium::implement_vertex!(Vertex, VertexPosition);
        let vertex_buffer = glium::VertexBuffer::immutable(display, &points)
            .map_err(BufferCreationErrorKind::Vertex)?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        const C: f32 = 3.5;
        let projection = Mat4F::orthographic_rh_zo(vek::FrustumPlanes {
            left: -0.4 * C, right: 0.4 * C, bottom: -0.3 * C, top: 0.3 * C,
            near: 0.1, far: 100.0,
        });
        // ----------------------------------------------------------------------------


        let scene = SceneBezCurve {
            program, solid_program,
            vertex_buffer,
            projection,
        };
        Ok(scene)
    }

    fn update(&mut self, _delta_time: f32) {}

    fn render(&mut self, frame: &mut glium::Frame) -> GLResult<()> {

        frame.clear_color_srgb(0.5, 0.5, 0.5, 1.0);
        frame.clear_depth(1.0);

        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            // Set point size here !!!!!!!!!!!! ---------------------------
            point_size: Some(10.0),
            // ------------------------------------------------------------
            ..Default::default()
        };


        // Draw the curve ----------------------------------------------------------
        let camera_pos = Vec3F::new(0.0, 0.0, 1.5);
        let view = Mat4F::look_at_rh(camera_pos, Vec3F::zero(), Vec3F::unit_y());
        let model = Mat4F::identity();

        let uniforms = uniform! {
            // Segments and strips may be inverted on NVIDIA
            NumSegments: 50_i32,
            NumStrips: 1_i32,
            LineColor: [1.0_f32, 1.0, 0.5, 1.0],
            MVP: (self.projection * view * model).into_col_arrays(),
        };

        // Set the number of vertices per patch.  IMPORTANT!!
        let draw_patches = glium::index::NoIndices(glium::index::PrimitiveType::Patches { vertices_per_patch: 4 });
        frame.draw(&self.vertex_buffer, &draw_patches, &self.program, &uniforms, &draw_params)
            .map_err(GLErrorKind::DrawError)?;
        // ------------------------------------------------------------------------- 

        // Draw the control points -------------------------------------------------
        let uniforms = uniform! {
            Color: [0.5_f32, 1.0, 1.0, 1.0],
            MVP: (self.projection * view * model).into_col_arrays(),
        };

        let draw_points = glium::index::NoIndices(glium::index::PrimitiveType::Points);
        frame.draw(&self.vertex_buffer, &draw_points, &self.solid_program, &uniforms, &draw_params)
            .map_err(GLErrorKind::DrawError)?;
        // ------------------------------------------------------------------------- 
        Ok(())
    }

    fn resize(&mut self, _display: &impl Facade, _width: u32, _height: u32) -> GLResult<()> {
        Ok(())
    }

    fn is_animating(&self) -> bool { false }
    fn toggle_animation(&mut self) {}
}


impl SceneBezCurve {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code          = include_str!("shaders/bezcurve.vert.glsl");
        let tess_control_shader_code    = include_str!("shaders/bezcurve.tesc.glsl");
        let tess_evaluation_shader_code = include_str!("shaders/bezcurve.tese.glsl");
        let fragment_shader_code        = include_str!("shaders/bezcurve.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_tessellation_control_shader(tess_control_shader_code)
            .with_tessellation_evaluation_shader(tess_evaluation_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }

    fn compile_solid_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/solid.vert.glsl");
        let fragment_shader_code = include_str!("shaders/solid.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true)
            .with_point_size_enable(true);
        glium::Program::new(display, sources)
    }
}
