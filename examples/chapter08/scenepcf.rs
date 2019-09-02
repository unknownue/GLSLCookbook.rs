
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{Plane, Frustum, ObjMesh, ObjMeshConfiguration};
use cookbook::{Mat4F, Mat3F, Vec3F, Vec4F};
use cookbook::framebuffer::{ShadowDepthAttachment, GLFrameBuffer};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::texture::DepthFormat;
use glium::{Surface, uniform, implement_uniform_block};


pub struct ScenePcf {

    programs: [glium::Program; 2],

    building: ObjMesh,
    frustum: Frustum,
    plane: Plane,

    shadow_fbo: GLFrameBuffer<ShadowDepthAttachment>,

    material_buffer : UniformBuffer<MaterialInfo>,
    light_buffer    : UniformBuffer<LightInfo>,

    angle: f32,
    is_animate: bool,
    aspect_ratio: f32,

    light_pv: Mat4F,
    view: Mat4F,
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
    Ka: [f32; 3], _padding1: f32,
    Kd: [f32; 3], _padding2: f32,
    Ks: [f32; 3],
    Shininess: f32,
}


impl Scene for ScenePcf {

    fn new(display: &impl Facade) -> GLResult<ScenePcf> {

        // Shader Program ------------------------------------------------------------
        let programs = ScenePcf::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let plane = Plane::new(display, 40.0, 40.0, 2, 2, 1.0, 1.0)?;
        let building = ObjMesh::load(display, "media/building.obj", ObjMeshConfiguration {
            is_with_adjacency: false,
            is_gen_tangents: false,
            is_center: false,
            is_print_load_message: true,
        })?;
        let mut frustum = Frustum::new(display)?;
        // ----------------------------------------------------------------------------

        // Initialize FrameBuffer Objects ---------------------------------------------
        let shadow_fbo = GLFrameBuffer::setup_depth(display, 512, 512, DepthFormat::I24)?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let shadow_scale = Mat4F::new(
            0.5, 0.0, 0.0, 0.0,
            0.0, 0.5, 0.0, 0.0,
            0.0, 0.0, 0.5, 0.0,
            0.5, 0.5, 0.5, 1.0,
        );
        let light_pos = Vec3F::new(-2.5, -2.0, -2.5); // World coords
        frustum.orient(light_pos, Vec3F::zero(), Vec3F::unit_y());
        frustum.set_perspective(40.0, 1.0, 0.1, 100.0);

        let light_pv = shadow_scale * frustum.get_projection_matrix() * frustum.get_view_matrix();
        
        let view = Mat4F::identity();
        let projection = Mat4F::identity();
        let angle = std::f32::consts::PI * 0.85;
        let is_animate = true;
        let aspect_ratio = 0.0;
        // ----------------------------------------------------------------------------


        // Initialize Uniforms --------------------------------------------------------
        glium::implement_uniform_block!(LightInfo, LightPosition, Intensity);
        let light_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;

        glium::implement_uniform_block!(MaterialInfo, Ka, Kd, Ks, Shininess);
        let material_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------

        let scene = ScenePcf {
            programs, shadow_fbo,
            plane, building, frustum,
            material_buffer, light_buffer,
            angle, is_animate, aspect_ratio,
            light_pv, view, projection,
        };
        Ok(scene)
    }

    fn update(&mut self, delta_time: f32) {
        const TWO_PI: f32 = std::f32::consts::PI * 2.0;
        const ROTATE_SPEED: f32 = 0.2;

        if self.is_animating() {
            self.angle = (self.angle + delta_time * ROTATE_SPEED) % TWO_PI;
        }
    }

    fn render(&mut self, frame: &mut glium::Frame) -> GLResult<()> {

        // Pass 1 (shadow map generation)
        self.pass1()?;
        // Pass 2 (render)
        self.pass2(frame)
    }

    fn resize(&mut self, display: &impl Facade, width: u32, height: u32) -> GLResult<()> {
        self.shadow_fbo = GLFrameBuffer::setup_depth(display, 512, 512, DepthFormat::I24)?;
        self.aspect_ratio = width as f32 / height as f32;
        Ok(())
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }

    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate
    }
}


impl ScenePcf {

    fn compile_shader_program(display: &impl Facade) -> Result<[Program; 2], ProgramCreationError> {

        let pass1_vertex   = include_str!("shaders/pcf/pass1.vert.glsl");
        let pass1_fragment = include_str!("shaders/pcf/pass1.frag.glsl");

        let pass2_vertex   = include_str!("shaders/pcf/pass2.vert.glsl");
        let pass2_fragment = include_str!("shaders/pcf/pass2.frag.glsl");

        let pass1 = glium::Program::new(display, GLSourceCode::new(pass1_vertex, pass1_fragment).with_srgb_output(false))?;
        let pass2 = glium::Program::new(display, GLSourceCode::new(pass2_vertex, pass2_fragment).with_srgb_output(true))?;
        Ok([pass1, pass2])
    }

    fn pass1(&mut self) -> GLResult<()> {

        self.view       = self.frustum.get_view_matrix();
        self.projection = self.frustum.get_projection_matrix();

        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };
        
        // See https://github.com/glium/glium/issues/826
        // glPolygonOffset(2.5, 10.0); is not implemented.

        self.draw_scene_pass1(&draw_params)?;

        // self.spit_out_depth_buffer(); // This is just used to get an image of the depth buffer
        
        Ok(())
    }

     fn pass2(&mut self, frame: &mut glium::Frame) -> GLResult<()> {

        let camera_pos = Vec3F::new(1.8 * self.angle.cos(), 7.0, 1.8 * self.angle.sin());
        self.view = Mat4F::look_at_rh(camera_pos, Vec3F::new(0.0, -0.175, 0.0), Vec3F::unit_y());
        self.projection = Mat4F::perspective_rh_zo(50.0_f32.to_radians(), self.aspect_ratio, 0.1, 100.0);

        let frustum_origin = self.frustum.get_origin();
        let light_pos = self.view * Vec4F::new(frustum_origin.x, frustum_origin.y, frustum_origin.z, 1.0);
        self.light_buffer.write(&LightInfo {
            LightPosition: light_pos.into_array(),
            Intensity: [0.85, 0.85, 0.85], ..Default::default()
        });

        frame.clear_color(0.0, 0.5, 0.5, 1.0);
        frame.clear_depth(1.0);

        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        self.draw_scene_pass2(frame, &self.programs[1], &draw_params)
    }

    fn draw_scene_pass1(&mut self, draw_params: &glium::DrawParameters) -> GLResult<()> {

        let program    = &self.programs[0];
        let buidling   = &self.building;
        let plane      = &self.plane;
        let projection = self.projection.clone();
        let view       = self.view.clone();

        self.shadow_fbo.rent_mut(|(framebuffer, _)| -> GLResult<()> {

            let model = Mat4F::identity();

            let uniforms = uniform! {
                MVP: (projection * view * model).into_col_arrays(),
            };

            // Render building --------------------------------------------------------
            framebuffer.clear_depth(1.0);
            buidling.render(framebuffer, program, draw_params, &uniforms)?;
            // ------------------------------------------------------------------------- 

            // Render Plane ------------------------------------------------------------
            plane.render(framebuffer, program, draw_params, &uniforms)
            // ------------------------------------------------------------------------- 
        })
    }

    fn draw_scene_pass2(&self, framebuffer: &mut impl Surface, program: &glium::Program, draw_params: &glium::DrawParameters) -> GLResult<()> {

        self.shadow_fbo.rent(|(_, shadowmap)| -> GLResult<()> {

            // Render building --------------------------------------------------------
            let model = Mat4F::identity();
            let mv: Mat4F = self.view * model;

            self.material_buffer.write(&MaterialInfo {
                Ka: [1.0 * 0.1, 0.85 * 0.1, 0.55 * 0.1],
                Kd: [1.0, 0.85, 0.55],
                Ks: [0.0, 0.0, 0.0],
                Shininess: 1.0, ..Default::default()
            });

            let uniforms = uniform! {
                MaterialInfo: &self.material_buffer,
                LightInfo: &self.light_buffer,
                ShadowMap: shadowmap.depth.sampled()
                    // wrap function should be BorderClamp
                    .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                ModelViewMatrix: mv.clone().into_col_arrays(),
                NormalMatrix: Mat3F::from(mv).into_col_arrays(),
                MVP: (self.projection * mv).into_col_arrays(),
                ShadowMatrix: (self.light_pv * model).into_col_arrays(),
            };

            self.building.render(framebuffer, program, draw_params, &uniforms)?;
            // ------------------------------------------------------------------------- 

            // Render Plane ------------------------------------------------------------
            self.material_buffer.write(&MaterialInfo {
                Ka: [0.05, 0.05, 0.05],
                Kd: [0.25, 0.25, 0.25],
                Ks: [0.0, 0.0, 0.0],
                Shininess: 1.0, ..Default::default()
            });

            let model = Mat4F::identity();
            let mv: Mat4F = self.view * model;

            let uniforms = uniform! {
                MaterialInfo: &self.material_buffer,
                LightInfo: &self.light_buffer,
                ShadowMap: shadowmap.depth.sampled()
                    // wrap function should be BorderClamp
                    .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                ModelViewMatrix: mv.clone().into_col_arrays(),
                NormalMatrix: Mat3F::from(mv).into_col_arrays(),
                MVP: (self.projection * mv).into_col_arrays(),
                ShadowMatrix: (self.light_pv * model).into_col_arrays(),
            };

            self.plane.render(framebuffer, program, draw_params, &uniforms)
            // ------------------------------------------------------------------------- 
        })
    }

    #[allow(dead_code)]
    fn spit_out_depth_buffer(&self) {
        unimplemented!()
    }
}
