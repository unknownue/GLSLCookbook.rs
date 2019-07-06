
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


fn main() {

    // let recipe = SceneRunner::parse_command_line_args();
    let recipe = String::from("basic");
    let title: String = String::from("Chapter 1 - ") + &recipe;

    let mut runner = SceneRunner::new(title, 500, 500, 0);
    let scene_data = SceneData::new(500, 500);

    match recipe.as_ref() {
        | "basic" => {
            let mut scene = SceneBasic::new(runner.display_backend(), scene_data);
            runner.run(&mut scene);
        },
        | _ => panic!("Unknown Scene."),
    };
}
