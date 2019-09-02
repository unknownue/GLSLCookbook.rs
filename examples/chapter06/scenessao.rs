
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{Plane, Quad, ObjMesh, ObjMeshConfiguration};
use cookbook::{Mat4F, Mat3F, Vec3F, Vec4F};
use cookbook::texture::{load_texture, load_custom_texture};
use cookbook::framebuffer::{DeferredPNCAttachment, ColorAttachment, GLDeferredFrameBuffer, GLFrameBuffer};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::texture::{UncompressedFloatFormat, MipmapsOption};
use glium::texture::texture2d::Texture2d;
use glium::{Surface, uniform, implement_uniform_block};


const KERNEL_SIZE: usize = 64;
type KernalBlock = [[f32; 4]; KERNEL_SIZE];

pub struct SceneSsao {

    programs: [glium::Program; 4],

    bunny  : ObjMesh,
    plane  : Plane,
    quad   : Quad,

    deferred_fbo: GLDeferredFrameBuffer::<DeferredPNCAttachment>,
    ssao_fbo1: GLFrameBuffer<ColorAttachment>,
    ssao_fbo2: GLFrameBuffer<ColorAttachment>,

    wood_tex : Texture2d,
    brick_tex: Texture2d,
    rand_tex : Texture2d,

    kernel_buffer   : UniformBuffer<KernalBlock>,
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
    L : [f32; 3], _padding1: f32,
    La: [f32; 3], _padding2: f32,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct MaterialInfo {
    Kd: [f32; 3],
    UseTex: bool,
}


impl Scene for SceneSsao {

    fn new(display: &impl Facade) -> GLResult<SceneSsao> {

        let (screen_width, screen_height) = display.get_context().get_framebuffer_dimensions();

        // Shader Program ------------------------------------------------------------
        let programs = SceneSsao::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let bunny = ObjMesh::load(display, "media/dragon.obj", ObjMeshConfiguration {
            is_with_adjacency: false,
            is_gen_tangents: false,
            is_center: true,
            is_print_load_message: true,
        })?;
        let plane = Plane::new(display, 10.0, 10.0, 1, 1, 10.0, 7.0)?;
        let quad = Quad::new(display)?;
        // ----------------------------------------------------------------------------

        // Initialize textures --------------------------------------------------------
        let wood_tex  = load_texture(display, "media/texture/hardwood2_diffuse.png")?;
        let brick_tex = load_texture(display, "media/texture/brick1.png")?;
        let rand_tex  = SceneSsao::build_rand_rotation_texture(display)?;
        // ---------------------------------------------------------------------------

        // Initialize FrameBuffer Objects ---------------------------------------------
        let deferred_fbo = GLDeferredFrameBuffer::setup(display, screen_width, screen_height)?;
        let ssao_fbo1 = GLFrameBuffer::setup(display, screen_width, screen_height, UncompressedFloatFormat::F16)?;
        let ssao_fbo2 = GLFrameBuffer::setup(display, screen_width, screen_height, UncompressedFloatFormat::F16)?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let projection = Mat4F::identity();
        let angle = std::f32::consts::PI / 2.0;
        let is_animate = true;
        // ----------------------------------------------------------------------------


        // Initialize Uniforms --------------------------------------------------------
        glium::implement_uniform_block!(LightInfo, LightPosition, L, La);
        let light_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;

        glium::implement_uniform_block!(MaterialInfo, Kd, UseTex);
        let material_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;

        // Create and assign the random sample kernel
        let kern = build_kernel();
        let kernel_buffer = UniformBuffer::immutable(display, kern)
            .map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------

        let scene = SceneSsao {
            programs,
            ssao_fbo1, ssao_fbo2, deferred_fbo,
            bunny, plane, quad,
            wood_tex, brick_tex, rand_tex,
            material_buffer, light_buffer, kernel_buffer,
            angle, is_animate, projection,
        };
        Ok(scene)
    }

    fn update(&mut self, delta_time: f32) {
        const TWO_PI: f32 = std::f32::consts::PI * 2.0;
        const ROTATE_SPEED: f32 = 1.0;

        if self.is_animating() {
            self.angle = (self.angle + delta_time * ROTATE_SPEED) % TWO_PI;
        }
    }

    fn render(&mut self, frame: &mut glium::Frame) -> GLResult<()> {

        self.pass1()?;    // Render to G-Buffers
        self.pass2()?;    // SSAO
        self.pass3()?;    // Blur
        self.pass4(frame) // Lighting
    }

    fn resize(&mut self, display: &impl Facade, width: u32, height: u32) -> GLResult<()> {
        self.deferred_fbo = GLDeferredFrameBuffer::setup(display, width, height)?;
        self.ssao_fbo1 = GLFrameBuffer::setup(display, width, height, UncompressedFloatFormat::F16)?;
        self.ssao_fbo2 = GLFrameBuffer::setup(display, width, height, UncompressedFloatFormat::F16)?;
        self.projection   = Mat4F::perspective_rh_zo(50.0_f32.to_radians(), width as f32 / height as f32, 0.3, 100.0);
        Ok(())
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }

    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate
    }
}


impl SceneSsao {

    fn compile_shader_program(display: &impl Facade) -> Result<[Program; 4], ProgramCreationError> {

        let pass1_vertex   = include_str!("shaders/ssao/pass1.vert.glsl");
        let pass1_fragment = include_str!("shaders/ssao/pass1.frag.glsl");

        let pass2_vertex   = include_str!("shaders/ssao/pass2.vert.glsl");
        let pass2_fragment = include_str!("shaders/ssao/pass2.frag.glsl");

        let pass3_vertex   = include_str!("shaders/ssao/pass3.vert.glsl");
        let pass3_fragment = include_str!("shaders/ssao/pass3.frag.glsl");

        let pass4_vertex   = include_str!("shaders/ssao/pass4.vert.glsl");
        let pass4_fragment = include_str!("shaders/ssao/pass4.frag.glsl");

        let pass1 = glium::Program::new(display, GLSourceCode::new(pass1_vertex, pass1_fragment).with_srgb_output(false))?;
        let pass2 = glium::Program::new(display, GLSourceCode::new(pass2_vertex, pass2_fragment).with_srgb_output(false))?;
        let pass3 = glium::Program::new(display, GLSourceCode::new(pass3_vertex, pass3_fragment).with_srgb_output(false))?;
        let pass4 = glium::Program::new(display, GLSourceCode::new(pass4_vertex, pass4_fragment).with_srgb_output(true))?;
        Ok([pass1, pass2, pass3, pass4])
    }

    fn build_rand_rotation_texture(display: &impl Facade) -> GLResult<Texture2d> {

        const SIZE: usize = 4;

        let mut rng = rand::thread_rng();
        let mut rand_directions = Vec::with_capacity(3 * SIZE * SIZE);
        for _ in 0..(SIZE * SIZE) {
            let v = cookbook::random::uniform_circle(&mut rng);
            rand_directions.push(v.x);
            rand_directions.push(v.y);
            rand_directions.push(v.z);
        }

        load_custom_texture(display, rand_directions, SIZE, SIZE, MipmapsOption::NoMipmap, UncompressedFloatFormat::F16F16F16)
    }

    fn pass1(&mut self) -> GLResult<()> {

        let program = &self.programs[0];
        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let view = Mat4F::look_at_rh(Vec3F::new(2.1, 1.5, 2.1), Vec3F::unit_y(), Vec3F::unit_y());

        self.light_buffer.write(&LightInfo {
            LightPosition: (view * Vec4F::new(3.0, 3.0, 1.5, 1.0)).into_array(),
            L: [0.3, 0.3, 0.3],
            La: [0.5, 0.5, 0.5], ..Default::default()
        });

        // Render Walls ----------------------------------------------------------
        // Plane1
        self.material_buffer.write(&MaterialInfo { UseTex: true, ..Default::default() });
        let plane = &self.plane;

        let model = Mat4F::identity();
        let mv: Mat4F = view * model;

        let uniforms = uniform! {
            MaterialInfo: &self.material_buffer,
            DiffTex: self.wood_tex.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.deferred_fbo.rent_mut(|(framebuffer, _)| -> GLResult<()> {

            framebuffer.clear_color(0.0, 0.0, 0.0, 1.0);
            framebuffer.clear_depth(1.0);

            plane.render(framebuffer, program, &draw_params, &uniforms)
        })?;
        // ------------------------------------------------------------------------- 

        // Plane2 ------------------------------------------------------------
        let model = Mat4F::rotation_x(90.0_f32.to_radians())
            .translated_3d(Vec3F::new(0.0, 0.0, -2.0));
        let mv: Mat4F = view * model;

        let uniforms = uniform! {
            MaterialInfo: &self.material_buffer,
            DiffTex: self.brick_tex.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.deferred_fbo.rent_mut(|(framebuffer, _)| -> GLResult<()> {
            plane.render(framebuffer, program, &draw_params, &uniforms)
        })?;
        // ------------------------------------------------------------------------- 

        // Render Plane3 ------------------------------------------------------------
        let model = Mat4F::rotation_x(90.0_f32.to_radians())
            .rotated_y(90.0_f32.to_radians())
            .translated_3d(Vec3F::new(-2.0, 0.0, 0.0));
        let mv: Mat4F = view * model;

        let uniforms = uniform! {
            MaterialInfo: &self.material_buffer,
            DiffTex: self.brick_tex.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.deferred_fbo.rent_mut(|(framebuffer, _)| -> GLResult<()> {
            plane.render(framebuffer, program, &draw_params, &uniforms)
        })?;
        // ------------------------------------------------------------------------- 

        // Render Mesh -------------------------------------------------------------
        self.material_buffer.write(&MaterialInfo {
            Kd: [0.9, 0.5, 0.2],
            UseTex: false,
        });

        let model = Mat4F::translation_3d(Vec3F::new(0.0, 0.282958, 0.0))
            .scaled_3d(Vec3F::new(2.0, 2.0, 2.0))
            .rotated_y(135.0_f32.to_radians());
        let mv: Mat4F = view * model;

        let uniforms = uniform! {
            MaterialInfo: &self.material_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        let bunny = &self.bunny;
        self.deferred_fbo.rent_mut(|(framebuffer, _)| -> GLResult<()> {
            bunny.render(framebuffer, program, &draw_params, &uniforms)
        })
        // ------------------------------------------------------------------------- 
    }

    fn pass2(&mut self) -> GLResult<()> {

        let deferred_fbo = &self.deferred_fbo;
        let projection = self.projection.into_col_arrays();
        let kernel_buffer = &self.kernel_buffer;
        let rand_tex = &self.rand_tex;
        let quad = &self.quad;
        let program = &self.programs[1];

        self.ssao_fbo1.rent_mut(|(framebuffer, _)| -> GLResult<()> {

            framebuffer.clear_color(0.0, 0.0, 0.0, 1.0);

            deferred_fbo.rent(|(_, attachment)| -> GLResult<()> {

                let uniforms = uniform! {
                    ProjectionMatrix: projection,
                    SampleKernel: kernel_buffer,
                    PositionTex: attachment.position.sampled()
                        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                    NormalTex: attachment.normal.sampled()
                        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                    RandTex: rand_tex.sampled()
                        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                };

                quad.render(framebuffer, program, &Default::default(), &uniforms)
            })
        })
    }

    fn pass3(&mut self) -> GLResult<()> {

        let ao_tex = &self.ssao_fbo1;
        let quad = &self.quad;
        let program = &self.programs[2];

        self.ssao_fbo2.rent_mut(|(framebuffer, _)| -> GLResult<()> {

            framebuffer.clear_color(0.0, 0.0, 0.0, 1.0);

            ao_tex.rent(|(_, attachment)| -> GLResult<()> {

                let uniforms = uniform! {
                    RandTex: attachment.color.sampled()
                        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                };

                quad.render(framebuffer, program, &Default::default(), &uniforms)
            })
        })
    }

    fn pass4(&self, frame: &mut glium::Frame) -> GLResult<()> {

        let deferred_fbo = &self.deferred_fbo;
        let ssao_fbo2 = &self.ssao_fbo2;

        frame.clear_color(0.5, 0.5, 0.5, 1.0);

        deferred_fbo.rent(|(_, deferred_attachment)| -> GLResult<()> {

            ssao_fbo2.rent(|(_, ao_attachment)| -> GLResult<()> {

                let uniforms = uniform! {
                    LightInfo: &self.light_buffer,
                    PositionTex: deferred_attachment.position.sampled()
                        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                    NormalTex: deferred_attachment.normal.sampled()
                        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                    ColorTex: deferred_attachment.color.sampled()
                        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                    AoTex: ao_attachment.color.sampled()
                        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                };

                self.quad.render(frame, &self.programs[3], &Default::default(), &uniforms)
            })
        })
    }
}


fn build_kernel() -> KernalBlock {

    let mut rng = rand::thread_rng();
    let mut kern: KernalBlock = [[0.0; 4]; KERNEL_SIZE];
    for i in 0..KERNEL_SIZE {
        let mut rand_dir = cookbook::random::uniform_hemisphere(&mut rng);
        let scale = (i as f32).powi(2) / (KERNEL_SIZE as f32).powi(2);
        rand_dir = rand_dir * (0.1 * (1.0 - scale) + 1.0 * scale);

        kern[i] = [rand_dir.x, rand_dir.y, rand_dir.z, 0.0];
    }

    kern
}
