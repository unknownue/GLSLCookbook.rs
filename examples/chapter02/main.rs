
extern crate glsl_cookbook_rs as cookbook;

mod scenebasic_attrib;
mod scenebasic_uniform;
mod scenebasic_uniformblock;

use scenebasic_attrib::SceneBasicAttrib;
use scenebasic_uniform::SceneBasicUniform;
use scenebasic_uniformblock::SceneBasicUniformBlock;

use cookbook::scenerunner::SceneRunner;
use cookbook::scene::Scene;
use cookbook::error::GLResult;

use std::collections::HashMap;
use lazy_static::lazy_static;

const TITLE_PREFIX: &'static str = "Chapter 2 - ";
const WINDOW_WIDTH : u32 = 500;
const WINDOW_HEIGHT: u32 = 500;
const IS_ENABLE_DEBUG: bool = true;
const MULTISAMPLING: u16 = 0; // Disable multisamping.


lazy_static! {
    static ref HASHMAP: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("basic-attrib".into(),        "Prints active attributes".into());
		m.insert("basic-uniform".into(),       "Basic scene with a uniform variable".into());
		m.insert("basic-uniform-block".into(), "Scene with a uniform block variable".into());
		m.insert("separable".into(),           "Scene using separable shaders and program pipelines".into());
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
        | "separable"           => unimplemented!(),
        | "basic-attrib"        => run::<SceneBasicAttrib>(title),
        | "basic-uniform"       => run::<SceneBasicUniform>(title),
        | "basic-uniform-block" => run::<SceneBasicUniformBlock>(title),
        | _ => unreachable!(),
    }
}
