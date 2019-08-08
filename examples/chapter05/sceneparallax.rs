
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::Plane;
use cookbook::texture::load_texture;
use cookbook::{Mat4F, Mat3F, Vec3F, Vec4F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::texture::texture2d::Texture2d;
use glium::{Surface, uniform, implement_uniform_block};
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};


#[derive(Debug)]
pub struct SceneParallax {

    program: glium::Program,

    plane: Plane,
    normal_map: Texture2d,
    height_map: Texture2d,
    color_map : Texture2d,

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


impl Scene for SceneParallax {

    fn new(display: &impl Facade) -> GLResult<SceneParallax> {

        // Shader Program ------------------------------------------------------------
        let program = SceneParallax::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let plane = Plane::new(display, 8.0, 8.0, 1, 1, 1.0, 1.0)?;
        // ----------------------------------------------------------------------------

        // Initialize Textures --------------------------------------------------------
        let normal_map = load_texture(display, "media/texture/mybrick/mybrick-normal.png")?;
        let height_map = load_texture(display, "media/texture/mybrick/mybrick-height.png")?;
        let color_map  = load_texture(display, "media/texture/mybrick/mybrick-color.png")?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let view = Mat4F::look_at_rh(Vec3F::new(-1.0, 0.0, 8.0), Vec3F::zero(), Vec3F::unit_y());
        let projection = Mat4F::identity();
        let angle = 100.0_f32.to_radians();
        let is_animate = true;
        // ----------------------------------------------------------------------------


        // Initialize Uniforms --------------------------------------------------------
        glium::implement_uniform_block!(LightInfo, LightPosition, L, La);
        let light_buffer = UniformBuffer::immutable(display, LightInfo {
            LightPosition: (view * Vec4F::new(2.0, 2.0, 1.0, 1.0)).into_array(),
            L: [0.7_f32, 0.7, 0.7],
            La: [0.01_f32, 0.01, 0.01], ..Default::default()
        }).map_err(BufferCreationErrorKind::UniformBlock)?;

        glium::implement_uniform_block!(MaterialInfo, Ks, Shininess);
        let material_buffer = UniformBuffer::immutable(display, MaterialInfo {
            Ks: [0.7, 0.7, 0.7],
            Shininess: 40.0,
        }).map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------

        let scene = SceneParallax {
            program,
            plane, normal_map, height_map, color_map,
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

        // Render plane -----------------------------------------------------------
        let model = Mat4F::rotation_x(90.0_f32.to_radians())
            .rotated_y(self.angle.cos());
        let mv: Mat4F = self.view * model;

        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            ColorTex    : self.color_map. sampled().minify_filter(MinifySamplerFilter::Nearest).magnify_filter(MagnifySamplerFilter::Linear),
            NormalMapTex: self.normal_map.sampled().minify_filter(MinifySamplerFilter::Nearest).magnify_filter(MagnifySamplerFilter::Linear),
            HeightMapTex: self.height_map.sampled().minify_filter(MinifySamplerFilter::Nearest).magnify_filter(MagnifySamplerFilter::Linear),
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.plane.render(frame, &self.program, &draw_params, &uniforms)
        // -------------------------------------------------------------------------
    }

    fn resize(&mut self, width: u32, height: u32) {

        self.projection = Mat4F::perspective_rh_zo(60.0_f32.to_radians(), width as f32 / height as f32, 0.3, 100.0);
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }
    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate;
    }
}


impl SceneParallax {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/parallax.vert.glsl");
        let fragment_shader_code = include_str!("shaders/parallax.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
