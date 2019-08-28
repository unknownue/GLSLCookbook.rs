
use crate::error::GLResult;

use glium::backend::Facade;
use glium::program;


pub trait Scene: Sized {

    fn new(display: &impl Facade) -> GLResult<Self>;

    /// This is called prior to every frame. Use this to update your animation.
    fn update(&mut self, t: f32);

    /// Draw your scene.
    fn render(&mut self, frame: &mut glium::Frame) -> GLResult<()>;

    /// Called when screen is resized.
    fn resize(&mut self, display: &impl Facade, width: u32, height: u32) -> GLResult<()>;

    fn is_animating(&self) -> bool;
    fn toggle_animation(&mut self);
}


pub struct GLSourceCode<'a> {
    input: program::ProgramCreationInput<'a>,
}

impl<'a> GLSourceCode<'a> {

    pub fn new(vertex_shader: &'a str, fragment_shader: &'a str) -> GLSourceCode<'a> {
        GLSourceCode {
            input: program::ProgramCreationInput::SourceCode {
                vertex_shader, fragment_shader,
                tessellation_control_shader: None,
                tessellation_evaluation_shader: None,
                geometry_shader: None,
                transform_feedback_varyings: None,
                outputs_srgb: false,
                uses_point_size: false,
            },
        }
    }

    pub fn with_geometry_shader(mut self, shader: &'a str) -> GLSourceCode<'a> {
        if let program::ProgramCreationInput::SourceCode { ref mut geometry_shader, .. } = &mut self.input {
            (*geometry_shader) = Some(shader);
        }
        self
    }

    pub fn with_tessellation_control_shader(mut self, shader: &'a str) -> GLSourceCode<'a> {
        if let program::ProgramCreationInput::SourceCode { ref mut tessellation_control_shader, .. } = &mut self.input {
            (*tessellation_control_shader) = Some(shader);
        }
        self
    }

    pub fn with_tessellation_evaluation_shader(mut self, shader: &'a str) -> GLSourceCode<'a> {
        if let program::ProgramCreationInput::SourceCode { ref mut tessellation_evaluation_shader, .. } = &mut self.input {
            (*tessellation_evaluation_shader) = Some(shader);
        }
        self
    }

    pub fn with_srgb_output(mut self, is_enable: bool) -> GLSourceCode<'a> {
        if let program::ProgramCreationInput::SourceCode { ref mut outputs_srgb, .. } = &mut self.input {
            *outputs_srgb = is_enable;
        }
        self
    }

    pub fn with_point_size_enable(mut self, is_enable: bool) -> GLSourceCode<'a> {
        if let program::ProgramCreationInput::SourceCode { ref mut uses_point_size, .. } = &mut self.input {
            *uses_point_size = is_enable;
        }
        self
    }
}

impl<'a> From<GLSourceCode<'a>> for program::ProgramCreationInput<'a> {

    fn from(v: GLSourceCode<'a>) -> program::ProgramCreationInput<'a> {
        v.input
    }
}



// .vert - a vertex shader
// .tesc - a tessellation control shader
// .tese - a tessellation evaluation shader
// .geom - a geometry shader
// .frag - a fragment shader
// .comp - a compute shader

