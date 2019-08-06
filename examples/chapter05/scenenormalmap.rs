
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{ObjMesh, ObjMeshConfiguration};
use cookbook::texture::load_texture;
use cookbook::{Mat4F, Mat3F, Vec3F, Vec4F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::texture::texture2d::Texture2d;
use glium::{Surface, uniform, implement_uniform_block};


#[derive(Debug)]
pub struct SceneNormalMap {

    program: glium::Program,

    ogre: ObjMesh,
    diffuse_tex: Texture2d,
    normal_tex: Texture2d,

    material_buffer: UniformBuffer<MaterialInfo>,
    light_buffer   : UniformBuffer<LightInfo>,

    view: Mat4F,
    projection : Mat4F,

    angle: f32,
    is_animate: bool,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct LightInfo {
    LightPosition: [f32; 4],
    L : [f32; 3], _padding1: f32,
    La: [f32; 3],
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct MaterialInfo {
    Ks: [f32; 3],
    Shininess: f32,
}


impl Scene for SceneNormalMap {

    fn new(display: &impl Facade) -> GLResult<SceneNormalMap> {

        // Shader Program ------------------------------------------------------------
        let program = SceneNormalMap::compile_shader_program(display)
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
        let diffuse_tex = load_texture(display, "media/texture/ogre_diffuse.png")?;
        let normal_tex  = load_texture(display, "media/texture/ogre_normalmap.png")?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let view = Mat4F::look_at_rh(Vec3F::new(-1.0, 0.25, 2.0), Vec3F::zero(), Vec3F::unit_y());
        let projection = Mat4F::identity();
        let angle = 100.0_f32.to_radians();
        let is_animate = true;
        // ----------------------------------------------------------------------------


        // Initialize Uniforms --------------------------------------------------------
        glium::implement_uniform_block!(LightInfo, LightPosition, L, La);
        let light_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;

        glium::implement_uniform_block!(MaterialInfo, Ks, Shininess);
        let material_buffer = UniformBuffer::immutable(display, MaterialInfo {
            Ks: [0.2, 0.2, 0.2],
            Shininess: 1.0,
        }).map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------

        let scene = SceneNormalMap {
            program,
            ogre, diffuse_tex, normal_tex,
            material_buffer, light_buffer,
            view, projection, angle, is_animate,
        };
        Ok(scene)
    }

    fn update(&mut self, delta_time: f32) {

        const TWO_PI: f32 = std::f32::consts::PI * 2.0;
        const ROTATE_SPEED: f32 = std::f32::consts::PI / 2.0;

        if self.is_animating() {
            self.angle = (self.angle + delta_time * ROTATE_SPEED) % TWO_PI;
        }
    }

    fn render(&self, frame: &mut glium::Frame) -> GLResult<()> {

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

        // Render teapot ----------------------------------------------------------
        let light_pos = self.view * Vec4F::new(10.0 * self.angle.cos(), 1.0, 10.0 * self.angle.sin(), 1.0);
        self.light_buffer.write(& LightInfo {
            LightPosition: light_pos.into_array(),
            L: [1.0_f32, 1.0, 1.0],
            La: [0.2_f32, 0.2, 0.2], ..Default::default()
        });
        let model = Mat4F::identity();
        let mv: Mat4F = self.view * model;

        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            ColorTex: self.diffuse_tex.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
            NormalMapTex: self.normal_tex.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.ogre.render(frame, &self.program, &draw_params, &uniforms)
        // -------------------------------------------------------------------------
    }

    fn resize(&mut self, _width: u32, _height: u32) {
        
        const C: f32 = 2.0;
        self.projection = Mat4F::orthographic_rh_zo(vek::FrustumPlanes {
            left: -0.4 * C, right: 0.4 * C, bottom: -0.3 * C, top: 0.3 * C,
            near: 0.1, far: 100.0,
        });
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }
    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate;
    }
}


impl SceneNormalMap {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/normalmap.vert.glsl");
        let fragment_shader_code = include_str!("shaders/normalmap.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
