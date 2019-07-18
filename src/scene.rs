
use crate::Mat4F;
use crate::error::GLResult;

use glium::backend::Facade;


pub trait Scene: Sized {

    fn new(display: &impl Facade) -> GLResult<Self>;

    /// This is called prior to every frame. Use this to update your animation.
    fn update(&mut self, t: f32);

    /// Draw your scene.
    fn render(&self, display: &glium::Display) -> GLResult<()>;

    /// Called when screen is resized.
    fn resize(&mut self, width: u32, height: u32) {
        self.scene_data_mut().set_dimension(width, height);
        // the viewport is dynamically set in render method.
    }

    fn scene_data(&self) -> &SceneData;
    fn scene_data_mut(&mut self) -> &mut SceneData;

    fn is_animating(&self) -> bool {
        self.scene_data().is_animate
    }

    fn set_animate(&mut self, animate: bool) {
        self.scene_data_mut().is_animate = animate;
    }
}


#[derive(Debug, Clone)]
pub struct SceneData {
    width : u32,
    height: u32,

    projection: Mat4F,
    view: Mat4F,
    model: Mat4F,

    is_animate: bool,
}

impl Default for SceneData {

    fn default() -> SceneData {
        SceneData {
            width: 800, height: 600,
            is_animate: false,
            projection : Default::default(),
            view       : Default::default(),
            model      : Default::default(),
        }
    }
}

impl SceneData {

    pub fn new(is_animate: bool) -> SceneData {
        SceneData {
            width: 800, height: 600,
            projection : Default::default(),
            view       : Default::default(),
            model      : Default::default(),
            is_animate,
        }
    }

    pub fn new_detail(is_animate: bool, projection: Mat4F, view: Mat4F, model: Mat4F) -> SceneData {
        SceneData {
            width: 800, height: 600,
            projection, view, model, is_animate,
        }
    }

    pub fn set_dimension(&mut self, width: u32, height: u32) {
        self.width  = width;
        self.height = height;
    }

    pub fn viewport(&self) -> glium::Rect {
        glium::Rect {
            left: 0, bottom: 0,
            width : self.width,
            height: self.height,
        }
    }
}
