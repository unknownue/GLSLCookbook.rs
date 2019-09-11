
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::{Mat4F, Mat3F, Vec3F, Vec4F};

use glium::backend::Facade;
use glium::program::{Program, ComputeShader, ProgramCreationError};
use glium::{Surface, uniform, implement_uniform_block};


#[derive(Debug)]
pub struct SceneParticles {

    render_program : Program,
    compute_program: ComputeShader,

    particles_buffer: glium::VertexBuffer<ParticleVertex>,
    attractor_buffer: glium::VertexBuffer<AttractorVertex>,

    angle: f32,
    is_animate: bool,

    bh1: Vec4F,
    bh2: Vec4F,

    projection: Mat4F,

    total_particles: u32,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct AttractorVertex {
    VertexPosition: [f32; 4],
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct ParticleVertex {
    VertexPosition: [f32; 4],
    VertexVelocity: [f32; 4],
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct MyBlock {
    Position: [f32; 4]
}

impl Scene for SceneParticles {

    fn new(display: &impl Facade) -> GLResult<SceneParticles> {

        // Shader Program ------------------------------------------------------------
        let render_program = SceneParticles::compile_render_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        let compute_program = SceneParticles::compile_compute_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------

        // Initialize Buffers ------------------------------------------------------------
        glium::implement_vertex!(AttractorVertex, VertexPosition);
        glium::implement_vertex!(ParticleVertex, VertexPosition, VertexPosition);
        glium::implement_uniform_block!(ParticleVertex, VertexPosition, VertexPosition);

        let n_particles = [100, 100, 100];
        let total_particles = (n_particles[0] * n_particles[1] * n_particles[2]) as u32;
        let particles_buffer = SceneParticles::init_buffers(display, n_particles, total_particles)?;
        let attractor_buffer = glium::VertexBuffer::empty(display, 2)
            .map_err(BufferCreationErrorKind::Vertex)?;
        // ----------------------------------------------------------------------------

        // ----------------------------------------------------------------------------
        let bh1 = Vec4F::new( 5.0, 0.0, 0.0, 1.0);
        let bh2 = Vec4F::new(-5.0, 0.0, 0.0, 1.0);

        let projection = Mat4F::identity();
        let angle = std::f32::consts::PI / 2.0;
        let is_animate = true;
        // ----------------------------------------------------------------------------

        let scene = SceneParticles {
            render_program, compute_program,
            particles_buffer, attractor_buffer,
            projection, angle, is_animate,
            bh1, bh2, total_particles,
        };
        Ok(scene)
    }

    fn update(&mut self, delta_time: f32) {
        
        const TWO_PI: f32 = std::f32::consts::PI * 2.0;
        const ROTATE_SPEED: f32 = 35.0;

        if self.is_animating() {
            self.angle = (self.angle + delta_time * ROTATE_SPEED) % TWO_PI;
        }
    }

    fn render(&mut self, frame: &mut glium::Frame) -> GLResult<()> {

        frame.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
        frame.clear_depth(1.0);

        let view = Mat4F::look_at_rh(Vec3F::new(2.0, 0.0, 20.0), Vec3F::zero(), Vec3F::unit_y());
        let model = Mat4F::identity();
        let mv: Mat4F = view * model;

        // Execute compute shader ------------------------------------------------
        // Rotate the attractors ("black holes")
        let rot: Mat4F = Mat4F::rotation_z(self.angle.to_radians());
        let att1: Vec3F = (rot * self.bh1).into();
        let att2: Vec3F = (rot * self.bh2).into();

        // let write_buffer = self.particles_buffer.map();

        let uniforms = uniform! {
            // MyBlock: &(*write_buffer),
            BlackHolePos1: att1.into_array(),
            BlackHolePos2: att2.into_array(),
        };
        self.compute_program.execute(uniforms, self.total_particles / 1000, 1, 1);
        // -------------------------------------------------------------------------

        // Render Particles ----------------------------------------------------------
        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: glium::draw_parameters::Blend::alpha_blending(),
            point_size: Some(1.0),
            ..Default::default()
        };

        let uniforms = uniform! {
            Color: [0.0_f32, 0.0, 0.0, 0.2],
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        let no_indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);
        frame.draw(&self.particles_buffer, &no_indices, &self.render_program, &uniforms, &draw_params)
            .map_err(GLErrorKind::DrawError)?;
        // -------------------------------------------------------------------------

        // Render attractors ----------------------------------------------------------
        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: glium::draw_parameters::Blend::alpha_blending(),
            point_size: Some(5.0),
            ..Default::default()
        };

        self.attractor_buffer.write(&[
            AttractorVertex { VertexPosition: [att1.x, att1.y, att1.z, 1.0] },
            AttractorVertex { VertexPosition: [att2.x, att2.y, att2.z, 1.0] },
        ]);

        let uniforms = uniform! {
            Color: [1.0_f32, 1.0, 0.0, 1.0],
            ModelViewMatrix: mv.clone().into_col_arrays(),
            NormalMatrix: Mat3F::from(mv).into_col_arrays(),
            MVP: (self.projection * mv).into_col_arrays(),
        };

        let no_indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

        frame.draw(&self.attractor_buffer, &no_indices, &self.render_program, &uniforms, &draw_params)
            .map_err(GLErrorKind::DrawError)?;
        // -------------------------------------------------------------------------

        Ok(())
    }

    fn resize(&mut self, _display: &impl Facade, width: u32, height: u32) -> GLResult<()> {

        self.projection = Mat4F::perspective_rh_zo(50.0_f32.to_radians(), width as f32 / height as f32, 1.0, 100.0);
        Ok(())
    }

    fn is_animating(&self) -> bool {
        self.is_animate
    }
    fn toggle_animation(&mut self) {
        self.is_animate = !self.is_animate;
    }
}


impl SceneParticles {

    fn compile_render_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/particles.vert.glsl");
        let fragment_shader_code = include_str!("shaders/particles.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::program::Program::new(display, sources)
    }

    fn compile_compute_program(display: &impl Facade) -> Result<ComputeShader, ProgramCreationError> {

        let compute_shader_code = include_str!("shaders/particles.comp.glsl");

        ComputeShader::from_source(display, compute_shader_code)
    }

    fn init_buffers(display: &impl Facade, n_particles: [usize; 3], total_particles: u32) -> GLResult<glium::VertexBuffer<ParticleVertex>> {

        // Initial positions of the particles
        let mut vertices = Vec::with_capacity(total_particles as usize * 4);

        let dx = 2.0 / (n_particles[0] - 1) as f32;
        let dy = 2.0 / (n_particles[1] - 1) as f32;
        let dz = 2.0 / (n_particles[2] - 1) as f32;

        // We want to center the particles at (0, 0, 0)
        let translate = Mat4F::translation_3d(Vec3F::new(-1.0, -1.0, -1.0));
        for (i, j, k) in iproduct!(0..n_particles[0], 0..n_particles[1], 0..n_particles[2]) {
            let p = Vec4F::new(dx * i as f32, dy * j as f32, dz * k as f32, 1.0);
            let vertex = ParticleVertex {
                VertexPosition: (translate * p).into_array(),
                VertexVelocity: [0.0, 0.0, 0.0, 0.0],
            };
            vertices.push(vertex);
        }

        let buffer = glium::VertexBuffer::dynamic(display, &vertices)
            .map_err(BufferCreationErrorKind::Vertex)?;
        Ok(buffer)
    }
}
