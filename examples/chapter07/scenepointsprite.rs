
use cookbook::scene::{Scene, GLSourceCode};
use cookbook::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use cookbook::{Mat4F, Vec3F};
use cookbook::texture::load_texture;

use glium::backend::Facade;
use glium::program::{Program, ProgramCreationError};
use glium::texture::texture2d::Texture2d;
use glium::{Surface, uniform};

use rand::distributions::Distribution;


#[derive(Debug)]
pub struct ScenePointSprite {

    program: glium::Program,

    vertex_buffer: glium::VertexBuffer<Vertex>,
    flower_tex: Texture2d,
    
    projection: Mat4F,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct Vertex {
    VertexPosition: [f32; 3],
}


impl Scene for ScenePointSprite {

    fn new(display: &impl Facade) -> GLResult<ScenePointSprite> {

        // Shader Program ------------------------------------------------------------
        let program = ScenePointSprite::compile_shader_program(display)
            .map_err(GLErrorKind::CreateProgram)?;
        // ----------------------------------------------------------------------------

        // Initialize Textures --------------------------------------------------------
        let flower_tex = load_texture(display, "media/texture/flower.png")?;
        // ----------------------------------------------------------------------------

        // Initialize MVP -------------------------------------------------------------
        let projection = Mat4F::identity();
        // ----------------------------------------------------------------------------

        // initialize Sprites ---------------------------------------------------------
        let mut rng = rand::thread_rng();
        let between = rand::distributions::Uniform::from(-1.0..1.0_f32);

        const NUM_SPRITES: usize = 50;
        let mut locations: [Vertex; NUM_SPRITES] = [Default::default(); NUM_SPRITES];

        for i in 0..NUM_SPRITES {
            locations[i] = Vertex {
                VertexPosition: [
                    between.sample(&mut rng),
                    between.sample(&mut rng),
                    between.sample(&mut rng),
                ],
            };
        }

        glium::implement_vertex!(Vertex, VertexPosition);
        let vertex_buffer = glium::VertexBuffer::immutable(display, &locations)
            .map_err(BufferCreationErrorKind::Vertex)?;
        // ----------------------------------------------------------------------------

        // Initialize Uniforms --------------------------------------------------------
        // ----------------------------------------------------------------------------


        let scene = ScenePointSprite {
            program, vertex_buffer,
            flower_tex,
            projection,
        };
        Ok(scene)
    }

    fn update(&mut self, _delta_time: f32) {}

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

        let camera_pos = Vec3F::new(0.0, 0.0, 3.0);
        let view = Mat4F::look_at_rh(camera_pos, Vec3F::zero(), Vec3F::unit_y());

        let model = Mat4F::identity();
        let mv: Mat4F = view * model;

        let uniforms = uniform! {
            Size2: 0.15_f32,
            SpriteTex: self.flower_tex.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
            ModelViewMatrix: mv.clone().into_col_arrays(),
            ProjectionMatrix: self.projection.into_col_arrays(),
        };

        let no_indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

        frame.draw(&self.vertex_buffer, &no_indices, &self.program, &uniforms, &draw_params)
            .map_err(GLErrorKind::DrawError)?;
        
        Ok(())
    }

    fn resize(&mut self, _display: &impl Facade, width: u32, height: u32) -> GLResult<()> {

        self.projection = Mat4F::perspective_rh_zo(60.0_f32.to_radians(), (width as f32) / (height as f32), 0.3, 100.0);
        Ok(())
    }

    fn is_animating(&self) -> bool { false }
    fn toggle_animation(&mut self) {}
}


impl ScenePointSprite {

    fn compile_shader_program(display: &impl Facade) -> Result<Program, ProgramCreationError> {

        let vertex_shader_code   = include_str!("shaders/pointsprite.vert.glsl");
        let geometry_shader_code = include_str!("shaders/pointsprite.geom.glsl");
        let fragment_shader_code = include_str!("shaders/pointsprite.frag.glsl");

        let sources = GLSourceCode::new(vertex_shader_code, fragment_shader_code)
            .with_geometry_shader(geometry_shader_code)
            .with_srgb_output(true);
        glium::Program::new(display, sources)
    }
}
