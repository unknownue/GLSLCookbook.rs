
use crate::Mat4F;

pub trait Scene {

    /// This is called prior to every frame. Use this to update your animation.
    fn update(&mut self, t: f32);

    /// Draw your scene.
    fn render(&self, display: &glium::Display);

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
    pub width : u32,
    pub height: u32,

    pub projection: Mat4F,
    pub view: Mat4F,
    pub model: Mat4F,

    pub is_animate: bool,
}

impl SceneData {

    pub fn new(width: u32, height: u32) -> SceneData {
        SceneData {
            width, height,

            projection: Default::default(),
            view: Default::default(),
            model: Default::default(),

            is_animate: false,
        }
    }

}
