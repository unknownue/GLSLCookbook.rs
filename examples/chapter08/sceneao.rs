
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{ObjMesh, ObjMeshConfiguration};
use cookbook::texture::load_texture;
use cookbook::{Mat4F, Mat3F, Vec3F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::texture::texture2d::Texture2d;
use glium::{Surface, uniform, implement_uniform_block};



#[derive(Debug)]
pub struct SceneAo {

    program: glium::Program,

    ogre: ObjMesh,
    ao_tex: Texture2d,
    diffuse_tex: Texture2d,

    light_buffer: UniformBuffer<LightInfo>,

    projection: Mat4F,

    angle: f32,
    is_animate: bool,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct LightInfo {
    LightPosition: [f32; 4],
    Intensity: [f32; 3], _padding1: f32,
}


impl Scene for SceneAo {

    fn new(display: &impl Facade) -> GLResult<SceneAo> {

        // Shader Program ------------------------------------------------------------
        let program = SceneAo::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------

        // Initialize Mesh ------------------------------------------------------------
        let ogre = ObjMesh::load(display, "media/bs_ears.obj", ObjMeshConfiguration {
            is_with_adjacency: false,
            is_gen_tangents: true,
            is_center: false,
            is_print_load_message: true,
        })?;
        // ----------------------------------------------------------------------------

        // Initialize Textures --------------------------------------------------------
        let ao_tex = load_texture(display, "media/texture/ao_ears.png")?;
        let diffuse_tex = load_texture(display, "media/texture/ogre_diffuse.png")?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let projection = Mat4F::identity();
        let angle = std::f32::consts::PI / 2.0;
        let is_animate = true;
        // ----------------------------------------------------------------------------

        // Initialize Uniforms --------------------------------------------------------
        glium::implement_uniform_block!(LightInfo, LightPosition, Intensity);
        let light_buffer = UniformBuffer::immutable(display, LightInfo {
            LightPosition: [0.0, 0.0, 0.0, 1.0],
            Intensity: [1.0, 1.0, 1.0], ..Default::default()
        }).map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------


        let scene = SceneAo {
            program, ogre,
            ao_tex, diffuse_tex,
            light_buffer,
            projection, angle, is_animate,
        };
        Ok(scene)
    }

    fn update(&mut self, delta_time: f32) {
        const TWO_PI: f32 = std::f32::consts::PI * 2.0;

        if self.is_animating() {
            self.angle = (self.angle + delta_time) % TWO_PI;
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

        // Render Ogre --------------------------------------------------------------
        let model = Mat4F::rotation_y(90.0_f32.to_radians());
        let view = Mat4F::look_at_rh(Vec3F::new(3.0 * self.angle.cos(), 0.0, 3.0 * self.angle.sin()), Vec3F::zero(), Vec3F::unit_y());
        let mv: Mat4F = view * model;

        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            AOTex: self.ao_tex.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
            DiffTex: self.diffuse_tex.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.ogre.render(frame, &self.program, &draw_params, &uniforms)
        // -------------------------------------------------------------------------
    }

    fn resize(&mut self, _display: &impl Facade, _width: u32, _height: u32) -> GLResult<()> {

        const C: f32 = 2.25;
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
        self.is_animate = !self.is_animate
    }
}


impl SceneAo {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/ao.vert.glsl");
        let fragment_shader_code = include_str!("shaders/ao.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
