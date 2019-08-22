
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{Teapot, Plane, Torus, Quad};
use cookbook::{Mat4F, Mat3F, Vec3F};
use cookbook::framebuffer::{ColorDepthAttachment, GLFrameBuffer};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::{Surface, uniform, implement_uniform_block};


pub struct SceneEdge {

    program: glium::Program,

    teapot: Teapot,
    plane: Plane,
    torus: Torus,
    fs_quad: Quad,

    fbo: GLFrameBuffer<ColorDepthAttachment>,
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


impl Scene for SceneEdge {

    fn new(display: &impl Facade) -> GLResult<SceneEdge> {

        let (screen_width, screen_height) = display.get_context().get_framebuffer_dimensions();
        let aspect_ratio = (screen_width as f32) / (screen_height as f32);

        // Shader Program ------------------------------------------------------------
        let program = SceneEdge::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
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
        let fbo = GLFrameBuffer::setup(display, screen_width, screen_height)?;

        glium::implement_uniform_block!(LightInfo, LightPosition, L, La);
        let light_buffer = UniformBuffer::immutable(display, LightInfo {
            LightPosition: [0.0_f32, 0.0, 0.0, 1.0],
            L: [1.0_f32, 1.0, 1.0],
            La: [0.2_f32, 0.2, 0.2], ..Default::default()
        }).map_err(BufferCreationErrorKind::UniformBlock)?;

        glium::implement_uniform_block!(MaterialInfo, Ka, Kd, Ks, Shininess);
        let material_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------

        let scene = SceneEdge {
            program, fbo,
            teapot, torus, plane, fs_quad, 
            material_buffer, light_buffer,
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
        self.pass2(frame, &draw_params)
    }

    fn resize(&mut self, display: &impl Facade, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
        self.fbo = GLFrameBuffer::setup(display, width, height).unwrap();
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }
    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate;
    }
}


impl SceneEdge {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/edge.vert.glsl");
        let fragment_shader_code = include_str!("shaders/edge.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }

    fn pass1(&mut self, draw_params: &glium::DrawParameters) -> GLResult<()> {

        let program = &self.program;

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
            Pass: 1_i32,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (projection * mv).into_col_arrays(),
        };

        let teapot = &self.teapot;
        self.fbo.rent_mut(|(framebuffer, _)| {

            framebuffer.clear_color(1.0, 0.0, 0.0, 1.0);
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
            Pass: 1_i32,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (projection * mv).into_col_arrays(),
        };

        let plane = &self.plane;
        self.fbo.rent_mut(|(framebuffer, _)| {
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
            Pass: 1_i32,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (projection * mv).into_col_arrays(),
        };

        let torus = &self.torus;
        self.fbo.rent_mut(|(framebuffer, _)| {
            // TODO: handle unwrap()
            torus.render(framebuffer, program, draw_params, &uniforms).unwrap();
        });
        // ------------------------------------------------------------------------- 
        Ok(())
    }

    fn pass2(&self, frame: &mut glium::Frame, draw_params: &glium::DrawParameters) -> GLResult<()> {

        frame.clear_color(0.5, 0.5, 0.5, 1.0);
        frame.clear_depth(1.0);

        self.fbo.rent(|(_, attachment)| {

            let uniforms = uniform! {
                Pass: 2_i32,
                EdgeThreshold: 0.05_f32,
                RenderTex: attachment.color.sampled()
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                ModelViewMatrix: Mat4F::identity().into_col_arrays(),
                NormalMatrix: Mat3F::identity().into_col_arrays(),
                MVP: Mat4F::identity().into_col_arrays(),
            };

            // TODO: handle unwrap()
            self.fs_quad.render(frame, &self.program, draw_params, &uniforms).unwrap();
        });

        Ok(())
    }
}
