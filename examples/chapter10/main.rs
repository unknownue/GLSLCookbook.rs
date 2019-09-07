
extern crate glsl_cookbook_rs as cookbook;

mod scenewave;
mod sceneparticles;
mod sceneparticlesfeedback;

use scenewave::SceneWave;
use sceneparticles::SceneParticles;
use sceneparticlesfeedback::SceneParticlesFeedback;

use cookbook::scenerunner::SceneRunner;
use cookbook::scene::Scene;
use cookbook::error::GLResult;

use std::collections::HashMap;
use lazy_static::lazy_static;

const TITLE_PREFIX: &'static str = "Chapter 10 - ";
const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const IS_ENABLE_DEBUG: bool = true;
const MULTISAMPLING: u16 = 0;


lazy_static! {
    static ref HASHMAP: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("fire".into(), "Particles simulating fire".into());
		m.insert("particles".into(), "A fountain of particles".into());
		m.insert("particles-feedback".into(), "A fountain of particles implemented with transform feedback".into());
		m.insert("particles-instanced".into(), "A fountain of instanced particles, mmmm.. donuts".into());
		m.insert("smoke".into(), "Particles simulating smoke".into());
		m.insert("wave".into(), "A plane wave displacement animation".into());
        m
    };
}

fn run<S: Scene>(recipe: String) -> GLResult<()> {

    let title: String = String::from(TITLE_PREFIX) + &recipe;

    let mut runner = SceneRunner::new(title, WINDOW_WIDTH, WINDOW_HEIGHT, IS_ENABLE_DEBUG, MULTISAMPLING)?;
    let mut scene = S::new(runner.display_backend())?;

    runner.run(&mut scene)
}

fn main() -> GLResult<()> {

    let (recipe, title) = SceneRunner::parse_command_line_args(&HASHMAP)?;

    match recipe.as_ref() {
        | "fire"                => unimplemented!(),
        | "particles"           => run::<SceneParticles>(title),
        | "particles-feedback"  => run::<SceneParticlesFeedback>(title),
        | "particles-instanced" => unimplemented!(),
        | "smoke"               => unimplemented!(),
        | "wave"                => run::<SceneWave>(title),
        | _ => unreachable!(),
    }
}
