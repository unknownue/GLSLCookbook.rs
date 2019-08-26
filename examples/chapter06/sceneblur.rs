
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{Teapot, Plane, Torus, Quad};
use cookbook::{Mat4F, Mat3F, Vec3F};
use cookbook::framebuffer::{ColorDepthAttachment, ColorAttachment, GLFrameBuffer};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::texture::UncompressedFloatFormat;
use glium::{Surface, uniform, implement_uniform_block};


pub struct SceneBlur {

    programs: [glium::Program; 3],

    teapot: Teapot,
    plane: Plane,
    torus: Torus,
    fs_quad: Quad,

    render_fbo      : GLFrameBuffer<ColorDepthAttachment>,
    intermediate_fbo: GLFrameBuffer<ColorAttachment>,

    weights: [f32; 5],

    weight_buffer: UniformBuffer<[f32; 5]>,
    material_buffer: UniformBuffer<MaterialInfo>,
    light_buffer: UniformBuffer<LightInfo>,

    aspect_ratio: f32,
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
    Ka: [f32; 3], _padding1: f32,
    Kd: [f32; 3], _padding2: f32,
    Ks: [f32; 3],
    Shininess: f32,
}


impl Scene for SceneBlur {

    fn new(display: &impl Facade) -> GLResult<SceneBlur> {

        let (screen_width, screen_height) = display.get_context().get_framebuffer_dimensions();
        let aspect_ratio = (screen_width as f32) / (screen_height as f32);

        // Shader Program ------------------------------------------------------------
        let programs = SceneBlur::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Compute and sum the weights ------------------------------------------------
        let mut weights: [f32; 5] = Default::default();
        let sigma2 = 8.0;

        weights[0] = gauss(0.0, sigma2);
        let mut sum = weights[0];

        for i in 1..5 {
            weights[i] = gauss(i as f32, sigma2);
            sum += 2.0 * weights[i];
        }

        // Normalize the weights and set the uniform
        for i in 0..5 {
            weights[i] = weights[i] / sum;
        }
        // ----------------------------------------------------------------------------

        // Initialize Mesh ------------------------------------------------------------
        let teapot = Teapot::new(display, 14, Mat4F::identity())?;
        let plane = Plane::new(display, 50.0, 50.0, 1, 1, 1.0, 1.0)?;
        let torus = Torus::new(display, 0.7 * 1.5, 0.3 * 1.5, 50, 50)?;
        let fs_quad = Quad::new(display)?;
        // ----------------------------------------------------------------------------

        // Initialize Textures --------------------------------------------------------
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let angle = std::f32::consts::PI / 4.0;
        let is_animate = true;
        // ----------------------------------------------------------------------------


        // Initialize Uniforms --------------------------------------------------------
        let render_fbo = GLFrameBuffer::setup(display, screen_width, screen_height, UncompressedFloatFormat::U8U8U8U8)?;
        let intermediate_fbo = GLFrameBuffer::setup(display, screen_width, screen_height, UncompressedFloatFormat::U8U8U8U8)?;

        glium::implement_uniform_block!(LightInfo, LightPosition, L, La);
        let light_buffer = UniformBuffer::immutable(display, LightInfo {
            LightPosition: [0.0_f32, 0.0, 0.0, 1.0],
            L: [1.0_f32, 1.0, 1.0],
            La: [0.2_f32, 0.2, 0.2], ..Default::default()
        }).map_err(BufferCreationErrorKind::UniformBlock)?;
        glium::implement_uniform_block!(MaterialInfo, Ka, Kd, Ks, Shininess);
        let material_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;

        let weight_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------

        let scene = SceneBlur {
            programs, render_fbo, intermediate_fbo,
            teapot, torus, plane, fs_quad, weights,
            weight_buffer, material_buffer, light_buffer,
            aspect_ratio, angle, is_animate,
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

        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        self.pass1(&draw_params)?;
        self.pass2()?;
        self.pass3(frame, &draw_params)
    }

    fn resize(&mut self, display: &impl Facade, width: u32, height: u32) -> GLResult<()> {
        self.aspect_ratio = width as f32 / height as f32;
        self.render_fbo = GLFrameBuffer::setup(display, width, height, UncompressedFloatFormat::U8U8U8U8)?;
        self.intermediate_fbo = GLFrameBuffer::setup(display, width, height, UncompressedFloatFormat::U8U8U8U8)?;
        Ok(())
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }
    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate;
    }
}


impl SceneBlur {

    #[cfg(not(target_os = "macos"))]
    fn compile_shader_program(display: &impl Facade) -> Result<[Program; 3], ProgramCreationError> {

        let pass1_vertex   = include_str!("shaders/blur/pass1.vert.glsl");
        let pass1_fragment = include_str!("shaders/blur/pass1.frag.glsl");

        let pass2_vertex   = include_str!("shaders/blur/pass2.vert.glsl");
        let pass2_fragment = include_str!("shaders/blur/pass2.frag.glsl");

        let pass3_vertex   = include_str!("shaders/blur/pass3.vert.glsl");
        let pass3_fragment = include_str!("shaders/blur/pass3.frag.glsl");

        let pass1 = glium::Program::new(display, GLSourceCode::new(pass1_vertex, pass1_fragment).with_srgb_output(false))?;
        let pass2 = glium::Program::new(display, GLSourceCode::new(pass2_vertex, pass2_fragment).with_srgb_output(false))?;
        let pass3 = glium::Program::new(display, GLSourceCode::new(pass3_vertex, pass3_fragment).with_srgb_output(true))?;
        Ok([pass1, pass2, pass3])
    }

    // There is a issue when transfering the weights to shader on macOS.
    // See https://github.com/unknownue/GLSLCookbook.rs/issues/5 for detail.
    // Here we use a shader that pre-calcualtes the weights in it.
    #[cfg(target_os = "macos")]
    fn compile_shader_program(display: &impl Facade) -> Result<[Program; 3], ProgramCreationError> {

        let pass1_vertex   = include_str!("shaders/blur/pass1.vert.glsl");
        let pass1_fragment = include_str!("shaders/blur/pass1.frag.glsl");

        let pass2_vertex   = include_str!("shaders/blur/pass2.vert.glsl");
        let pass2_fragment = include_str!("shaders/blur/pass2_macOS.frag.glsl");

        let pass3_vertex   = include_str!("shaders/blur/pass3.vert.glsl");
        let pass3_fragment = include_str!("shaders/blur/pass3_macOS.frag.glsl");

        let pass1 = glium::Program::new(display, GLSourceCode::new(pass1_vertex, pass1_fragment).with_srgb_output(false))?;
        let pass2 = glium::Program::new(display, GLSourceCode::new(pass2_vertex, pass2_fragment).with_srgb_output(false))?;
        let pass3 = glium::Program::new(display, GLSourceCode::new(pass3_vertex, pass3_fragment).with_srgb_output(true))?;
        Ok([pass1, pass2, pass3])
    }

    fn pass1(&mut self, draw_params: &glium::DrawParameters) -> GLResult<()> {

        let program = &self.programs[0];

        let view = Mat4F::look_at_rh(Vec3F::new(7.0 * self.angle.cos(), 4.0, 7.0 * self.angle.sin()), Vec3F::zero(), Vec3F::unit_y());
        let projection = Mat4F::perspective_rh_zo(60.0_f32.to_radians(), self.aspect_ratio, 0.3, 100.0);


        // Render teapot ---------------------------------------------------------
        self.material_buffer.write(&MaterialInfo {
            Ka: [0.1, 0.1, 0.1],
            Kd: [0.9, 0.9, 0.9],
            Ks: [0.95, 0.95, 0.95],
            Shininess: 100.0, ..Default::default()
        });

        let model = Mat4F::rotation_x(-90.0_f32.to_radians());
        let mv: Mat4F = view * model;

        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (projection * mv).into_col_arrays(),
        };

        let teapot = &self.teapot;
        self.render_fbo.rent_mut(|(framebuffer, _)| {

            framebuffer.clear_color(0.0, 0.0, 0.0, 1.0);
            framebuffer.clear_depth(1.0);
            // TODO: handle unwrap()
            teapot.render(framebuffer, program, draw_params, &uniforms).unwrap();
        });
        // ------------------------------------------------------------------------- 

        // Render plane ------------------------------------------------------------
        self.material_buffer.write(&MaterialInfo {
            Ka: [0.1, 0.1, 0.1],
            Kd: [0.4, 0.4, 0.4],
            Ks: [0.0, 0.0, 0.0],
            Shininess: 1.0, ..Default::default()
        });

        let model = Mat4F::translation_3d(Vec3F::new(0.0, -0.75, 0.0));
        let mv: Mat4F = view * model;

        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (projection * mv).into_col_arrays(),
        };

        let plane = &self.plane;
        self.render_fbo.rent_mut(|(framebuffer, _)| {
            // TODO: handle unwrap()
            plane.render(framebuffer, program, draw_params, &uniforms).unwrap();
        });
        // ------------------------------------------------------------------------- 

        // Render torus ------------------------------------------------------------
        self.material_buffer.write(&MaterialInfo {
            Ka: [0.1, 0.1, 0.1],
            Kd: [0.9, 0.5, 0.2],
            Ks: [0.95, 0.95, 0.95],
            Shininess: 100.0, ..Default::default()
        });

        let model = Mat4F::rotation_x(90.0_f32.to_radians())
            .translated_3d(Vec3F::new(1.0, 1.0, 3.0));
        let mv: Mat4F = view * model;

        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (projection * mv).into_col_arrays(),
        };

        let torus = &self.torus;
        self.render_fbo.rent_mut(|(framebuffer, _)| {
            // TODO: handle unwrap()
            torus.render(framebuffer, program, draw_params, &uniforms).unwrap();
        });
        // ------------------------------------------------------------------------- 
        Ok(())
    }

    fn pass2(&mut self) -> GLResult<()> {

        let render_fbo = &self.render_fbo;
        let fs_quad = &self.fs_quad;
        let program = &self.programs[1];

        self.weight_buffer.write(&self.weights);
        let weight_buffer = &self.weight_buffer;

        self.intermediate_fbo.rent_mut(|(framebuffer, _)| {

            framebuffer.clear_color(0.0, 0.0, 0.0, 1.0);

            render_fbo.rent(|(_, attachment)| {

                let uniforms = uniform! {
                    WeightBlock: weight_buffer,
                    Texture0: attachment.color.sampled()
                        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                };

                // Disable depth test
                fs_quad.render(framebuffer, program, &Default::default(), &uniforms).unwrap();
            });
        });

        Ok(())
    }

    fn pass3(&self, frame: &mut glium::Frame, draw_params: &glium::DrawParameters) -> GLResult<()> {

        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame.clear_depth(1.0);

        self.intermediate_fbo.rent(|(_, attachment)| {

            let uniforms = uniform! {
                WeightBlock: &self.weight_buffer,
                Texture0: attachment.color.sampled()
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            };

            // TODO: handle unwrap()
            self.fs_quad.render(frame, &self.programs[2], draw_params, &uniforms).unwrap();
        });

        Ok(())
    }
}

fn gauss(x: f32, sigma2: f32) -> f32 {
    const TWO_PI: f64 = std::f64::consts::PI * 2.0;
    let sigma2 = sigma2 as f64;
    let x = x as f64;

	let coeff: f64 = 1.0 / (TWO_PI * sigma2);
    let expon: f64 = -(x * x) / (2.0 * sigma2);

    (coeff * expon.exp()) as f32
}
