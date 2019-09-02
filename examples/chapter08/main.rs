
extern crate glsl_cookbook_rs as cookbook;

mod sceneshadowmap;

use sceneshadowmap::SceneShadowMap;

use cookbook::scenerunner::SceneRunner;
use cookbook::scene::Scene;
use cookbook::error::GLResult;

use std::collections::HashMap;
use lazy_static::lazy_static;

const TITLE_PREFIX: &'static str = "Chapter 8 - ";
const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const IS_ENABLE_DEBUG: bool = true;
const MULTISAMPLING: u16 = 0;


lazy_static! {
    static ref HASHMAP: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("ao".into(), "Ambient occlusion from a texture".into());
		m.insert("jitter".into(), "Blur shadow map edges using a random jitter".into());
		m.insert("pcf".into(), "Blur shadow map edges using percentage-closer-filtering".into());
		m.insert("shadow-map".into(), "Simple shadow map".into());
		m.insert("shadow-volume".into(), "Shadow Volumes using geometry shader".into());
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
        | "ao"            => unimplemented!(),
        | "jitter"        => unimplemented!(),
        | "pcf"           => unimplemented!(),
        | "shadow-map"    => run::<SceneShadowMap>(title),
        | "shadow-volume" => unimplemented!(),
        | _ => unreachable!(),
    }
}
