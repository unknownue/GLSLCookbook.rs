
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{Teapot, Plane, Torus, Quad};
use cookbook::{Mat4F, Mat3F, Vec3F};
use cookbook::noise;
use cookbook::framebuffer::{ColorDepthAttachment, GLFrameBuffer};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::texture::texture2d::Texture2d;
use glium::texture::{UncompressedFloatFormat, MipmapsOption};
use glium::{Surface, uniform, implement_uniform_block};


pub struct SceneNightVision {

    programs: [glium::Program; 2],

    teapot  : Teapot,
    plane   : Plane,
    torus   : Torus,
    fs_quad : Quad,

    render_fbo: GLFrameBuffer<ColorDepthAttachment>,
    noise_tex: Texture2d,

    material_buffer : UniformBuffer<MaterialInfo>,
    light_buffer    : UniformBuffer<LightInfo>,

    angle: f32,
    is_animate: bool,
    projection: Mat4F,

    screen_width : i32,
    screen_height: i32,
}


#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct LightInfo {
    LightPosition: [f32; 4],
    Intensity: [f32; 3], _padding: f32,
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


impl Scene for SceneNightVision {

    fn new(display: &impl Facade) -> GLResult<SceneNightVision> {

        let (screen_width, screen_height) = display.get_context().get_framebuffer_dimensions();

        // Shader Program ------------------------------------------------------------
        let programs = SceneNightVision::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let teapot = Teapot::new(display, 14, Mat4F::identity())?;
        let plane = Plane::new(display, 50.0, 50.0, 1, 1, 1.0, 1.0)?;
        let torus = Torus::new(display, 0.7 * 1.5, 0.3 * 1.5, 50, 50)?;
        let fs_quad = Quad::new(display)?;
        // ----------------------------------------------------------------------------

        // Initialize Textures --------------------------------------------------------
        let noise_tex = noise::generate_periodic_2d_texture(display, 200.0, 0.5, 512, 512, MipmapsOption::NoMipmap)?;
        // ----------------------------------------------------------------------------

        // Initialize FrameBuffer Objects ---------------------------------------------
        let render_fbo = GLFrameBuffer::setup(display, screen_width, screen_height, UncompressedFloatFormat::U8U8U8U8)?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let projection = Mat4F::identity();
        let angle = std::f32::consts::PI / 4.0;
        let is_animate = true;
        // ----------------------------------------------------------------------------


        // Initialize Uniforms --------------------------------------------------------
        glium::implement_uniform_block!(LightInfo, LightPosition, Intensity);
        let light_buffer = UniformBuffer::immutable(display, LightInfo {
            LightPosition: [0.0, 0.0, 0.0, 1.0],
            Intensity: [1.0, 1.0, 1.0], ..Default::default()
        }).map_err(BufferCreationErrorKind::UniformBlock)?;

        glium::implement_uniform_block!(MaterialInfo, Ka, Kd, Ks, Shininess);
        let material_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------

        let scene = SceneNightVision {
            programs, render_fbo,
            teapot, torus, plane, fs_quad, noise_tex,
            material_buffer, light_buffer,
            angle, is_animate, projection,
            screen_width : screen_width as i32,
            screen_height: screen_height as i32,
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

        self.pass1()?;
        self.pass2(frame)
    }

    fn resize(&mut self, display: &impl Facade, width: u32, height: u32) -> GLResult<()> {
        self.screen_width  = width  as i32;
        self.screen_height = height as i32;
        self.render_fbo = GLFrameBuffer::setup(display, width, height, UncompressedFloatFormat::U8U8U8U8)?;
        self.projection = Mat4F::perspective_rh_zo(60.0_f32.to_radians(), width as f32 / height as f32, 0.3, 100.0);
        Ok(())
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }

    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate
    }
}


impl SceneNightVision {

    fn compile_shader_program(display: &impl Facade) -> Result<[Program; 2], ProgramCreationError> {

        let pass1_vertex   = include_str!("shaders/nightvision/pass1.vert.glsl");
        let pass1_fragment = include_str!("shaders/nightvision/pass1.frag.glsl");

        let pass2_vertex   = include_str!("shaders/nightvision/pass2.vert.glsl");
        let pass2_fragment = include_str!("shaders/nightvision/pass2.frag.glsl");

        let pass1 = glium::Program::new(display, GLSourceCode::new(pass1_vertex, pass1_fragment).with_srgb_output(false))?;
        let pass2 = glium::Program::new(display, GLSourceCode::new(pass2_vertex, pass2_fragment).with_srgb_output(true))?;
        Ok([pass1, pass2])
    }

    fn pass1(&mut self) -> GLResult<()> {

        let program_pass1 = &self.programs[0];
        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let view = Mat4F::look_at_rh(Vec3F::new(7.0 * self.angle.cos(), 4.0, 7.0 * self.angle.sin()), Vec3F::zero(), Vec3F::unit_y());

        // Render Teapot --------------------------------------------------------
        let model = Mat4F::rotation_x(-90.0_f32.to_radians());
        let mv: Mat4F = view * model;

        self.material_buffer.write(&MaterialInfo {
            Ka: [0.1, 0.1, 0.1],
            Kd: [0.9, 0.9, 0.9],
            Ks: [0.95, 0.95, 0.95],
            Shininess: 100.0, ..Default::default()
        });

        let uniforms = uniform! {
            MaterialInfo: &self.material_buffer,
            LightInfo: &self.light_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        let teapot = &self.teapot;
        self.render_fbo.rent_mut(|(framebuffer, _)| -> GLResult<()> {

            framebuffer.clear_color(0.5, 0.5, 0.5, 1.0);
            framebuffer.clear_depth(1.0);

            teapot.render(framebuffer, program_pass1, &draw_params, &uniforms)
        })?;
        // ------------------------------------------------------------------------- 

        // Render Plane ------------------------------------------------------------
        let model = Mat4F::translation_3d(Vec3F::new(0.0, -0.75, 0.0));
        let mv: Mat4F = view * model;

        self.material_buffer.write(&MaterialInfo {
            Ka: [0.1, 0.1, 0.1],
            Kd: [0.4, 0.4, 0.4],
            Ks: [0.0, 0.0, 0.0],
            Shininess: 1.0, ..Default::default()
        });

        let uniforms = uniform! {
            MaterialInfo: &self.material_buffer,
            LightInfo: &self.light_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        let plane = &self.plane;
        self.render_fbo.rent_mut(|(framebuffer, _)| -> GLResult<()> {
            plane.render(framebuffer, program_pass1, &draw_params, &uniforms)
        })?;
        // ------------------------------------------------------------------------- 

        // Render Torus ------------------------------------------------------------
        let model = Mat4F::rotation_x(90.0_f32.to_radians())
            .translated_3d(Vec3F::new(1.0, 1.0, 3.0));
        let mv: Mat4F = view * model;

        self.material_buffer.write(&MaterialInfo {
            Ka: [0.1, 0.1, 0.1],
            Kd: [0.9, 0.5, 0.2],
            Ks: [0.95, 0.95, 0.95],
            Shininess: 100.0, ..Default::default()
        });

        let uniforms = uniform! {
            MaterialInfo: &self.material_buffer,
            LightInfo: &self.light_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        let torus = &self.torus;
        self.render_fbo.rent_mut(|(framebuffer, _)| -> GLResult<()> {
            torus.render(framebuffer, program_pass1, &draw_params, &uniforms)
        })?;
        // ------------------------------------------------------------------------- 

        Ok(())
    }

    fn pass2(&self, frame: &mut glium::Frame) -> GLResult<()> {

        frame.clear_color(0.5, 0.5, 0.5, 1.0);

        self.render_fbo.rent(|(_, attachment)| -> GLResult<()> {

            let uniforms = uniform! {
                Width : self.screen_width,
                Height: self.screen_height,
                Radius: self.screen_width as f32 / 3.5,
                RenderTex: attachment.color.sampled()
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                NoiseTex: self.noise_tex.sampled()
                    .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat)
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Linear)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear), 
            };

            self.fs_quad.render(frame, &self.programs[1], &Default::default(), &uniforms)
        })
    }
}
