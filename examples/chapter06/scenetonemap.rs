
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{Teapot, Plane, Sphere, Quad};
use cookbook::{Mat4F, Mat3F, Vec3F, Vec4F};
use cookbook::framebuffer::{HdrColorDepthAttachment, GLFrameBuffer};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::{Surface, uniform, implement_uniform_block};


pub struct SceneToneMap {

    program: glium::Program,

    teapot: Teapot,
    plane: Plane,
    sphere: Sphere,
    quad: Quad,

    hdr_fbo: GLFrameBuffer::<HdrColorDepthAttachment>,

    material_buffer: UniformBuffer<MaterialInfo>,
    light_buffer: UniformBuffer<LightsWrapper>,

    ave_lum: f32,
    aspect_ratio: f32,
    view: Mat4F,
    projection: Mat4F,
}


#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct LightsWrapper {
    Lights: [LightInfo; 5],
}


#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct LightInfo {
    Position: [f32; 4],
    L : [f32; 3], _padding1: f32,
    La: [f32; 3], _padding2: f32,
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


impl Scene for SceneToneMap {

    fn new(display: &impl Facade) -> GLResult<SceneToneMap> {

        let (screen_width, screen_height) = display.get_context().get_framebuffer_dimensions();
        let aspect_ratio = (screen_width as f32) / (screen_height as f32);

        // Shader Program ------------------------------------------------------------
        let program = SceneToneMap::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let teapot = Teapot::new(display, 14, Mat4F::identity())?;
        let plane = Plane::new(display, 20.0, 20.0, 1, 1, 1.0, 1.0)?;
        let sphere = Sphere::new(display, 2.0, 50, 50)?;
        let quad = Quad::new(display)?;
        // ----------------------------------------------------------------------------

        // Initialize FrameBuffer Objects ---------------------------------------------
        let hdr_fbo = GLFrameBuffer::setup(display, screen_width, screen_height)?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let view = Mat4F::look_at_rh(Vec3F::new(2.0, 0.0, 14.0), Vec3F::zero(), Vec3F::unit_y());
        let projection = Mat4F::identity();
        let ave_lum = 0.0;
        // ----------------------------------------------------------------------------


        // Initialize Uniforms --------------------------------------------------------
        glium::implement_uniform_block!(LightInfo, Position, L, La);
        glium::implement_uniform_block!(LightsWrapper, Lights);
        let light_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;

        glium::implement_uniform_block!(MaterialInfo, Ka, Kd, Ks, Shininess);
        let material_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------

        let scene = SceneToneMap {
            program, hdr_fbo,
            teapot, sphere, plane, quad,
            material_buffer, light_buffer,
            aspect_ratio, view, projection, ave_lum,
        };
        Ok(scene)
    }

    fn update(&mut self, _delta_time: f32) {
        // Nothing to do, leave it empty...
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
        self.compute_log_ave_luminance();
        self.pass2(frame, &draw_params)
    }

    fn resize(&mut self, display: &impl Facade, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
        self.hdr_fbo = GLFrameBuffer::setup(display, width, height).unwrap();
        self.projection = Mat4F::perspective_rh_zo(60.0_f32.to_radians(), self.aspect_ratio, 0.3, 100.0);
    }

    fn is_animating(&self) -> bool { false }
    fn toggle_animation(&mut self) {}
}


impl SceneToneMap {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/tonemap.vert.glsl");
        let fragment_shader_code = include_str!("shaders/tonemap.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }

    fn pass1(&mut self, draw_params: &glium::DrawParameters) -> GLResult<()> {

        let program = &self.program;

        let light_data = [
            LightInfo {
                Position: (self.view * Vec4F::new(-7.0, 4.0, 2.5, 1.0)).into_array(),
                L:  [1.0, 1.0, 1.0],
                La: [0.2, 0.2, 0.2], ..Default::default()
            },
            LightInfo {
                Position: (self.view * Vec4F::new(0.0, 4.0, 2.5, 1.0)).into_array(),
                L:  [1.0, 1.0, 1.0],
                La: [0.2, 0.2, 0.2], ..Default::default()
            },
            LightInfo {
                Position: (self.view * Vec4F::new(7.0, 4.0, 2.5, 1.0)).into_array(),
                L:  [1.0, 1.0, 1.0],
                La: [0.2, 0.2, 0.2], ..Default::default()
            },
            LightInfo::default(),
            LightInfo::default(),
        ];
        self.light_buffer.write(&LightsWrapper { Lights: light_data });

        self.material_buffer.write(&MaterialInfo {
            Ka: [0.2, 0.2, 0.2],
            Kd: [0.9, 0.3, 0.2],
            Ks: [1.0, 1.0, 1.0],
            Shininess: 100.0, ..Default::default()
        });

        // Render backdrop plane ----------------------------------------------
        let model = Mat4F::rotation_x(90.0_f32.to_radians());
        let mv: Mat4F = self.view * model;

        let uniforms = uniform! {
            LightsWrapper: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            Pass: 1_i32,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        let plane = &self.plane;
        self.hdr_fbo.rent_mut(|(framebuffer, _)| {

            framebuffer.clear_color(0.5, 0.5, 0.5, 1.0);
            framebuffer.clear_depth(1.0);
            // TODO: handle unwrap()
            plane.render(framebuffer, program, draw_params, &uniforms).unwrap();
        });
        // ------------------------------------------------------------------------- 

        // Render bottom plane -----------------------------------------------------
        let model = Mat4F::translation_3d(Vec3F::new(0.0, -5.0, 0.0));
        let mv: Mat4F = self.view * model;

        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            Pass: 1_i32,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.hdr_fbo.rent_mut(|(framebuffer, _)| {
            // TODO: handle unwrap()
            plane.render(framebuffer, program, draw_params, &uniforms).unwrap();
        });
        // ------------------------------------------------------------------------- 

        // Render top plane --------------------------------------------------------
        let model = Mat4F::rotation_x(180.0_f32.to_radians())
            .translated_3d(Vec3F::new(0.0, 5.0, 0.0));
        let mv: Mat4F = self.view * model;

        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            Pass: 1_i32,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.hdr_fbo.rent_mut(|(framebuffer, _)| {
            // TODO: handle unwrap()
            plane.render(framebuffer, program, draw_params, &uniforms).unwrap();
        });
        // ------------------------------------------------------------------------- 

        // Render sphere -----------------------------------------------------------
        self.material_buffer.write(&MaterialInfo {
            Ka: [0.2, 0.2, 0.2],
            Kd: [0.4, 0.9, 0.4],
            Ks: [1.0, 1.0, 1.0],
            Shininess: 100.0, ..Default::default()
        });

        let model = Mat4F::translation_3d(Vec3F::new(-3.0, -3.0, 2.0));
        let mv: Mat4F = self.view * model;

        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            Pass: 1_i32,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        let sphere = &self.sphere;
        self.hdr_fbo.rent_mut(|(framebuffer, _)| {
            // TODO: handle unwrap()
            sphere.render(framebuffer, program, draw_params, &uniforms).unwrap();
        });
        // -----------------------------------------------------------------------

        // Render teapot ---------------------------------------------------------
        self.material_buffer.write(&MaterialInfo {
            Ka: [0.2, 0.2, 0.2],
            Kd: [0.4, 0.4, 0.9],
            Ks: [1.0, 1.0, 1.0],
            Shininess: 100.0, ..Default::default()
        });

        let model = Mat4F::rotation_x(-90_f32.to_radians())
            .translated_3d(Vec3F::new(3.0, -5.0, 1.5));
        let mv: Mat4F = self.view * model;

        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            Pass: 1_i32,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        let teapot = &self.teapot;
        self.hdr_fbo.rent_mut(|(framebuffer, _)| {
            // TODO: handle unwrap()
            teapot.render(framebuffer, program, draw_params, &uniforms).unwrap();
        });
        // ------------------------------------------------------------------------- 
        Ok(())
    }

    fn compute_log_ave_luminance(&mut self) {

        // TODO: Implement read data from texture and calculate `ave_lum`.
        // Original implementation https://github.com/PacktPublishing/OpenGL-4-Shading-Language-Cookbook-Third-Edition/blob/805471f2fa03f6ab18172e707b203c71c1973fd3/chapter06/scenetonemap.cpp#L137.
        
        // glium does not suppot read format `UncompressedFloatFormat::F32F32F32F32` from texture.
        // See https://github.com/glium/glium/issues/1502 for detail.
        // So here we hard code the value.
        self.ave_lum = 0.581015;
    }

    fn pass2(&self, frame: &mut glium::Frame, draw_params: &glium::DrawParameters) -> GLResult<()> {

        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame.clear_depth(1.0);

        self.hdr_fbo.rent(|(_, attachment)| {

            let uniforms = uniform! {
                Pass: 2_i32,
                AveLum: self.ave_lum,
                HdrTex: attachment.color.sampled()
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                ModelViewMatrix: Mat4F::identity().into_col_arrays(),
                NormalMatrix: Mat3F::identity().into_col_arrays(),
                MVP: Mat4F::identity().into_col_arrays(),
            };

            // TODO: handle unwrap()
            self.quad.render(frame, &self.program, draw_params, &uniforms).unwrap();
        });

        Ok(())
    }
}
