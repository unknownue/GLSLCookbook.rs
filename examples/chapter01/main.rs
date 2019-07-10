
// Use an alias name for glsl_cookbook_rs crate.
extern crate glsl_cookbook_rs as cookbook;

mod scenebasic;

use scenebasic::SceneBasic;
use cookbook::scenerunner::SceneRunner;
use cookbook::scene::SceneData;

use std::collections::HashMap;
use lazy_static::lazy_static;


lazy_static! {
    static ref HASHMAP: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("basic".into(), "Basic Scene".into());
        m
    };
}

macro_rules! run {
    ($runner:ident, $scene:ident) => {
        let mut scene = match $scene {
            | Ok(scene) => scene,
            | Err(err) => panic!("{}", err),
        };
        match $runner.run(&mut scene) {
            | Ok(_) => {},
            | Err(err) => println!("{}", err),
        }
    };
}

fn main() {

    let recipe = SceneRunner::parse_command_line_args(&HASHMAP).unwrap();
    let title: String = String::from("Chapter 1 - ") + &recipe;

    let mut runner = SceneRunner::new(title, 500, 500, 0);
    let scene_data = SceneData::unset();

    match recipe.as_ref() {
        | "basic" => {
            let scene = SceneBasic::new(runner.display_backend(), scene_data);
            run!(runner, scene);
        },
        | _ => panic!("Unknown Scene."),
    };
}
