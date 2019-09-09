
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLError, GLErrorKind, BufferCreationErrorKind};
use cookbook::objects::Grid;
use cookbook::texture::load_texture;
use cookbook::particle;
use cookbook::{Mat4F, Vec3F};
use cookbook::Drawable;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::texture::{Texture1d, Texture2d};
use glium::{Surface, uniform};


#[derive(Debug)]
pub struct SceneParticlesFeedback {

    programs: [glium::Program; 2],
    flat_program: glium::Program,

    grid: Grid,

    vbuffer1: glium::VertexBuffer<ParticleVertex>,
    vbuffer2: glium::VertexBuffer<ParticleVertex>,

    water_tex: Texture2d,
    random_tex: Texture1d,

    time: f32,
    delta_time: f32,
    angle: f32,
    is_animate: bool,
    n_particles: usize,
    pass: usize,

    projection : Mat4F,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct ParticleVertex {
    VertexPosition: [f32; 3], 
    VertexVelocity: [f32; 3],
    VertexAge: f32,
}


impl Scene for SceneParticlesFeedback {

    fn new(display: &impl Facade) -> GLResult<SceneParticlesFeedback> {

        // Shader Program ------------------------------------------------------------
        let programs = SceneParticlesFeedback::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        let flat_program = SceneParticlesFeedback::compile_flat_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------

        // Initialize Mesh ------------------------------------------------------------
        const N_PARTICLES: usize = 4000;
        const PARTICLE_LIFETIME: f32 = 6.0;

        let grid = Grid::new(display, 10.0, 10)?;
        let (vbuffer1, vbuffer2) = SceneParticlesFeedback::init_buffers(display, PARTICLE_LIFETIME, N_PARTICLES)?;
        // ----------------------------------------------------------------------------

        // Initialize Texture ---------------------------------------------------------
        let water_tex = load_texture(display, "media/texture/bluewater.png")?;
        let random_tex = particle::random_tex_1d(display, N_PARTICLES * 3)?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let projection = Mat4F::identity();
        let angle = std::f32::consts::FRAC_PI_2;
        let is_animate = true;
        let delta_time = 0.0;
        let time = 0.0;
        let n_particles = N_PARTICLES;
        let pass = 0;
        // ----------------------------------------------------------------------------

        let scene = SceneParticlesFeedback {
            programs, flat_program,
            grid, vbuffer1, vbuffer2,
            water_tex, random_tex,
            projection, angle, is_animate, time, delta_time, n_particles, pass,
        };
        Ok(scene)
    }

    fn update(&mut self, delta_time: f32) {
        
        const TWO_PI: f32 = std::f32::consts::PI * 2.0;
        const ROTATE_SPEED: f32 = 0.55;

        self.delta_time = delta_time;

        if self.is_animating() {
            self.time += delta_time;
            self.angle = (self.angle + delta_time * ROTATE_SPEED) % TWO_PI;
        }
    }

    fn render2(&mut self, display: &impl Facade, frame: &mut glium::Frame) -> GLResult<()> {

        frame.clear_color_srgb(0.1, 0.1, 0.1, 1.0);
        frame.clear_depth(1.0);

        self.render_scene(display, frame, self.pass)?;

        self.pass = 1 - self.pass;
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

    // ignore
    fn render(&mut self, _frame: &mut glium::Frame) -> GLResult<()> { unimplemented!() }
}


impl SceneParticlesFeedback {

    fn compile_shader_program(display: &impl Facade) -> Result<[Program; 2], ProgramCreationError> {

        let pass1_vertex   = include_str!("shaders/transfeedback/pass1.vert.glsl");
        let pass1_fragment = include_str!("shaders/transfeedback/pass1.frag.glsl");

        let pass2_vertex   = include_str!("shaders/transfeedback/pass2.vert.glsl");
        let pass2_fragment = include_str!("shaders/transfeedback/pass2.frag.glsl");

        let transform_feedback_varyings = vec![
            String::from("Position"),
            String::from("Velocity"),
            String::from("Age"),
        ];

        let pass1 = glium::Program::new(display, GLSourceCode::new(pass1_vertex, pass1_fragment)
            .with_transform_feedback_varyings(transform_feedback_varyings.clone(), glium::program::TransformFeedbackMode::Interleaved)
            .with_srgb_output(true))?;
        let pass2 = glium::Program::new(display, GLSourceCode::new(pass2_vertex, pass2_fragment)
            // .with_transform_feedback_varyings(transform_feedback_varyings.clone(), glium::program::TransformFeedbackMode::Interleaved)
            .with_srgb_output(true))?;

        // dbg!(pass1.get_transform_feedback_buffers());
        Ok([pass1, pass2])
    }

    fn compile_flat_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/flat.vert.glsl");
        let fragment_shader_code = include_str!("shaders/flat.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }

    fn init_buffers(display: &impl Facade, particle_lifetime: f32, n_particles: usize) -> GLResult<(glium::VertexBuffer<ParticleVertex>, glium::VertexBuffer<ParticleVertex>)> {

        let rate = particle_lifetime / n_particles as f32;

        glium::implement_vertex!(ParticleVertex, VertexPosition, VertexVelocity, VertexAge);

        let temp_data: Vec<ParticleVertex> = (0..n_particles).map(|i| {
            ParticleVertex {
                VertexAge: -rate * (n_particles - i) as f32,
                ..Default::default()
            }
        }).collect();

        let vbuffer1 = glium::VertexBuffer::dynamic(display, &temp_data)
            .map_err(BufferCreationErrorKind::Vertex)?;
        let vbuffer2 = glium::VertexBuffer::empty_dynamic(display, n_particles)
            .map_err(BufferCreationErrorKind::Vertex)?;
        // dbg!(vbuffer2.get_bindings());

        Ok((vbuffer1, vbuffer2))
    }


    fn render_scene(&mut self, display: &impl Facade, frame: &mut glium::Frame, current_pass: usize) -> GLResult<()> {

        let view = Mat4F::look_at_rh(Vec3F::new(4.0 * self.angle.cos(), 1.5, 4.0 * self.angle.sin()), Vec3F::new(0.0, 1.5, 0.0), Vec3F::unit_y());
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
            color: [0.3_f32, 0.3, 0.3, 1.0],
            MVP: (self.projection * mv).into_col_arrays(),
        };

        self.grid.render(frame, &self.flat_program, &draw_params, &uniforms)?;
        // -------------------------------------------------------------------------

        {
            // Particle Update pass -----------------------------------------------------
            let (transform_feedback, input_buffer) = if current_pass == 0 {
                let feedback = glium::vertex::TransformFeedbackSession::new(display, &self.programs[0], &mut self.vbuffer2)
                    .map_err(|e| GLError::custom(format!("Falied to creation transform feedback session: {}", e)))?;
                (feedback, &self.vbuffer1)
            } else {
                let feedback = glium::vertex::TransformFeedbackSession::new(display, &self.programs[0], &mut self.vbuffer1)
                    .map_err(|e| GLError::custom(format!("Falied to creation transform feedback session: {}", e)))?;
                (feedback, &self.vbuffer2)
            };

            let draw_params = glium::draw_parameters::DrawParameters {
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                // ################################################################################
                // ################################################################################
                draw_primitives: false,  // Enable GL_RASTERIZER_DISCARD feature
                transform_feedback: Some(&transform_feedback),
                // ################################################################################
                // ################################################################################
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            };

            let uniforms = uniform! {
                Time  : self.time,
                DeltaT: self.delta_time,
                ParticleLifeTime: 6.0_f32,
                Accel:   [0.0_f32, -0.5, 0.0],
                Emitter: [1.0_f32,  0.0, 0.0],
                EmitterBasis: particle::make_arbitrary_basis(Vec3F::new(-1.0, 2.0, 0.0)).into_col_arrays(),
                RandomTex: self.random_tex.sampled()
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            };

            let draw_points = glium::index::NoIndices(glium::index::PrimitiveType::Points);

            frame.draw(input_buffer, &draw_points, &self.programs[0], &uniforms, &draw_params)
                .map_err(GLErrorKind::DrawError)?;;
            // ----------------------------------------------------------------------------
        }

        {
            // Particle Render pass --------------------------------------------------------
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
                ParticleLifeTime: 6.0_f32,
                ParticleSize: 0.05_f32,
                ModelViewMatrix: mv.into_col_arrays(),
                ProjectionMatrix: self.projection.into_col_arrays(),
                ParticleTex: self.water_tex.sampled()
                    .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            };

            let per_instance = if current_pass == 0 {
                self.vbuffer2.per_instance()
            } else {
                self.vbuffer1.per_instance()
            }.map_err(|_| GLError::device("Invalid draw instance usage"))?;

            let draw_quad = glium::vertex::EmptyVertexAttributes { len: 6 };
            let no_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

            frame.draw((draw_quad, per_instance), &no_indices, &self.programs[1], &uniforms, &draw_params)
                .map_err(GLErrorKind::DrawError)?;
            // -------------------------------------------------------------------------
        }

        Ok(())
    }
}
