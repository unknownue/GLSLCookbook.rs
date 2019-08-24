
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
pub struct SceneMultilight {

    program: glium::Program,

    mesh: ObjMesh,
    mesh_material: MaterialInfo,
    plane: Plane,
    plane_material: MaterialInfo,

    material_buffer: UniformBuffer<MaterialInfo>,
    light_buffer   : UniformBuffer<LightsWrapper>,

    light_data: LightsWrapper,

    view       : Mat4F,
    projection : Mat4F,
}


#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct LightsWrapper {
    lights: [LightInfo; 5],
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct LightInfo {
    Position: [f32; 4],
    La: [f32; 3], _padding1: f32,
    L : [f32; 3], _padding2: f32,
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


impl Scene for SceneMultilight {

    fn new(display: &impl Facade) -> GLResult<SceneMultilight> {

        // Shader Program ------------------------------------------------------------
        let program = SceneMultilight::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------


        // Initialize Mesh ------------------------------------------------------------
        let plane = Plane::new(display, 10.0, 10.0, 100, 100, 1.0, 1.0)?;
        let plane_material = MaterialInfo {
            Ka: [0.1_f32, 0.1, 0.1],
            Kd: [0.1_f32, 0.1, 0.1],
            Ks: [0.9_f32, 0.9, 0.9],
            Shininess: 180.0_f32, ..Default::default()
        };
        let mesh = ObjMesh::load(display, "media/pig_triangulated.obj", ObjMeshConfiguration {
            is_with_adjacency: false,
            is_gen_tangents: false,
            is_center: true,
            is_print_load_message: true,
        })?;
        let mesh_material = MaterialInfo {
            Ka: [0.5_f32, 0.5, 0.5],
            Kd: [0.4_f32, 0.4, 0.4],
            Ks: [0.9_f32, 0.9, 0.9],
            Shininess: 180.0_f32, ..Default::default()
        };
        // ----------------------------------------------------------------------------


        // Initialize MVP -------------------------------------------------------------
        let view = Mat4F::look_at_rh(Vec3F::new(0.5, 0.75, 0.75), Vec3F::zero(), Vec3F::unit_y());
        let projection = Mat4F::identity();
        // ----------------------------------------------------------------------------


        // Initialize Uniforms --------------------------------------------------------
        let mut lights: LightsWrapper = Default::default();
        for i in 0..5 {
            const TWO_PI: f32 = std::f32::consts::PI * 2.0;
            let x = 2.0 * ((TWO_PI / 5.0) * (i as f32)).cos();
            let z = 2.0 * ((TWO_PI / 5.0) * (i as f32)).sin();

            let light = LightInfo {
                Position: (view * Vec4F::new(x, 1.2, z + 1.0, 1.0)).into_array(),
                ..Default::default()
            };
            lights.lights[i] = light;
        }

        lights.lights[0].L = [0.0, 0.8, 0.8];
        lights.lights[1].L = [0.0, 0.0, 0.8];
        lights.lights[2].L = [0.8, 0.0, 0.0];
        lights.lights[3].L = [0.0, 0.8, 0.0];
        lights.lights[4].L = [0.8, 0.8, 0.8];

        lights.lights[0].La = [0.0, 0.2, 0.2];
        lights.lights[1].La = [0.0, 0.0, 0.2];
        lights.lights[2].La = [0.2, 0.0, 0.0];
        lights.lights[3].La = [0.0, 0.2, 0.0];
        lights.lights[4].La = [0.2, 0.2, 0.2];

        glium::implement_uniform_block!(LightInfo, Position, La, L);
        glium::implement_uniform_block!(LightsWrapper, lights);
        let light_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;

        glium::implement_uniform_block!(MaterialInfo, Ka, Kd, Ks, Shininess);
        let material_buffer = UniformBuffer::empty_immutable(display)
            .map_err(BufferCreationErrorKind::UniformBlock)?;

        // cookbook::utils::print_active_uniform_blocks(&program);
        // ----------------------------------------------------------------------------


        let scene = SceneMultilight {
            program,
            plane, mesh, plane_material, mesh_material,
            material_buffer, light_buffer, light_data: lights,
            view, projection,
        };
        Ok(scene)
    }

    fn update(&mut self, _delta_time: f32) {
        // nothing to do, just keep it empty
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


        // Render Mesh -------------------------------------------------------------
        self.material_buffer.write(&self.mesh_material);
        self.light_buffer.write(&self.light_data);

        let model = Mat4F::rotation_y(90.0_f32.to_radians());
        let mv: Mat4F = self.view * model;
        let uniforms = uniform! {
            LightsWrapper: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.mesh.render(frame, &self.program, &draw_params, &uniforms)?;
        // -------------------------------------------------------------------------

        // Render Plane ----------------------------------------------------------
        self.material_buffer.write(&self.plane_material);
        self.light_buffer.write(&self.light_data);

        let model = Mat4F::translation_3d(Vec3F::new(0.0, -0.45, 0.0));
        let mv: Mat4F = self.view * model;
        let uniforms = uniform! {
            LightsWrapper: &self.light_buffer,
            MaterialInfo: &self.material_buffer,
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.plane.render(frame, &self.program, &draw_params, &uniforms)
        // -------------------------------------------------------------------------
    }

    fn resize(&mut self, _display: &impl Facade, width: u32, height: u32) -> GLResult<()> {

        self.projection = Mat4F::perspective_rh_zo(70.0_f32.to_radians(), width as f32 / height as f32, 0.3, 100.0);
        Ok(())
    }

    fn is_animating(&self) -> bool { false }
    fn toggle_animation(&mut self) {}
}


impl SceneMultilight {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/multilight.vert.glsl");
        let fragment_shader_code = include_str!("shaders/multilight.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
