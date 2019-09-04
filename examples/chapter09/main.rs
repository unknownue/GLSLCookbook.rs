
extern crate glsl_cookbook_rs as cookbook;

mod scenenoise;
mod scenesky;
mod scenewood;
mod scenedecay;

use scenenoise::SceneNoise;
use scenesky::SceneSky;
use scenewood::SceneWood;
use scenedecay::SceneDecay;

use cookbook::scenerunner::SceneRunner;
use cookbook::scene::Scene;
use cookbook::error::GLResult;

use std::collections::HashMap;
use lazy_static::lazy_static;

const TITLE_PREFIX: &'static str = "Chapter 9 - ";
const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const IS_ENABLE_DEBUG: bool = true;
const MULTISAMPLING: u16 = 0;


lazy_static! {
    static ref HASHMAP: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("noise".into(), "Just display the raw noise texture".into());
		m.insert("decay".into(), "decay of a teapot".into());
		m.insert("night-vision".into(), "night visiion goggles".into());
		m.insert("paint".into(), "paint spatters on a teapot".into());
		m.insert("sky".into(), "clouds and sky".into());
		m.insert("wood".into(), "wood".into());
		m.insert("rust".into(), "rust".into());
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
        | "noise"        => run::<SceneNoise>(title),
        | "decay"        => run::<SceneDecay>(title),
        | "night-vision" => unimplemented!(),
        | "paint"        => unimplemented!(),
        | "sky"          => run::<SceneSky>(title),
        | "wood"         => run::<SceneWood>(title),
        | "rust"         => unimplemented!(),
        | _ => unreachable!(),
    }
}
