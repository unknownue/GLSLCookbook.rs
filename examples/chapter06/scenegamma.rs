
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{ObjMesh, ObjMeshConfiguration};
use cookbook::{Mat4F, Mat3F, Vec3F, Vec4F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::{Surface, uniform, implement_uniform_block};


#[derive(Debug)]
pub struct SceneGamma {

    program: glium::Program,

    ogre: ObjMesh,
    
    material_buffer: UniformBuffer<MaterialInfo>,
    light_buffer   : UniformBuffer<LightInfo>,

    angle: f32,
    is_animate: bool,

    projection: Mat4F,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct LightInfo {
    LightPosition: [f32; 4],
    Intensity: [f32; 3], _padding1: f32,
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


impl Scene for SceneGamma {

    fn new(display: &impl Facade) -> GLResult<SceneGamma> {

        // Shader Program ------------------------------------------------------------
        let program = SceneGamma::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let ogre = ObjMesh::load(display, "media/bs_ears.obj", ObjMeshConfiguration {
            is_with_adjacency: false,
            is_gen_tangents: false,
            is_center: false,
            is_print_load_message: true,
        })?;
        // ----------------------------------------------------------------------------


        // Initialize MVP -------------------------------------------------------------
        let angle = std::f32::consts::PI / 2.0;
        let projection = Mat4F::identity();
        let is_animate = true;
        // ----------------------------------------------------------------------------


        // Initialize Uniforms --------------------------------------------------------
        glium::implement_uniform_block!(LightInfo, LightPosition, Intensity);
        let light_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;

        glium::implement_uniform_block!(MaterialInfo, Ka, Kd, Ks, Shininess);
        let material_buffer = UniformBuffer::immutable(display, MaterialInfo {
            Ka: [0.0_f32, 0.0, 0.0],
            Kd: [1.0_f32, 1.0, 1.0],
            Ks: [0.0_f32, 0.0, 0.0],
            Shininess: 100.0_f32, ..Default::default()
        }).map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------


        let scene = SceneGamma {
            program,
            ogre, material_buffer, light_buffer,
            projection, is_animate, angle,
        };
        Ok(scene)
    }

    fn update(&mut self, delta_time: f32) {

        const TWO_PI: f32 = std::f32::consts::PI * 2.0;
        const ROTATE_SPEED: f32 = std::f32::consts::PI / 8.0;

        if self.is_animating() {
            self.angle = (self.angle + delta_time * ROTATE_SPEED) % TWO_PI;
        }
    }

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

        let view = Mat4F::look_at_rh(Vec3F::new(3.0 * self.angle.cos(), 0.0, 3.0 * self.angle.sin()), Vec3F::zero(), Vec3F::unit_y());
        self.light_buffer.write(&LightInfo {
            LightPosition: (view * Vec4F::new(10.0, 0.0, 0.0, 1.0)).into_array(),
            Intensity : [1.0_f32, 1.0, 1.0], ..Default::default()
        });

        let model = Mat4F::identity();
        let mv: Mat4F = view * model;

        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            Gamma: 2.2_f32,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.ogre.render(frame, &self.program, &draw_params, &uniforms)
    }

    fn resize(&mut self, _display: &impl Facade, _width: u32, _height: u32) -> GLResult<()> {

        const C: f32 = 2.5;
        self.projection = Mat4F::orthographic_rh_zo(vek::FrustumPlanes {
            left: -0.4 * C, right: 0.4 * C, bottom: -0.3 * C, top: 0.3 * C,
            near: 0.1, far: 100.0,
        });;
        Ok(())
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }
    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate;
    }
}


impl SceneGamma {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/gamma.vert.glsl");
        let fragment_shader_code = include_str!("shaders/gamma.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
