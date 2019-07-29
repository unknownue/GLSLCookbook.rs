
extern crate glsl_cookbook_rs as cookbook;

mod scenedirectional;
mod sceneperfragment;
mod scenespot;
mod scenetoon;
mod scenefog;

use scenedirectional::SceneDirectional;
use sceneperfragment::ScenePerfragment;
use scenespot::SceneSpot;
use scenetoon::SceneToon;
use scenefog::SceneFog;

use cookbook::scenerunner::SceneRunner;
use cookbook::scene::Scene;
use cookbook::error::GLResult;

use std::collections::HashMap;
use lazy_static::lazy_static;

const TITLE_PREFIX: &'static str = "Chapter 4 - ";
const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const IS_ENABLE_DEBUG: bool = true;
const MULTISAMPLING: u16 = 0; // Disable multisamping.


lazy_static! {
    static ref HASHMAP: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("directional".into(), "Directional light source".into());
		m.insert("fog".into(), "Fog".into());
		m.insert("multi-light".into(), "Multiple light sources".into());
		m.insert("per-frag".into(), "Per-fragment shading".into());
		m.insert("spot".into(), "Spot light".into());
		m.insert("toon".into(), "Toon shading".into());
		m.insert("pbr".into(), "Physically based rendering (PBR) shader".into());
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
        | "directional" => run::<SceneDirectional>(title),
        | "fog"         => run::<SceneFog>(title),
        | "multi-light" => unimplemented!(),
        | "per-frag"    => run::<ScenePerfragment>(title),
        | "spot"        => run::<SceneSpot>(title),
        | "toon"        => run::<SceneToon>(title),
        | "pbr"         => unimplemented!(),
        | _ => unreachable!(),
    }
}
