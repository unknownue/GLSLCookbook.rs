
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::{Mat4F, Vec3F};

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::{Surface, uniform};



#[derive(Debug)]
pub struct SceneQuadTess {

    program: glium::Program,

    vertex_buffer: glium::VertexBuffer<Vertex>,

    viewport: Mat4F,
    projection: Mat4F,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct Vertex {
    VertexPosition: [f32; 2],
}


impl Scene for SceneQuadTess {

    fn new(display: &impl Facade) -> GLResult<SceneQuadTess> {

        // Shader Program ------------------------------------------------------------
        let program = SceneQuadTess::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // initialize patches ---------------------------------------------------------
        let points: Vec<Vertex> = [[-1.0_f32, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]]
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

        let viewport = Mat4F::identity();
        // ----------------------------------------------------------------------------


        let scene = SceneQuadTess {
            program,
            vertex_buffer,
            projection, viewport,
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
            ..Default::default()
        };


        // Draw quad -------------------------------------------------------------
        let camera_pos = Vec3F::new(0.0, 0.0, 1.5);
        let view = Mat4F::look_at_rh(camera_pos, Vec3F::zero(), Vec3F::unit_y());
        let model = Mat4F::identity();

        let uniforms = uniform! {
            Inner: 4_i32,
            Outer: 4_i32,
            ViewportMatrix: self.viewport.into_col_arrays(),
            LineWidth: 1.5_f32,
            LineColor: [0.05_f32, 0.0, 0.05, 1.0],
            QuadColor: [1.0_f32, 1.0, 1.0, 1.0],
            MVP: (self.projection * view * model).into_col_arrays(),
        };

        // Set the number of vertices per patch.  IMPORTANT!!
        let draw_patches = glium::index::NoIndices(glium::index::PrimitiveType::Patches { vertices_per_patch: 4 });
        frame.draw(&self.vertex_buffer, &draw_patches, &self.program, &uniforms, &draw_params)
            .map_err(GLErrorKind::DrawError)?;
        // ------------------------------------------------------------------------- 

        Ok(())
    }

    fn resize(&mut self, _display: &impl Facade, width: u32, height: u32) -> GLResult<()> {

        self.viewport = Mat4F::new(
            width as f32 / 2.0, 0.0, 0.0, 0.0,
            0.0, height as f32 / 2.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            width as f32 / 2.0, height as f32 / 2.0, 0.0, 1.0,
        );
        Ok(())
    }

    fn is_animating(&self) -> bool { false }
    fn toggle_animation(&mut self) {}
}


impl SceneQuadTess {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code          = include_str!("shaders/quadtess.vert.glsl");
        let geometry_shader_code        = include_str!("shaders/quadtess.geom.glsl");
        let tess_control_shader_code    = include_str!("shaders/quadtess.tesc.glsl");
        let tess_evaluation_shader_code = include_str!("shaders/quadtess.tese.glsl");
        let fragment_shader_code        = include_str!("shaders/quadtess.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_geometry_shader(geometry_shader_code)
            .with_tessellation_control_shader(tess_control_shader_code)
            .with_tessellation_evaluation_shader(tess_evaluation_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
