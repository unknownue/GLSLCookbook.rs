
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{SkyBox, ObjMesh, ObjMeshConfiguration};
use cookbook::texture::{load_cubemap, load_texture, CubeMapFaceExtension};
use cookbook::{Mat4F, Vec3F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::{UniformBuffer, MagnifySamplerFilter, MinifySamplerFilter};
use glium::texture::cubemap::Cubemap;
use glium::texture::texture2d::Texture2d;
use glium::{Surface, uniform, implement_uniform_block};


#[derive(Debug)]
pub struct SceneDiffIbl {

    program: glium::Program,
    sky_prog: glium::Program,

    spot: ObjMesh,
    skybox: SkyBox,

    cube: Cubemap,
    diff_cube: Cubemap,
    color_tex: Texture2d,

    material_buffer: UniformBuffer<MaterialInfo>,

    camera_pos: Vec3F,
    view: Mat4F,
    projection : Mat4F,

    camera_angle: f32,
    is_animate: bool,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct MaterialInfo {
    MaterialColor: [f32; 3],
}


impl Scene for SceneDiffIbl {

    fn new(display: &impl Facade) -> GLResult<SceneDiffIbl> {

        // Shader Program ------------------------------------------------------------
        let program = SceneDiffIbl::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        let sky_prog = SceneDiffIbl::compile_sky_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let spot = ObjMesh::load(display, "media/spot/spot_triangulated.obj", ObjMeshConfiguration {
            is_with_adjacency: false,
            is_gen_tangents: false,
            is_center: false,
            is_print_load_message: true,
        })?;
        let skybox = SkyBox::new(display, 100.0)?;
        // ----------------------------------------------------------------------------

        // Initialize Textures --------------------------------------------------------
        let cube = load_cubemap(display, "media/texture/cube/grace/grace", CubeMapFaceExtension::Hdr)?;
        let diff_cube = load_cubemap(display, "media/texture/cube/grace-diffuse/grace-diffuse", CubeMapFaceExtension::Hdr)?;
        let color_tex = load_texture(display, "media/spot/spot_texture.png")?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let projection = Mat4F::identity();
        let camera_pos = Vec3F::new(0.0, 4.0, 7.0);
        let view = Mat4F::identity();
        let camera_angle = 90.0_f32.to_radians();
        let is_animate = true;
        // ----------------------------------------------------------------------------


        // Initialize Uniforms --------------------------------------------------------
        glium::implement_uniform_block!(MaterialInfo, MaterialColor);
        let material_buffer = UniformBuffer::immutable(display, MaterialInfo {
            MaterialColor: [0.4, 0.4, 0.4],
        }).map_err(BufferCreationErrorKind::UniformBlock)?;
        // ----------------------------------------------------------------------------

        let scene = SceneDiffIbl {
            program, sky_prog,
            spot, skybox, diff_cube, cube, color_tex,
            material_buffer,
            projection, view, camera_pos, camera_angle, is_animate,
        };
        Ok(scene)
    }

    fn update(&mut self, delta_time: f32) {

        const TWO_PI: f32 = std::f32::consts::PI * 2.0;
        const ROTATE_SPEED: f32 = 0.5;

        if self.is_animating() {
            self.camera_angle = (self.camera_angle + delta_time * ROTATE_SPEED) % TWO_PI;
            self.camera_pos = Vec3F::new(self.camera_angle.cos() * 4.0, 0.0, self.camera_angle.sin() * 4.0);
        }

        self.view = Mat4F::look_at_rh(self.camera_pos, Vec3F::zero(), Vec3F::unit_y());
    }

    fn render(&mut self, frame: &mut glium::Frame) -> GLResult<()> {

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

        // Render Sky -------------------------------------------------------------
        let model = Mat4F::identity();
        let mv: Mat4F = self.view * model;

        let uniforms = uniform! {
            SkyBoxTex: self.cube.sampled()
                .minify_filter(MinifySamplerFilter::Nearest)
                .magnify_filter(MagnifySamplerFilter::Linear),
            MVP: (self.projection * mv).into_col_arrays(),
        };
        self.skybox.render(frame, &self.sky_prog, &draw_params, &uniforms)?;
        // -------------------------------------------------------------------------

        // Render scene ------------------------------------------------------------
        let model = Mat4F::rotation_y(180.0_f32.to_radians());
        let mv: Mat4F = self.view * model;

        let uniforms = uniform! {
            MaterialInfo: &self.material_buffer,
            CamPos: self.camera_pos.into_array(),
            DiffLightTex: self.diff_cube.sampled()
                .minify_filter(MinifySamplerFilter::Nearest)
                .magnify_filter(MagnifySamplerFilter::Linear),
            ColorTex: self.color_tex.sampled()
                .minify_filter(MinifySamplerFilter::Nearest)
                .magnify_filter(MagnifySamplerFilter::Linear),
            ModelMatrix: model.into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.spot.render(frame, &self.program, &draw_params, &uniforms)
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


impl SceneDiffIbl {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/diffuseibl.vert.glsl");
        let fragment_shader_code = include_str!("shaders/diffuseibl.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }

    fn compile_sky_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/skybox.vert.glsl");
        let fragment_shader_code = include_str!("shaders/skybox.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
