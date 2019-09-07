
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLError, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::Grid;
use cookbook::texture::load_texture;
use cookbook::{Mat4F, Vec3F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::texture::texture2d::Texture2d;
use glium::{Surface, uniform};


#[derive(Debug)]
pub struct SceneParticles {

    program: glium::Program,
    flat_program: glium::Program,

    grid: Grid,
    vertex_buffer: glium::VertexBuffer<ParticleVertex>,

    water_tex: Texture2d,

    time: f32,
    angle: f32,
    is_animate: bool,

    projection : Mat4F,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct ParticleVertex {
    VertexInitVel: [f32; 3],
    VertexBirthTime: f32,
}

impl Scene for SceneParticles {

    fn new(display: &impl Facade) -> GLResult<SceneParticles> {

        // Shader Program ------------------------------------------------------------
        let program = SceneParticles::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        let flat_program = SceneParticles::compile_flat_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------

        // Initialize Mesh ------------------------------------------------------------
        let grid = Grid::new(display, 10.0, 10)?;
        let vertex_buffer = SceneParticles::init_buffers(display)?;
        // ----------------------------------------------------------------------------

        // Initialize Texture ---------------------------------------------------------
        let water_tex = load_texture(display, "media/texture/bluewater.png")?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let projection = Mat4F::identity();
        let angle = std::f32::consts::FRAC_PI_2;
        let is_animate = true;
        let time = 0.0;
        // ----------------------------------------------------------------------------

        let scene = SceneParticles {
            program, flat_program,
            grid, water_tex, vertex_buffer,
            projection, angle, is_animate, time,
        };
        Ok(scene)
    }

    fn update(&mut self, delta_time: f32) {
        
        const TWO_PI: f32 = std::f32::consts::PI * 2.0;
        const ROTATE_SPEED: f32 = 0.55;

        if self.is_animating() {
            self.time += delta_time;
            self.angle = (self.angle + delta_time * ROTATE_SPEED) % TWO_PI;
        }
    }

    fn render(&mut self, frame: &mut glium::Frame) -> GLResult<()> {

        frame.clear_color_srgb(0.1, 0.1, 0.1, 1.0);
        frame.clear_depth(1.0);

        let view = Mat4F::look_at_rh(Vec3F::new(3.0 * self.angle.cos(), 1.5, 3.0 * self.angle.sin()), Vec3F::new(0.0, 1.5, 0.0), Vec3F::unit_y());
        let model = Mat4F::identity();
        let mv: Mat4F = view * model;

        // Render Grid -------------------------------------------------------------
        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        let uniforms = uniform! {
            color: [0.4_f32, 0.4, 0.4, 1.0],
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.grid.render(frame, &self.flat_program, &draw_params, &uniforms)?;
        // -------------------------------------------------------------------------

        // Render Particles --------------------------------------------------------
        let draw_params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: false,
                ..Default::default()
            },
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        let uniforms = uniform! {
            Time: self.time,
            ParticleLiftTime: 5.5_f32,
            EmitterPos: [1.0_f32, 0.0, 0.0],
            ModelViewMatrix: mv.into_col_arrays(),
            ProjectionMatrix: self.projection.into_col_arrays(),
            ParticleTex: self.water_tex.sampled()
                .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
        };

        let draw_vertices = glium::vertex::EmptyVertexAttributes { len: 6 };
        let per_instance = self.vertex_buffer.per_instance()
            .map_err(|_| GLError::device("Invalid draw instance usage"))?;
        let no_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        frame.draw((draw_vertices, per_instance), &no_indices, &self.program, &uniforms, &draw_params)
            .map_err(GLErrorKind::DrawError)?;
        // -------------------------------------------------------------------------

        Ok(())
    }

    fn resize(&mut self, _display: &impl Facade, width: u32, height: u32) -> GLResult<()> {

        self.projection = Mat4F::perspective_rh_zo(60.0_f32.to_radians(), width as f32 / height as f32, 0.3, 100.0);
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

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/particles.vert.glsl");
        let fragment_shader_code = include_str!("shaders/particles.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }

    fn compile_flat_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/flat.vert.glsl");
        let fragment_shader_code = include_str!("shaders/flat.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }

    fn init_buffers(display: &impl Facade) -> GLResult<glium::VertexBuffer<ParticleVertex>> {

        use cookbook::particle;
        use rand::distributions::Distribution;

        let mut rng = rand::thread_rng();
        let between = rand::distributions::Uniform::from(0.0..1.0_f32);

        const N_PARTICLES: usize = 8000;
        const PARTICLE_LIFE_TIME: f32 = 5.5;

        let emitter_dir = Vec3F::new(-1.0, 2.0, 0.0);
        let rate = PARTICLE_LIFE_TIME / N_PARTICLES as f32;

        let mix = |a: f32, b: f32, r: f32| -> f32 { a * (1.0 - r) + b * r };

        let emitter_basis = particle::make_arbitrary_basis(emitter_dir);
        let data: Vec<ParticleVertex> = (0..N_PARTICLES).map(|i| {
            
            let theta = mix(0.0, std::f32::consts::PI / 20.0, between.sample(&mut rng));
            let phi   = mix(0.0, std::f32::consts::PI * 2.0,  between.sample(&mut rng));

            let v = Vec3F::new(
                theta.sin() * phi.cos(),
                theta.cos(),
                theta.sin() * phi.sin(),
            );

            let velocity: f32 = mix(1.25, 1.5, between.sample(&mut rng));
            let v = (emitter_basis * v).normalized() * velocity;

            ParticleVertex {
                VertexInitVel: v.into_array(),
                VertexBirthTime: i as f32 * rate,
            }
        }).collect();


        glium::implement_vertex!(ParticleVertex, VertexInitVel, VertexBirthTime);
        let vertex_buffer = glium::VertexBuffer::immutable(display, &data)
            .map_err(BufferCreationErrorKind::Vertex)?;

        Ok(vertex_buffer)
    }
}
