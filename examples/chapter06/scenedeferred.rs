
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{Teapot, Plane, Torus, Quad};
use cookbook::{Mat4F, Mat3F, Vec3F};
use cookbook::framebuffer::{DeferredPNCAttachment, GLDeferredFrameBuffer};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::{Surface, uniform, implement_uniform_block};


pub struct SceneDeferred {

    programs: [glium::Program; 2],

    teapot  : Teapot,
    plane   : Plane,
    torus   : Torus,
    fs_quad : Quad,

    deferred_fbo: GLDeferredFrameBuffer::<DeferredPNCAttachment>,

    material_buffer : UniformBuffer<MaterialInfo>,
    light_buffer    : UniformBuffer<LightInfo>,

    angle: f32,
    is_animate: bool,
    projection: Mat4F,
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
    Kd: [f32; 3],
}


impl Scene for SceneDeferred {

    fn new(display: &impl Facade) -> GLResult<SceneDeferred> {

        let (screen_width, screen_height) = display.get_context().get_framebuffer_dimensions();

        // Shader Program ------------------------------------------------------------
        let programs = SceneDeferred::compile_shader_program_pass1(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let teapot = Teapot::new(display, 14, Mat4F::identity())?;
        let plane = Plane::new(display, 50.0, 50.0, 1, 1, 1.0, 1.0)?;
        let torus = Torus::new(display, 0.7 * 1.5, 0.3 * 1.5, 50, 50)?;
        let fs_quad = Quad::new(display)?;
        // ----------------------------------------------------------------------------

        // Initialize FrameBuffer Objects ---------------------------------------------
        let deferred_fbo = GLDeferredFrameBuffer::setup(display, screen_width, screen_height)?;
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

        glium::implement_uniform_block!(MaterialInfo, Kd);
        let material_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------

        let scene = SceneDeferred {
            programs, deferred_fbo,
            teapot, torus, plane, fs_quad,
            material_buffer, light_buffer,
            angle, is_animate, projection,
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
        self.deferred_fbo = GLDeferredFrameBuffer::setup(display, width, height)?;
        self.projection   = Mat4F::perspective_rh_zo(60.0_f32.to_radians(), width as f32 / height as f32, 0.3, 100.0);
        Ok(())
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }

    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate
    }
}


impl SceneDeferred {

    fn compile_shader_program_pass1(display: &impl Facade) -> Result<[Program; 2], ProgramCreationError> {

        let pass1_vertex   = include_str!("shaders/deferred/pass1.vert.glsl");
        let pass1_fragment = include_str!("shaders/deferred/pass1.frag.glsl");

        let pass2_vertex   = include_str!("shaders/deferred/pass2.vert.glsl");
        let pass2_fragment = include_str!("shaders/deferred/pass2.frag.glsl");

        let pass1 = glium::Program::new(display, GLSourceCode::new(pass1_vertex, pass1_fragment).with_srgb_output(true))?;
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

        self.material_buffer.write(&MaterialInfo { Kd: [0.9, 0.9, 0.9] });

        let uniforms = uniform! {
            MaterialInfo: &self.material_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        let teapot = &self.teapot;
        self.deferred_fbo.rent_mut(|(framebuffer, _)| {

            framebuffer.clear_color(0.0, 0.0, 0.0, 1.0);
            framebuffer.clear_depth(1.0);
            // TODO: handle unwrap()
            teapot.render(framebuffer, program_pass1, &draw_params, &uniforms).unwrap();
        });
        // ------------------------------------------------------------------------- 

        // Render Plane ------------------------------------------------------------
        let model = Mat4F::translation_3d(Vec3F::new(0.0, -0.75, 0.0));
        let mv: Mat4F = view * model;

        self.material_buffer.write(&MaterialInfo { Kd: [0.4, 0.4, 0.4] });

        let uniforms = uniform! {
            MaterialInfo: &self.material_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        let plane = &self.plane;
        self.deferred_fbo.rent_mut(|(framebuffer, _)| {
            // TODO: handle unwrap()
            plane.render(framebuffer, program_pass1, &draw_params, &uniforms).unwrap();
        });
        // ------------------------------------------------------------------------- 

        // Render Torus ------------------------------------------------------------
        let model = Mat4F::rotation_x(90.0_f32.to_radians())
            .translated_3d(Vec3F::new(1.0, 1.0, 3.0));
        let mv: Mat4F = view * model;

        self.material_buffer.write(&MaterialInfo { Kd: [0.9, 0.5, 0.2] });

        let uniforms = uniform! {
            MaterialInfo: &self.material_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        let torus = &self.torus;
        self.deferred_fbo.rent_mut(|(framebuffer, _)| {
            // TODO: handle unwrap()
            torus.render(framebuffer, program_pass1, &draw_params, &uniforms).unwrap();
        });
        // ------------------------------------------------------------------------- 

        // Write the material one more time
        // It seems that the data of this buffer are not updated on macOS sometimes.
        self.material_buffer.write(&MaterialInfo { Kd: [0.9, 0.9, 0.9] });

        Ok(())
    }

    fn pass2(&self, frame: &mut glium::Frame) -> GLResult<()> {

        frame.clear_color(0.5, 0.5, 0.5, 1.0);
        frame.clear_depth(1.0);

        self.deferred_fbo.rent(|(_, attachment)| {

            let uniforms = uniform! {
                LightInfo: &self.light_buffer,
                PositionTex: attachment.position.sampled()
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                NormalTex: attachment.normal.sampled()
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                ColorTex: attachment.color.sampled()
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            };

            // TODO: handle unwrap()
            self.fs_quad.render(frame, &self.programs[1], &Default::default(), &uniforms).unwrap();
        });

        Ok(())
    }
}
