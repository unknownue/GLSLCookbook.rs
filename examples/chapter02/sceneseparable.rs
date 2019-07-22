
use cookbook::scene::{Scene, SceneData};
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

    fn render(&self, _frame: &mut glium::Frame) -> GLResult<()> {
        unimplemented!()
    }

    #[inline(always)]
    fn scene_data(&self) -> &SceneData { unimplemented!() }
    #[inline(always)]
    fn scene_data_mut(&mut self) -> &mut SceneData { unimplemented!() }
}
