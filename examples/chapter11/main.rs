
extern crate glsl_cookbook_rs as cookbook;
#[macro_use] extern crate itertools;

mod sceneparticles;

use sceneparticles::SceneParticles;

use cookbook::scenerunner::SceneRunner;
use cookbook::scene::Scene;
use cookbook::error::GLResult;

use std::collections::HashMap;
use lazy_static::lazy_static;

const TITLE_PREFIX: &'static str = "Chapter 11 - ";
const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const IS_ENABLE_DEBUG: bool = true;
const MULTISAMPLING: u16 = 0;


lazy_static! {
    static ref HASHMAP: HashMap<String, String> = {
        let mut m = HashMap::new();
		m.insert("particles".into(), "Simple particle simulation".into());
		m.insert("mandelbrot".into(), "Mandelbrot set with compute shader".into());
		m.insert("cloth".into(), "Cloth simulation with compute shader".into());
		m.insert("edge".into(), "Edge detection filter using compute shader".into());
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
        | "particles"  => run::<SceneParticles>(title),
        | "mandelbrot" => unimplemented!(),
        | "cloth"      => unimplemented!(),
        | "edge"       => unimplemented!(),
        | _ => unreachable!(),
    }
}
