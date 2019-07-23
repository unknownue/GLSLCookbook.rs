
extern crate glsl_cookbook_rs as cookbook;

mod scenediffuse;
mod scenephong;
mod scenetwoside;

use scenediffuse::SceneDiffuse;
use scenephong::ScenePhong;
use scenetwoside::SceneTwoside;

use cookbook::scenerunner::SceneRunner;
use cookbook::scene::Scene;
use cookbook::error::GLResult;

use std::collections::HashMap;
use lazy_static::lazy_static;

const TITLE_PREFIX: &'static str = "Chapter 3 - ";
const WINDOW_WIDTH : u32 = 500;
const WINDOW_HEIGHT: u32 = 500;
const IS_ENABLE_DEBUG: bool = true;
const MULTISAMPLING: u16 = 0; // Disable multisamping.


lazy_static! {
    static ref HASHMAP: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("phong".into(),      "Phong reflection mdoel (per-vertex)".into());
		m.insert("diffuse".into(),    "Diffuse shading only".into());
		m.insert("discard".into(),    "xample of discarding fragments".into());
		m.insert("flat".into(),       "Flat shading".into());
		m.insert("subroutine".into(), "Using a shader subroutine".into());
		m.insert("two-side".into(),   "Two-sided lighting".into());
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
        | "phong"      => run::<ScenePhong>(title),
        | "diffuse"    => run::<SceneDiffuse>(title),
        | "discard"    => unimplemented!(),
        | "flat"       => unimplemented!(),
        | "subroutine" => unimplemented!(),
        | "two-side"   => run::<SceneTwoside>(title),
        | _ => unreachable!(),
    }
}
