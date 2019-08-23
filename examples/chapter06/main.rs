
extern crate glsl_cookbook_rs as cookbook;

mod sceneedge;
mod sceneblur;
mod scenetonemap;
mod scenehdrbloom;

use sceneedge::SceneEdge;
use sceneblur::SceneBlur;
use scenetonemap::SceneToneMap;
use scenehdrbloom::SceneHdrBloom;

use cookbook::scenerunner::SceneRunner;
use cookbook::scene::Scene;
use cookbook::error::GLResult;

use std::collections::HashMap;
use lazy_static::lazy_static;

const TITLE_PREFIX: &'static str = "Chapter 6 - ";
const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const IS_ENABLE_DEBUG: bool = true;
const MULTISAMPLING: u16 = 8;


lazy_static! {
    static ref HASHMAP: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("blur".into(), "Gaussian blur".into());
		m.insert("deferred".into(), "deferred rendering".into());
		m.insert("edge".into(), "edge detection filter".into());
		m.insert("gamma".into(), "gamma correction".into());
		m.insert("msaa".into(), "multisample anti-aliasing".into());
		m.insert("tone-map".into(), "tone mapping example".into());
		m.insert("hdr-bloom".into(), "bloom example with HDR tone mapping".into());
		m.insert("oit".into(), "order independent transparency (requires OpenGL 4.3)".into());
		m.insert("ssao".into(), "Screen space ambieng occlusion example".into());
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
        | "blur"      => run::<SceneBlur>(title),
        | "deferred"  => unimplemented!(),
        | "edge"      => run::<SceneEdge>(title),
        | "gamma"     => unimplemented!(),
        | "msaa"      => unimplemented!(),
        | "tone-map"  => run::<SceneToneMap>(title),
        | "hdr-bloom" => run::<SceneHdrBloom>(title),
        | "oit"       => unimplemented!(),
        | "ssao"      => unimplemented!(),
        | _ => unreachable!(),
    }
}
