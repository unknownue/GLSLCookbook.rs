
// Use an alias name for glsl_cookbook_rs crate.
extern crate glsl_cookbook_rs as cookbook;

mod scenebasic;

use scenebasic::SceneBasic;

use cookbook::scenerunner::SceneRunner;
use cookbook::scene::Scene;
use cookbook::error::GLResult;

use std::collections::HashMap;
use lazy_static::lazy_static;

const TITLE_PREFIX: &'static str = "Chapter 1 - ";
const WINDOW_WIDTH : u32 = 500;
const WINDOW_HEIGHT: u32 = 500;
const IS_ENABLE_DEBUG: bool = true;
const MULTISAMPLING: u16 = 0; // Disable multisamping.


lazy_static! {
    static ref HASHMAP: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("basic".into(), "Basic Scene".into());
        m
    };
}

fn run<S: 'static + Scene>(recipe: String) -> GLResult<()> {
    let title: String = String::from(TITLE_PREFIX) + &recipe;
    SceneRunner::run::<S>((title, WINDOW_WIDTH, WINDOW_HEIGHT, MULTISAMPLING, IS_ENABLE_DEBUG).into())
}

fn main() -> GLResult<()> {

    let (recipe, title) = SceneRunner::parse_command_line_args(&HASHMAP)?;

    match recipe.as_ref() {
        | "basic" => run::<SceneBasic>(title),
        | _       => unreachable!(),
    }
}
