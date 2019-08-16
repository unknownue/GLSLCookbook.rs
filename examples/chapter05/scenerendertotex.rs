
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{Cube, ObjMesh, ObjMeshConfiguration};
use cookbook::texture::load_texture;
use cookbook::{Mat4F, Mat3F, Vec3F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::texture::texture2d::Texture2d;
use glium::framebuffer::{SimpleFrameBuffer, DepthRenderBuffer};
use glium::{Surface, uniform, implement_uniform_block};

// Note: Since glium::framebuffer::SimpleFrameBuffer is need for Texture Rendering, but contains referential member,
//     there rental crate is used to avoid the self-reference conflit in Rust.
//     It makes the code more uglier, but it works.
// See https://github.com/glium/glium/blob/master/examples/deferred.rs for an example of this use case.


pub struct SceneRenderToTex {

    program: glium::Program,

    cube: Cube,
    spot: ObjMesh,

    spot_texture: Texture2d,

    fbo: fbo_rentals::FBORental,

    material_buffer: UniformBuffer<MaterialInfo>,
    light_buffer   : UniformBuffer<LightInfo>,

    projection : Mat4F,

    angle: f32,
    is_animate: bool
}

pub struct FBOResource {
    render_tex: Texture2d,
    depth_buffer: DepthRenderBuffer,
}

rental! {
    mod fbo_rentals {

        #[rental]
        pub struct FBORental {
            res: Box<super::FBOResource>,
            framebuffer: (glium::framebuffer::SimpleFrameBuffer<'res>, &'res super::FBOResource),
        }
    }
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


impl Scene for SceneRenderToTex {

    fn new(display: &impl Facade) -> GLResult<SceneRenderToTex> {

        // Shader Program ------------------------------------------------------------
        let program = SceneRenderToTex::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let cube = Cube::new(display, 1.0)?;
        let spot = ObjMesh::load(display, "media/spot/spot_triangulated.obj", ObjMeshConfiguration {
            is_with_adjacency: false,
            is_gen_tangents: false,
            is_center: false,
            is_print_load_message: true,
        })?;
        // ----------------------------------------------------------------------------

        // Initialize Textures --------------------------------------------------------
        let spot_texture = load_texture(display, "media/spot/spot_texture.png")?;
        // ----------------------------------------------------------------------------

        // Initialize FrameBuffer Objects ---------------------------------------------
        let fbo = SceneRenderToTex::setup_frame_buffer_object(display)?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let projection = Mat4F::identity();
        let angle: f32 = 140.0_f32.to_radians();
        let is_animate = true;
        // ----------------------------------------------------------------------------


        // Initialize Uniforms --------------------------------------------------------
        glium::implement_uniform_block!(LightInfo, LightPosition, L, La);
        let light_buffer = UniformBuffer::immutable(display, LightInfo {
            LightPosition: [0.0_f32, 0.0, 0.0, 1.0],
            L: [1.0_f32, 1.0, 1.0],
            La: [0.15_f32, 0.15, 0.15], ..Default::default()
        }).map_err(BufferCreationErrorKind::UniformBlock)?;

        glium::implement_uniform_block!(MaterialInfo, Ks, Shininess);
        let material_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------

        let scene = SceneRenderToTex {
            program,
            cube, spot, spot_texture, fbo,
            material_buffer, light_buffer,
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

        self.render_to_texture()?;
        self.render_scene(frame)
    }

    fn resize(&mut self, width: u32, height: u32) {

        self.projection = Mat4F::perspective_rh_zo(45.0_f32.to_radians(), width as f32 / height as f32, 0.3, 100.0);
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }

    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate;
    }
}


impl SceneRenderToTex {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/rendertotex.vert.glsl");
        let fragment_shader_code = include_str!("shaders/rendertotex.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }

    fn setup_frame_buffer_object(display: &impl Facade) -> GLResult<fbo_rentals::FBORental> {

        let render_tex = Texture2d::empty(display, 512, 512)
            .map_err(GLErrorKind::CreateTexture)?;
        let depth_buffer = DepthRenderBuffer::new(display, glium::texture::DepthFormat::F32, 512, 512)
            .map_err(BufferCreationErrorKind::RenderBuffer)?;

        // Build the self-referential struct using rental crate.
        let fbo = fbo_rentals::FBORental::new(
            Box::new(FBOResource { render_tex, depth_buffer }),
            // TODO: handle unwrap()
            |res| { 
                let framebuffer = SimpleFrameBuffer::with_depth_buffer(display, &res.render_tex, &res.depth_buffer).unwrap();
                (framebuffer, &res)
            }
        );
        Ok(fbo)
    }

    fn render_to_texture(&mut self) -> GLResult<()> {

        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            viewport: Some(glium::Rect { left: 0, bottom: 0, width: 512, height: 512 }),
            ..Default::default()
        };

        self.material_buffer.write(&MaterialInfo {
            Ks: [0.95, 0.95, 0.95],
            Shininess: 100.0,
        });

        // Render Spot ----------------------------------------------------------
        let view = Mat4F::look_at_rh(Vec3F::new(0.0, 0.0, 2.5), Vec3F::zero(), Vec3F::unit_y());
        let projection = Mat4F::perspective_rh_zo(50.0_f32.to_radians(), 1.0, 0.3, 100.0);
        let model = Mat4F::rotation_y(self.angle);

        let mv: Mat4F = view * model;
        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            RenderTex: self.spot_texture.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Linear)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (projection * mv).into_col_arrays(),
        };

        // Avoid use self in the following closure.
        let spot    = &self.spot;
        let program = &self.program;

        self.fbo.rent_mut(|(framebuffer, _)| {

            framebuffer.clear_color(0.5, 0.5, 0.5, 1.0);
            framebuffer.clear_depth(1.0);
            // TODO: handle unwrap()
            spot.render(framebuffer, program, &draw_params, &uniforms).unwrap();
        });

        Ok(())
        // ------------------------------------------------------------------------- 
    }

    fn render_scene(&self, frame: &mut glium::Frame) -> GLResult<()> {

        frame.clear_color(0.5, 0.5, 0.5, 1.0);
        frame.clear_depth(1.0);

        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        self.material_buffer.write(&MaterialInfo {
            Ks: [0.0, 0.0, 0.0],
            Shininess: 1.0,
        }); 

        // Render Cube ----------------------------------------------------------
        let camera_pos = Vec3F::new(2.0 * self.angle.cos(), 1.5, 2.0 * self.angle.sin());
        let view = Mat4F::look_at_rh(camera_pos, Vec3F::zero(), Vec3F::unit_y());
        let model = Mat4F::identity();
        let mv: Mat4F = view * model;

        self.fbo.rent(|(_, res)| {

            let uniforms = uniform! {
                LightInfo: &self.light_buffer,
                MaterialInfo: &self.material_buffer,
                RenderTex: res.render_tex.sampled()
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Linear)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
                ModelViewMatrix: mv.clone().into_col_arrays(),
                NormalMatrix: Mat3F::from(mv).into_col_arrays(),
                MVP: (self.projection * mv).into_col_arrays(),
            };

            // TODO: handle unwrap()
            self.cube.render(frame, &self.program, &draw_params, &uniforms).unwrap();
        });

        Ok(())
        // ------------------------------------------------------------------------- 
    }
}
