
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::{ObjMesh, ObjMeshConfiguration, Plane};
use cookbook::{Mat4F, Mat3F, Vec3F, Vec4F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::uniforms::UniformBuffer;
use glium::{Surface, uniform, implement_uniform_block};



#[derive(Debug)]
pub struct ScenePbr {

    program: glium::Program,

    mesh: ObjMesh,
    plane: Plane,

    material_buffer: UniformBuffer<MaterialInfo>,
    light_buffer   : UniformBuffer<LightsWrapper>,

    light_pos: Vec4F,
    light_data: LightsWrapper,

    view       : Mat4F,
    projection : Mat4F,

    light_angle: f32,
    light_rotate_speed: f32,
    is_animate: bool,
}


#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct LightsWrapper {
    // Due to glium::uniforms::UniformBlock is not implement for [T; 3], but implment for [T; 5],
    // here just force its to 5 element, but actually 3 is used.
    Light: [LightInfo; 5],
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct LightInfo {
    Position: [f32; 4],
    L : [f32; 3], _padding1: f32,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct MaterialInfo {
    MaterialColor: [f32; 3],
    MaterialRough: f32,
    IsMetal: bool,
}

impl Scene for ScenePbr {

    fn new(display: &impl Facade) -> GLResult<ScenePbr> {

        // Shader Program ------------------------------------------------------------
        let program = ScenePbr::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let plane = Plane::new(display, 20.0, 20.0, 1, 1, 1.0, 1.0)?;
        let mesh = ObjMesh::load(display, "media/spot/spot_triangulated.obj", ObjMeshConfiguration {
            is_with_adjacency: false,
            is_gen_tangents: false,
            is_center: false,
            is_print_load_message: true,
        })?;
        // ----------------------------------------------------------------------------


        // Initialize MVP -------------------------------------------------------------
        let view = Mat4F::look_at_rh(Vec3F::new(0.0, 4.0, 7.0), Vec3F::zero(), Vec3F::unit_y());
        let projection = Mat4F::identity();
        let light_angle = 0.0;
        let is_animate = true;
        let light_pos = Vec4F::new(5.0, 5.0, 5.0, 1.0);
        let light_rotate_speed = 1.5;
        // ----------------------------------------------------------------------------


        // Initialize Uniforms --------------------------------------------------------
        let mut light_data: LightsWrapper = Default::default();

        light_data.Light[0].L = [45.0, 45.0, 45.0];
        light_data.Light[1].L = [ 0.3,  0.3,  0.3];
        light_data.Light[2].L = [45.0, 45.0, 45.0];

        light_data.Light[0].Position = (view * light_pos).into_array();
        light_data.Light[1].Position = [0.0, 0.15, -1.0, 0.0];
        light_data.Light[2].Position = (view * Vec4F::new(-7.0, 3.0, 7.0, 1.0)).into_array();

        glium::implement_uniform_block!(LightInfo, Position, L);
        glium::implement_uniform_block!(LightsWrapper, Light);
        let light_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;

        glium::implement_uniform_block!(MaterialInfo, MaterialColor, MaterialRough, IsMetal);
        let material_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;

        // cookbook::utils::print_active_uniform_blocks(&program);
        // ----------------------------------------------------------------------------


        let scene = ScenePbr {
            program,
            plane, mesh,
            material_buffer, light_buffer,
            light_data, light_pos, light_rotate_speed,
            view, projection, light_angle, is_animate,
        };
        Ok(scene)
    }

    fn update(&mut self, delta_time: f32) {

        const TWO_PI: f32 = std::f32::consts::PI * 2.0;

        if self.is_animating() {
            self.light_angle = (self.light_angle + delta_time * self.light_rotate_speed) % TWO_PI;
            self.light_pos.x = self.light_angle.cos() * 7.0;
            self.light_pos.y = 3.0;
            self.light_pos.z = self.light_angle.sin() * 7.0;
        }

        self.light_data.Light[0].Position = (self.view * self.light_pos).into_array();
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

        // Draw Scene -----------------------------------------------------------------
        self.draw_floor(frame, &draw_params)?;

        // Draw dielectric cows with varying roughness
        const NUM_COWS: usize = 9;
        const COW_BASE_COLOR: Vec3F = Vec3F::new(0.1, 0.33, 0.17);

        for i in 0..NUM_COWS {
            let cow_x = (i as f32) * (10.0 / (NUM_COWS - 1) as f32) - 5.0;
            let rough = (i as f32 + 1.0) * (1.0 / NUM_COWS as f32);
            self.draw_spot(frame, &draw_params, Vec3F::new(cow_x, 0.0, 0.0), rough, false, COW_BASE_COLOR)?;
        }

        // Draw metal cows
        const METAL_ROUGH: f32 = 0.43;
        // Gold
        self.draw_spot(frame, &draw_params, Vec3F::new(-3.0, 0.0, 3.0), METAL_ROUGH, true, Vec3F::new(1.0, 0.71, 0.29))?;
        // Copper
        self.draw_spot(frame, &draw_params, Vec3F::new(-1.5, 0.0, 3.0), METAL_ROUGH, true, Vec3F::new(0.95, 0.64, 0.54))?;
        // Aluminum
        self.draw_spot(frame, &draw_params, Vec3F::new( 0.0, 0.0, 3.0), METAL_ROUGH, true, Vec3F::new(0.91, 0.92, 0.92))?;
        // Titanium
        self.draw_spot(frame, &draw_params, Vec3F::new( 1.5, 0.0, 3.0), METAL_ROUGH, true, Vec3F::new(0.542, 0.497, 0.449))?;
        // Silver
        self.draw_spot(frame, &draw_params, Vec3F::new( 3.0, 0.0, 3.0), METAL_ROUGH, true, Vec3F::new(0.95, 0.93, 0.88))
        // ----------------------------------------------------------------------------
    }

    fn resize(&mut self, width: u32, height: u32) {

        self.projection = Mat4F::perspective_rh_zo(60.0_f32.to_radians(), width as f32 / height as f32, 0.3, 100.0);
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }

    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate;
    }
}


impl ScenePbr {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/pbr.vert.glsl");
        let fragment_shader_code = include_str!("shaders/pbr.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }

    fn draw_floor(&self, frame: &mut glium::Frame, draw_params: &glium::DrawParameters) -> GLResult<()> {

        self.material_buffer.write(&MaterialInfo {
            MaterialColor: [0.2, 0.2, 0.2],
            MaterialRough: 0.9,
            IsMetal: false,
        });
        self.light_buffer.write(&self.light_data);

        let model = Mat4F::translation_3d(Vec3F::new(0.0, -0.75, 0.0));
        let mv: Mat4F = self.view * model;
        let uniforms = uniform! {
            LightsWrapper: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.plane.render(frame, &self.program, draw_params, &uniforms)
    }

    fn draw_spot(&self, frame: &mut glium::Frame, draw_params: &glium::DrawParameters, pos: Vec3F, rough: f32, is_metal: bool, color: Vec3F) -> GLResult<()> {

        self.material_buffer.write(&MaterialInfo {
            MaterialColor: color.into_array(),
            MaterialRough: rough,
            IsMetal: is_metal,
        });
        self.light_buffer.write(&self.light_data);

        let model = Mat4F::rotation_y(180.0_f32.to_radians())
            .translated_3d(pos);
        let mv: Mat4F = self.view * model;
        let uniforms = uniform! {
            LightsWrapper: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.mesh.render(frame, &self.program, draw_params, &uniforms)
    }
}
