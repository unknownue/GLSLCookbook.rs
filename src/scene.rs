
use crate::error::GLResult;

use glium::backend::Facade;


pub trait Scene: Sized {

    fn new(display: &impl Facade) -> GLResult<Self>;

    /// This is called prior to every frame. Use this to update your animation.
    fn update(&mut self, t: f32);

    /// Draw your scene.
    fn render(&self, frame: &mut glium::Frame) -> GLResult<()>;

    /// Called when screen is resized.
    fn resize(&mut self, width: u32, height: u32);

    fn is_animating(&self) -> bool;
    fn toggle_animation(&mut self);
}
