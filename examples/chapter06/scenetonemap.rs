
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{Teapot, Plane, Sphere, Quad};
use cookbook::{Mat4F, Mat3F, Vec3F, Vec4F};
use cookbook::framebuffer::{ColorDepthAttachment, GLFrameBuffer};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::texture::UncompressedFloatFormat;
use glium::{Surface, uniform, implement_uniform_block};


pub struct SceneToneMap {

    programs: [glium::Program; 2],

    teapot: Teapot,
    plane: Plane,
    sphere: Sphere,
    quad: Quad,

    hdr_fbo: GLFrameBuffer::<ColorDepthAttachment>,

    material_buffer: UniformBuffer<MaterialInfo>,
    light_buffer: UniformBuffer<[LightInfo; 5]>,

    ave_lum: f32,
    screen_width : u32,
    screen_height: u32,

    aspect_ratio: f32,
    view: Mat4F,
    projection: Mat4F,
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
        let programs = SceneToneMap::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let teapot = Teapot::new(display, 14, Mat4F::identity())?;
        let plane = Plane::new(display, 20.0, 10.0, 1, 1, 1.0, 1.0)?;
        let sphere = Sphere::new(display, 2.0, 50, 50)?;
        let quad = Quad::new(display)?;
        // ----------------------------------------------------------------------------

        // Initialize FrameBuffer Objects ---------------------------------------------
        let hdr_fbo = GLFrameBuffer::setup(display, screen_width, screen_height, UncompressedFloatFormat::F32F32F32)?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let view = Mat4F::look_at_rh(Vec3F::new(2.0, 0.0, 14.0), Vec3F::zero(), Vec3F::unit_y());
        let projection = Mat4F::identity();
        let ave_lum = 0.0;
        // ----------------------------------------------------------------------------


        // Initialize Uniforms --------------------------------------------------------
        glium::implement_uniform_block!(LightInfo, Position, L, La);
        let light_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;

        glium::implement_uniform_block!(MaterialInfo, Ka, Kd, Ks, Shininess);
        let material_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------

        let scene = SceneToneMap {
            programs, hdr_fbo,
            teapot, sphere, plane, quad,
            material_buffer, light_buffer,
            screen_width, screen_height,
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

    fn resize(&mut self, display: &impl Facade, width: u32, height: u32) -> GLResult<()> {
        self.aspect_ratio = width as f32 / height as f32;
        self.hdr_fbo = GLFrameBuffer::setup(display, width, height, UncompressedFloatFormat::F32F32F32)?;
        self.projection = Mat4F::perspective_rh_zo(60.0_f32.to_radians(), self.aspect_ratio, 0.3, 100.0);
        self.screen_width  = width;
        self.screen_height = height;
        Ok(())
    }

    fn is_animating(&self) -> bool { false }
    fn toggle_animation(&mut self) {}
}


impl SceneToneMap {

    fn compile_shader_program(display: &impl Facade) -> Result<[Program; 2], ProgramCreationError> {

        let pass1_vertex   = include_str!("shaders/tonemap/pass1.vert.glsl");
        let pass1_fragment = include_str!("shaders/tonemap/pass1.frag.glsl");

        let pass2_vertex   = include_str!("shaders/tonemap/pass2.vert.glsl");
        let pass2_fragment = include_str!("shaders/tonemap/pass2.frag.glsl");

        let pass1 = glium::Program::new(display, GLSourceCode::new(pass1_vertex, pass1_fragment).with_srgb_output(false))?;
        let pass2 = glium::Program::new(display, GLSourceCode::new(pass2_vertex, pass2_fragment).with_srgb_output(true))?;
        Ok([pass1, pass2])
    }

    fn pass1(&mut self, draw_params: &glium::DrawParameters) -> GLResult<()> {

        let program = &self.programs[0];

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
        self.light_buffer.write(&light_data);

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
            LightBlock: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        let plane = &self.plane;
        self.hdr_fbo.rent_mut(|(framebuffer, _)| -> GLResult<()> {

            framebuffer.clear_color(0.5, 0.5, 0.5, 1.0);
            framebuffer.clear_depth(1.0);

            plane.render(framebuffer, program, draw_params, &uniforms)
        })?;
        // ------------------------------------------------------------------------- 

        // Render bottom plane -----------------------------------------------------
        let model = Mat4F::translation_3d(Vec3F::new(0.0, -5.0, 0.0));
        let mv: Mat4F = self.view * model;

        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.hdr_fbo.rent_mut(|(framebuffer, _)| -> GLResult<()> {
            plane.render(framebuffer, program, draw_params, &uniforms)
        })?;
        // ------------------------------------------------------------------------- 

        // Render top plane --------------------------------------------------------
        let model = Mat4F::rotation_x(180.0_f32.to_radians())
            .translated_3d(Vec3F::new(0.0, 5.0, 0.0));
        let mv: Mat4F = self.view * model;

        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.hdr_fbo.rent_mut(|(framebuffer, _)| -> GLResult<()> {
            plane.render(framebuffer, program, draw_params, &uniforms)
        })?;
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
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        let sphere = &self.sphere;
        self.hdr_fbo.rent_mut(|(framebuffer, _)| -> GLResult<()> {
            sphere.render(framebuffer, program, draw_params, &uniforms)
        })?;
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
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        let teapot = &self.teapot;
        self.hdr_fbo.rent_mut(|(framebuffer, _)| -> GLResult<()> {
            teapot.render(framebuffer, program, draw_params, &uniforms)
        })
        // ------------------------------------------------------------------------- 
    }

    fn compute_log_ave_luminance(&mut self) {

        // For accurate estimation, we must calculate from `hdr_texture` every frame. -----------------
        // This process is very very slow on CPU.
        // let mut sum = 0.0;

        // self.hdr_fbo.rent(|(_, attachment)| {
        //     unsafe {
        //         let pixels: glium::texture::pixel_buffer::PixelBuffer<(f32, f32, f32, f32)> = attachment.color.unchecked_read_to_pixel_buffer();
        //         let pixels: Vec<(f32, f32, f32, f32)> = pixels.read_as_texture_1d()
        //             .expect("Failed to read as texture 1d");

        //         for pixel in pixels {
        //             let lum = pixel.0 * 0.2126 + pixel.1 * 0.7152 + pixel.2 * 0.0722;
        //             sum += (lum + 0.00001).ln();
        //         }
        //     }
        // });

        // self.ave_lum = (sum / (self.screen_width as f32 * self.screen_height as f32)).exp();
        // ------------------------------------------------------------------------------------------

        // For static scene. Just hard code this value may reduce the above heavy computation. ------
        self.ave_lum = 0.581015;
        // ------------------------------------------------------------------------------------------
    }

    fn pass2(&self, frame: &mut glium::Frame, draw_params: &glium::DrawParameters) -> GLResult<()> {

        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame.clear_depth(1.0);

        self.hdr_fbo.rent(|(_, attachment)| -> GLResult<()> {

            let uniforms = uniform! {
                AveLum: self.ave_lum,
                HdrTex: attachment.color.sampled()
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            };

            self.quad.render(frame, &self.programs[1], draw_params, &uniforms)
        })
    }
}
