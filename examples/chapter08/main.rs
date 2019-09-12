
extern crate glsl_cookbook_rs as cookbook;

mod sceneshadowmap;
mod scenepcf;
mod sceneao;

use sceneshadowmap::SceneShadowMap;
use scenepcf::ScenePcf;
use sceneao::SceneAo;

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

fn run<S: 'static + Scene>(recipe: String) -> GLResult<()> {
    let title: String = String::from(TITLE_PREFIX) + &recipe;
    SceneRunner::new(title, WINDOW_WIDTH, WINDOW_HEIGHT, IS_ENABLE_DEBUG, MULTISAMPLING)?.run::<S>()
}

fn main() -> GLResult<()> {

    let (recipe, title) = SceneRunner::parse_command_line_args(&HASHMAP)?;

    match recipe.as_ref() {
        | "ao"            => run::<SceneAo>(title),
        | "jitter"        => unimplemented!(),
        | "pcf"           => run::<ScenePcf>(title),
        | "shadow-map"    => run::<SceneShadowMap>(title),
        | "shadow-volume" => unimplemented!(),
        | _ => unreachable!(),
    }
}
