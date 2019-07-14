
use crate::Mat4F;
use crate::error::GLResult;

use glium::backend::Facade;


pub trait Scene: Sized {

    fn new(display: &impl Facade, scene_data: SceneData) -> GLResult<Self>;

    /// This is called prior to every frame. Use this to update your animation.
    fn update(&mut self, t: f32);

    /// Draw your scene.
    fn render(&self, display: &glium::Display) -> GLResult<()>;

    /// Called when screen is resized.
    fn resize(&mut self, width: u32, height: u32);


    fn scene_data(&self) -> &SceneData;
    fn scene_data_mut(&mut self) -> &mut SceneData;

    fn is_animating(&self) -> bool {
        self.scene_data().is_animate
    }

    fn set_animate(&mut self, animate: bool) {
        self.scene_data_mut().is_animate = animate;
    }

    fn set_dimension(&mut self, width: u32, height: u32) {
        self.scene_data_mut().width  = width;
        self.scene_data_mut().height = height;
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

impl SceneData {

    pub fn unset() -> SceneData {
        SceneData {
            width: 0, height: 0,

            projection: Default::default(),
            view: Default::default(),
            model: Default::default(),

            is_animate: false,
        }
    }

    pub fn set_dimension(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}
