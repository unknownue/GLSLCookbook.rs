
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{ObjMesh, ObjMeshConfiguration};
use cookbook::{Mat4F, Mat3F, Vec3F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::{Surface, uniform, implement_uniform_block};



#[derive(Debug)]
pub struct SceneShadeWire {

    program: glium::Program,

    ogre: ObjMesh,
    material_buffer : UniformBuffer<MaterialInfo>,
    line_buffer     : UniformBuffer<LineInfo>,
    light_buffer    : UniformBuffer<LightInfo>,

    projection: Mat4F,
    screen_width : f32,
    screen_height: f32,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct MaterialInfo {
    Ka: [f32; 3], _padding1: f32,
    Kd: [f32; 3], _padding2: f32,
    Ks: [f32; 3],
    Shininess: f32,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct LineInfo {
    LineColor: [f32; 4],
    LineWidth: f32, _padding: [f32; 3],
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct LightInfo {
    LightPosition: [f32; 4],
    Intensity: [f32; 3], _padding: f32,
}

impl Scene for SceneShadeWire {

    fn new(display: &impl Facade) -> GLResult<SceneShadeWire> {

        let (screen_width, screen_height) = display.get_context().get_framebuffer_dimensions();
        let (screen_width, screen_height) = (screen_width as f32, screen_height as f32);

        // Shader Program ------------------------------------------------------------
        let program = SceneShadeWire::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------

        // Initialize Meshes ----------------------------------------------------------
        let ogre = ObjMesh::load(display, "media/bs_ears.obj", ObjMeshConfiguration {
            is_with_adjacency: false,
            is_gen_tangents: false,
            is_center: false,
            is_print_load_message: true,
        })?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        const C: f32 = 1.5;
        let projection = Mat4F::orthographic_rh_zo(vek::FrustumPlanes {
            left: -0.4 * C, right: 0.4 * C, bottom: -0.3 * C, top: 0.3 * C,
            near: 0.1, far: 100.0,
        });
        // ----------------------------------------------------------------------------

        // Initialize Uniforms --------------------------------------------------------
        glium::implement_uniform_block!(LightInfo, LightPosition, Intensity);
        let light_buffer = UniformBuffer::immutable(display, LightInfo {
            LightPosition: [0.0, 0.0, 0.0, 1.0],
            Intensity: [1.0, 1.0, 1.0], ..Default::default()
        }).map_err(BufferCreationErrorKind::UniformBlock)?;

        glium::implement_uniform_block!(LineInfo, LineColor, LineWidth);
        let line_buffer = UniformBuffer::immutable(display, LineInfo {
            LineColor: [0.05, 0.0, 0.05, 1.0],
            LineWidth: 0.75, ..Default::default()
        }).map_err(BufferCreationErrorKind::UniformBlock)?;

        glium::implement_uniform_block!(MaterialInfo, Ka, Kd, Ks, Shininess);
        let material_buffer = UniformBuffer::immutable(display, MaterialInfo {
            Ka: [0.2, 0.2, 0.2],
            Kd: [0.7, 0.7, 0.7],
            Ks: [0.8, 0.8, 0.8],
            Shininess: 100.0, ..Default::default()
        }).map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------


        let scene = SceneShadeWire {
            program,
            ogre,
            material_buffer, line_buffer, light_buffer,
            projection, screen_width, screen_height,
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

        let camera_pos = Vec3F::new(0.0, 0.0, 3.0);
        let view = Mat4F::look_at_rh(camera_pos, Vec3F::zero(), Vec3F::unit_y());

        let model = Mat4F::identity();
        let mv: Mat4F = view * model;

        let viewport = Mat4F::new(
            self.screen_width / 2.0, 0.0, 0.0, 0.0,
            0.0, self.screen_height / 2.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            self.screen_width / 2.0, self.screen_height / 2.0, 0.0, 1.0,
        );

        let uniforms = uniform! {
            MaterialInfo: &self.material_buffer,
            LineInfo: &self.line_buffer,
            LightInfo: &self.light_buffer,
            ViewportMatrix: viewport.into_col_arrays(),
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.ogre.render(frame, &self.program, &draw_params, &uniforms)?;

        Ok(())
    }

    fn resize(&mut self, _display: &impl Facade, width: u32, height: u32) -> GLResult<()> {
        self.screen_width  = width as f32;
        self.screen_height = height as f32;
        Ok(())
    }

    fn is_animating(&self) -> bool { false }
    fn toggle_animation(&mut self) {}
}


impl SceneShadeWire {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/shadewire.vert.glsl");
        let geometry_shader_code = include_str!("shaders/shadewire.geom.glsl");
        let fragment_shader_code = include_str!("shaders/shadewire.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_geometry_shader(geometry_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
