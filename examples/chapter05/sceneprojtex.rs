
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{Teapot, Plane};
use cookbook::texture::load_texture;
use cookbook::{Mat4F, Mat3F, Vec3F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::texture::texture2d::Texture2d;
use glium::{Surface, uniform, implement_uniform_block};



#[derive(Debug)]
pub struct SceneProjTex {

    program: glium::Program,

    teapot: Teapot,
    plane: Plane,
    flower_tex: Texture2d,

    material_buffer: UniformBuffer<MaterialInfo>,
    light_buffer   : UniformBuffer<LightInfo>,

    project_matrix: Mat4F,
    projection: Mat4F,

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


impl Scene for SceneProjTex {

    fn new(display: &impl Facade) -> GLResult<SceneProjTex> {

        // Shader Program ------------------------------------------------------------
        let program = SceneProjTex::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let teapot = Teapot::new(display, 14, Mat4F::identity())?;
        let plane = Plane::new(display, 100.0, 100.0, 1, 1, 1.0, 1.0)?;
        // ----------------------------------------------------------------------------

        // Initialize Textures --------------------------------------------------------
        let flower_tex = load_texture(display, "media/texture/flower.png")?;
        // ----------------------------------------------------------------------------


        // Initialize MVP -------------------------------------------------------------
        let projection = Mat4F::identity();
        let angle = 90.0_f32.to_radians();
        let is_animate = true;

        let proj_pos = Vec3F::new(2.0, 5.0, 5.0);
        let proj_at  = Vec3F::new(-2.0, -4.0, 0.0);
        let proj_up  = Vec3F::new(0.0, 1.0, 0.0);
        let proj_view = Mat4F::look_at_rh(proj_pos, proj_at, proj_up);
        let proj_proj = Mat4F::perspective_rh_zo(30.0_f32.to_radians(), 1.0, 0.2, 1000.0);
        let bias = Mat4F::scaling_3d(Vec3F::new(0.5, 0.5, 0.5))
            .translated_3d(Vec3F::new(0.5, 0.5, 0.5));
        let project_matrix = bias * proj_proj * proj_view;
        // ----------------------------------------------------------------------------


        // Initialize Uniforms --------------------------------------------------------
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

        let scene = SceneProjTex {
            program,
            teapot, plane, flower_tex,
            material_buffer, light_buffer,
            projection, project_matrix, angle, is_animate,
        };
        Ok(scene)
    }

    fn update(&mut self, delta_time: f32) {

        const TWO_PI: f32 = std::f32::consts::PI * 2.0;
        const ROTATE_SPEED: f32 = std::f32::consts::PI / 2.0;

        if self.is_animating() {
            self.angle = (self.angle + delta_time * ROTATE_SPEED) % TWO_PI;
        }
    }

    fn render(&self, frame: &mut glium::Frame) -> GLResult<()> {

        frame.clear_color_srgb(0.5, 0.5, 0.5, 1.0);
        frame.clear_depth(1.0);

        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        // Render teapot ----------------------------------------------------------
        let camera_pos = Vec3F::new(7.0 * self.angle.cos(), 2.0, 7.0 * self.angle.sin());
        let view = Mat4F::look_at_rh(camera_pos, Vec3F::zero(), Vec3F::unit_y());
        let model = Mat4F::rotation_x(-90.0_f32.to_radians())
            .translated_3d(Vec3F::new(0.0, -1.0, 0.0));
        let mv: Mat4F = view * model;

        self.material_buffer.write(&MaterialInfo {
            Ka: [0.1, 0.1, 0.1],
            Kd: [0.5, 0.2, 0.1],
            Ks: [0.95, 0.95, 0.95],
            Shininess: 100.0, ..Default::default()
        });

        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            ProjectorTex: self.flower_tex.sampled()
                // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                // Due to limit of glium, currently there is no support for SamplerWrapFunction::Border
                // See https://github.com/glium/glium/issues/1772 for detail.
                .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
            ProjectorMatrix: self.project_matrix.into_col_arrays(),
            ModelMatrix: model.into_col_arrays(),
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.teapot.render(frame, &self.program, &draw_params, &uniforms)?;
        // -------------------------------------------------------------------------

        // Render plane ----------------------------------------------------------
        let model = Mat4F::translation_3d(Vec3F::new(0.0, -0.75, 0.0));
        let mv: Mat4F = view * model;

        self.material_buffer.write(&MaterialInfo {
            Ka: [0.1, 0.1, 0.1],
            Kd: [0.4, 0.4, 0.4],
            Ks: [0.0, 0.0, 0.0],
            Shininess: 1.0, ..Default::default()
        });

        let uniforms = uniform! {
            LightInfo: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            ProjectorTex: self.flower_tex.sampled()
                .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
            ProjectorMatrix: self.project_matrix.into_col_arrays(),
            ModelViewMatrix: mv.clone().into_col_arrays(),
            ModelMatrix: model.into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.plane.render(frame, &self.program, &draw_params, &uniforms)
        // -------------------------------------------------------------------------
    }

    fn resize(&mut self, width: u32, height: u32) {

        self.projection = Mat4F::perspective_rh_zo(50.0_f32.to_radians(), width as f32 / height as f32, 0.3, 100.0);
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }
    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate;
    }
}


impl SceneProjTex {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/projtex.vert.glsl");
        let fragment_shader_code = include_str!("shaders/projtex.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
