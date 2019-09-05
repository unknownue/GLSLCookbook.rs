
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind};
use cookbook::objects::{Teapot, SkyBox};
use cookbook::texture::{load_cubemap, CubeMapFaceExtension};
use cookbook::noise;
use cookbook::{Mat4F, Vec3F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::texture::texture2d::Texture2d;
use glium::texture::MipmapsOption;
use glium::texture::cubemap::Cubemap;
use glium::{Surface, uniform};


#[derive(Debug)]
pub struct SceneRust {

    program: glium::Program,
    sky_program: glium::Program,

    teapot: Teapot,
    skybox: SkyBox,

    noise_tex: Texture2d,
    cube_tex: Cubemap,

    projection : Mat4F,

    angle: f32,
    is_animate: bool,
}


impl Scene for SceneRust {

    fn new(display: &impl Facade) -> GLResult<SceneRust> {

        // Shader Program ------------------------------------------------------------
        let program = SceneRust::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        let sky_program = SceneRust::compile_skybox_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let teapot = Teapot::new(display, 14, Mat4F::identity())?;
        let skybox = SkyBox::new(display, 100.0)?;
        // ----------------------------------------------------------------------------

        // Initialize Textures --------------------------------------------------------
        let noise_tex = noise::generate_2d_texture(display, 16.0, 0.5, 128, 128, MipmapsOption::NoMipmap)?;
        let cube_tex = load_cubemap(display, "media/texture/cube/pisa-hdr/pisa", CubeMapFaceExtension::Hdr)?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let projection = Mat4F::identity();
        let angle = 90.0_f32.to_radians();
        let is_animate = true;
        // ----------------------------------------------------------------------------


        let scene = SceneRust {
            program, sky_program,
            teapot, skybox, noise_tex, cube_tex,
            projection,
            angle, is_animate,
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

        frame.clear_color_srgb(0.1, 0.1, 0.1, 1.0);
        frame.clear_depth(1.0);

        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let camera_pos = Vec3F::new(7.0 * self.angle.cos(), 2.0, 7.0 * self.angle.sin());
        let view = Mat4F::look_at_rh(camera_pos, Vec3F::zero(), Vec3F::unit_y());

        // Render sky ----------------------------------------------------------
        let model = Mat4F::identity();
        let mv: Mat4F = view * model;

        let uniforms = uniform! {
            SkyBoxTex: self.cube_tex.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear)
                .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.skybox.render(frame, &self.sky_program, &draw_params, &uniforms)?;
        // -------------------------------------------------------------------------

        // Render scene ----------------------------------------------------------
        let model = Mat4F::rotation_x(-90.0_f32.to_radians())
            .translated_3d(Vec3F::new(0.0, -1.0, 0.0));
        let mv: Mat4F = view * model;

        let uniforms = uniform! {
            WorldCameraPosition: camera_pos.into_array(),
            MaterialColor: [0.7255_f32, 0.255, 0.055, 1.0],
            ReflectFactor: 0.85_f32,
            CubeMapTex: self.cube_tex.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear)
                .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp),
            NoiseTex: self.noise_tex.sampled()
                .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat)
                .minify_filter(glium::uniforms::MinifySamplerFilter::Linear)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear), 
            ModelMatrix: model.into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.teapot.render(frame, &self.program, &draw_params, &uniforms)
        // -------------------------------------------------------------------------
    }

    fn resize(&mut self, _display: &impl Facade, width: u32, height: u32) -> GLResult<()> {
        self.projection = Mat4F::perspective_rh_zo(50.0_f32.to_radians(), width as f32 / height as f32, 0.3, 100.0);
        Ok(())
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }
    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate;
    }
}


impl SceneRust {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/rust.vert.glsl");
        let fragment_shader_code = include_str!("shaders/rust.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }

    fn compile_skybox_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/skybox.vert.glsl");
        let fragment_shader_code = include_str!("shaders/skybox.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
