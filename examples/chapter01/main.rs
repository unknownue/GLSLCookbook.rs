
// Use an alias name for glsl_cookbook_rs crate.
extern crate glsl_cookbook_rs as cookbook;

mod scenebasic;

use scenebasic::SceneBasic;

use cookbook::scenerunner::SceneRunner;
use cookbook::scene::{Scene, SceneData};
use cookbook::error::{GLResult, GLError};

use std::collections::HashMap;
use lazy_static::lazy_static;


lazy_static! {
    static ref HASHMAP: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("basic".into(), "Basic Scene".into());
        m
    };
}

fn run<S: Scene>(recipe: String) -> GLResult<()> {

    let title: String = String::from("Chapter 1 - ") + &recipe;

    let mut runner = SceneRunner::new(title, 500, 500, 0)?;
    let scene_data = SceneData::unset();

    let mut scene = S::new(runner.display_backend(), scene_data)?;
    runner.run(&mut scene)
}

fn _main() -> GLResult<()> {

    let recipe = SceneRunner::parse_command_line_args(&HASHMAP)?;

    match recipe.as_ref() {
        | "basic" => run::<SceneBasic>(recipe),
        | _       => Err(GLError::args("Unknown Scene")),
    }
}

fn main() {

    match _main() {
        | Ok(())   => {},
        | Err(err) => panic!("{}", err),
    }
}
