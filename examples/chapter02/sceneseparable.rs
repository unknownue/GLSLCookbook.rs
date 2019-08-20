
use cookbook::scene::Scene;
use cookbook::error::{GLResult, GLError};

use glium::backend::Facade;

#[derive(Debug)]
pub struct SceneSeparable;

impl Scene for SceneSeparable {

    fn new(_display: &impl Facade) -> GLResult<SceneSeparable> {
        Err(GLError::unsupported("Program Pipeline"))
    }

    fn update(&mut self, _delta_time: f32) {
        unimplemented!()
    }

    fn render(&mut self, _frame: &mut glium::Frame) -> GLResult<()> {
        unimplemented!()
    }

    fn resize(&mut self, _display: &impl Facade, _width: u32, _height: u32) {}

    fn is_animating(&self) -> bool { false }
    fn toggle_animation(&mut self) {}
}
