
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{SkyBox, Teapot};
use cookbook::texture::{load_cubemap, CubeMapFaceExtension};
use cookbook::{Mat4F, Vec3F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::{UniformBuffer, MagnifySamplerFilter, MinifySamplerFilter, SamplerWrapFunction};
use glium::texture::cubemap::Cubemap;
use glium::{Surface, uniform, implement_uniform_block};


#[derive(Debug)]
pub struct SceneReflectCube {

    program: glium::Program,
    sky_prog: glium::Program,

    teapot: Teapot,
    skybox: SkyBox,
    cube_tex: Cubemap,

    material_buffer: UniformBuffer<MaterialInfo>,

    projection : Mat4F,

    angle: f32,
    is_animate: bool,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct MaterialInfo {
    MaterialColor: [f32; 4],
    ReflectFactor: f32,
}


impl Scene for SceneReflectCube {

    fn new(display: &impl Facade) -> GLResult<SceneReflectCube> {

        // Shader Program ------------------------------------------------------------
        let program = SceneReflectCube::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        let sky_prog = SceneReflectCube::compile_sky_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // cookbook::utils::print_active_uniform_blocks(&program);
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let teapot = Teapot::new(display, 14, Mat4F::identity())?;
        let skybox = SkyBox::new(display, 100.0)?;
        // ----------------------------------------------------------------------------

        // Initialize Textures --------------------------------------------------------
        let cube_tex = load_cubemap(display, "media/texture/cube/pisa-hdr/pisa", CubeMapFaceExtension::Hdr)?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let projection = Mat4F::identity();
        let angle = 90.0_f32.to_radians();
        let is_animate = true;
        // ----------------------------------------------------------------------------

        // Initialize Uniforms --------------------------------------------------------
        glium::implement_uniform_block!(MaterialInfo, MaterialColor, ReflectFactor);
        let material_buffer = UniformBuffer::immutable(display, MaterialInfo {
            MaterialColor: [0.5, 0.5, 0.5, 1.0],
            ReflectFactor: 0.85,
        }).map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------

        let scene = SceneReflectCube {
            program, sky_prog,
            teapot, skybox, cube_tex,
            material_buffer,
            projection, angle, is_animate,
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

        // Render Sky -------------------------------------------------------------
        let camera_pos = Vec3F::new(7.0 * self.angle.cos(), 2.0, 7.0 * self.angle.sin());
        let view = Mat4F::look_at_rh(camera_pos, Vec3F::zero(), Vec3F::unit_y());
        let model = Mat4F::identity();
        let mv: Mat4F = view * model;

        let uniforms = uniform! {
            SkyBoxTex: self.cube_tex.sampled()
                .minify_filter(MinifySamplerFilter::Nearest)
                .magnify_filter(MagnifySamplerFilter::Linear)
                .wrap_function(SamplerWrapFunction::Clamp),
            MVP: (self.projection * mv).into_col_arrays(),
        };
        self.skybox.render(frame, &self.sky_prog, &draw_params, &uniforms)?;
        // -------------------------------------------------------------------------

        // Render scene ------------------------------------------------------------
        let model = Mat4F::rotation_x(-90.0_f32.to_radians())
            .translated_3d(Vec3F::new(0.0, -1.0, 0.0));
        let mv: Mat4F = view * model;

        let uniforms = uniform! {
            MaterialInfo: &self.material_buffer,
            CubeMapTex: self.cube_tex.sampled()
                .minify_filter(MinifySamplerFilter::Nearest)
                .magnify_filter(MagnifySamplerFilter::Linear)
                .wrap_function(SamplerWrapFunction::Clamp),
            WorldCameraPosition: camera_pos.into_array(),
            ModelMatrix: model.into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.teapot.render(frame, &self.program, &draw_params, &uniforms)
        // -------------------------------------------------------------------------
    }

    fn resize(&mut self, width: u32, height: u32) {

        self.projection = Mat4F::perspective_rh_zo(50.0_f32.to_radians(), width as f32 / height as f32, 0.3, 100.0);
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }
    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate;
    }
}


impl SceneReflectCube {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/cubemap_reflect.vert.glsl");
        let fragment_shader_code = include_str!("shaders/cubemap_reflect.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }

    fn compile_sky_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/skybox.vert.glsl");
        let fragment_shader_code = include_str!("shaders/skybox.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
