
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{Plane, Frustum, ObjMesh, Quad, ObjMeshConfiguration};
use cookbook::{Mat4F, Mat3F, Vec3F, Vec4F};
use cookbook::framebuffer::{ShadowDepthAttachment, GLFrameBuffer};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::texture::DepthFormat;
use glium::{Surface, uniform, implement_uniform_block};


pub struct ScenePcf {

    programs: [glium::Program; 3],

    building: ObjMesh,
    frustum: Frustum,
    plane: Plane,
    quad: Quad,

    shadow_fbo: GLFrameBuffer<ShadowDepthAttachment>,

    material_buffer : UniformBuffer<MaterialInfo>,
    light_buffer    : UniformBuffer<LightInfo>,

    angle: f32,
    is_animate: bool,
    aspect_ratio: f32,

    light_pv: Mat4F,
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
        let plane = Plane::new(display, 20.0, 20.0, 1, 1, 1.0, 1.0)?;
        let building = ObjMesh::load(display, "media/building.obj", ObjMeshConfiguration {
            is_with_adjacency: false,
            is_gen_tangents: false,
            is_center: false,
            is_print_load_message: true,
        })?;
        let mut frustum = Frustum::new(display)?;
        let quad = Quad::new(display)?;
        // ----------------------------------------------------------------------------

        // Initialize FrameBuffer Objects ---------------------------------------------
        let shadow_fbo = GLFrameBuffer::setup_depth(display, 512, 512, DepthFormat::I24)?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let shadow_scale = Mat4F::new(
            0.5, 0.0, 0.0, 0.5,
            0.0, 0.5, 0.0, 0.5,
            0.0, 0.0, 0.5, 0.5,
            0.0, 0.0, 0.0, 1.0,
        );
        let light_pos = Vec3F::new(-2.5, -2.0, -2.5); // World coords
        frustum.orient(light_pos, Vec3F::zero(), Vec3F::unit_y());
        frustum.set_perspective(40.0, 1.0, 0.1, 100.0);

        let light_pv = shadow_scale * frustum.get_projection_matrix() * frustum.get_view_matrix();
        
        let angle = std::f32::consts::PI * 2.0 * 0.85;
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
            plane, building, frustum, quad,
            material_buffer, light_buffer,
            angle, is_animate, aspect_ratio, light_pv,
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
        self.pass2(frame)?;
        // Pass 3 (render shadow map)
        self.pass3(frame)
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

    fn compile_shader_program(display: &impl Facade) -> Result<[Program; 3], ProgramCreationError> {

        let pass1_vertex   = include_str!("shaders/pcf/pass1.vert.glsl");
        let pass1_fragment = include_str!("shaders/pcf/pass1.frag.glsl");

        let pass2_vertex   = include_str!("shaders/pcf/pass2.vert.glsl");
        let pass2_fragment = include_str!("shaders/pcf/pass2.frag.glsl");

        let pass3_vertex   = include_str!("shaders/shadowmap/pass3.vert.glsl");
        let pass3_fragment = include_str!("shaders/shadowmap/pass3.frag.glsl");

        let pass1 = glium::Program::new(display, GLSourceCode::new(pass1_vertex, pass1_fragment).with_srgb_output(false))?;
        let pass2 = glium::Program::new(display, GLSourceCode::new(pass2_vertex, pass2_fragment).with_srgb_output(true))?;
        let pass3 = glium::Program::new(display, GLSourceCode::new(pass3_vertex, pass3_fragment).with_srgb_output(true))?;
        Ok([pass1, pass2, pass3])
    }

    fn pass1(&mut self) -> GLResult<()> {

        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };
        
        // glPolygonOffset(2.5, 10.0); is not support in glium.
        // See https://github.com/glium/glium/issues/826

        self.draw_scene_pass1(&draw_params)?;

        // self.spit_out_depth_buffer(); // This is just used to get an image of the depth buffer
        
        Ok(())
    }

    fn pass2(&mut self, frame: &mut glium::Frame) -> GLResult<()> {

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

        self.draw_scene_pass2(frame, &draw_params)
    }

    fn pass3(&self, frame: &mut glium::Frame) -> GLResult<()> {

        let draw_params = glium::draw_parameters::DrawParameters {
            viewport: Some(glium::Rect {
                left: 0, bottom: 0, width: 512, height: 512,
            }),
            ..Default::default()
        };

        self.shadow_fbo.rent(|(_, shadowmap)| -> GLResult<()> {

            let uniforms = uniform! {
                ShadowTex: shadowmap.depth.sampled()
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            };

            self.quad.render(frame, &self.programs[2], &draw_params, &uniforms)
        })
    }

    fn draw_scene_pass1(&mut self, draw_params: &glium::DrawParameters) -> GLResult<()> {

        let program    = &self.programs[0];
        let buidling   = &self.building;
        let plane      = &self.plane;
        let view       = self.frustum.get_view_matrix();
        let projection = self.frustum.get_projection_matrix();

        self.shadow_fbo.rent_mut(|(framebuffer, _)| -> GLResult<()> {

            let model = Mat4F::identity();

            let uniforms = uniform! {
                MVP: (projection * view * model).into_col_arrays(),
            };
            framebuffer.clear_depth(1.0);

            // Render building --------------------------------------------------------
            buidling.render(framebuffer, program, draw_params, &uniforms)?;
            // ------------------------------------------------------------------------- 

            // Render Plane ------------------------------------------------------------
            plane.render(framebuffer, program, draw_params, &uniforms)
            // ------------------------------------------------------------------------- 
        })
    }

    fn draw_scene_pass2(&self, framebuffer: &mut impl Surface, draw_params: &glium::DrawParameters) -> GLResult<()> {
        
        let camera_pos = Vec3F::new(1.8 * self.angle.cos(), 0.7, 1.8 * self.angle.sin());
        let view = Mat4F::look_at_rh(camera_pos, Vec3F::new(0.0, -0.175, 0.0), Vec3F::unit_y());
        let projection = Mat4F::perspective_rh_zo(50.0_f32.to_radians(), self.aspect_ratio, 0.1, 100.0);

        let frustum_origin = self.frustum.get_origin();
        let light_pos = view * Vec4F::new(frustum_origin.x, frustum_origin.y, frustum_origin.z, 1.0);
        self.light_buffer.write(&LightInfo {
            LightPosition: light_pos.into_array(),
            Intensity: [0.85, 0.85, 0.85], ..Default::default()
        });

        self.shadow_fbo.rent(|(_, shadowmap)| -> GLResult<()> {

            let model = Mat4F::identity();
            let mv: Mat4F = view * model;

            let uniforms = uniform! {
                MaterialInfo: &self.material_buffer,
                LightInfo: &self.light_buffer,
                ShadowMap: shadowmap.depth.sampled()
                    .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                    .depth_texture_comparison(Some(glium::uniforms::DepthTextureComparison::LessOrEqual)),
                ModelViewMatrix: mv.clone().into_col_arrays(),
                NormalMatrix: Mat3F::from(mv).into_col_arrays(),
                MVP: (projection * mv).into_col_arrays(),
                ShadowMatrix: (self.light_pv * model).into_col_arrays(),
            };

            // Render building --------------------------------------------------------
            self.material_buffer.write(&MaterialInfo {
                Ka: [1.0 * 0.1, 0.85 * 0.1, 0.55 * 0.1],
                Kd: [1.0, 0.85, 0.55],
                Ks: [0.0, 0.0, 0.0],
                Shininess: 1.0, ..Default::default()
            });

            self.building.render(framebuffer, &self.programs[1], draw_params, &uniforms)?;
            // ------------------------------------------------------------------------- 

            // Render Plane ------------------------------------------------------------
            self.material_buffer.write(&MaterialInfo {
                Ka: [0.05, 0.05, 0.05],
                Kd: [0.25, 0.25, 0.25],
                Ks: [0.0, 0.0, 0.0],
                Shininess: 1.0, ..Default::default()
            });

            self.plane.render(framebuffer, &self.programs[1], draw_params, &uniforms)
            // ------------------------------------------------------------------------- 
        })
    }

    #[allow(dead_code)]
    fn spit_out_depth_buffer(&self) {
        unimplemented!()
    }
}
