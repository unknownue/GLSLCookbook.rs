
extern crate glsl_cookbook_rs as cookbook;

mod scenepointsprite;
mod sceneshadewire;
mod scenesilhouette;
mod scenebezcurve;

use scenepointsprite::ScenePointSprite;
use sceneshadewire::SceneShadeWire;
use scenesilhouette::SceneSilhouette;
use scenebezcurve::SceneBezCurve;

use cookbook::scenerunner::SceneRunner;
use cookbook::scene::Scene;
use cookbook::error::GLResult;

use std::collections::HashMap;
use lazy_static::lazy_static;

const TITLE_PREFIX: &'static str = "Chapter 7 - ";
const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const IS_ENABLE_DEBUG: bool = true;
const MULTISAMPLING: u16 = 0;


lazy_static! {
    static ref HASHMAP: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("bez-curve".into(), "2D Bezier curve with tessellation shader".into());
		m.insert("point-sprite".into(), "Point sprites with the geometry shader".into());
		m.insert("quad-tess".into(), "Demonstrates how quad tessellation works".into());
		m.insert("shade-wire".into(), "Uses the geometry shader to draw a mesh over a shaded object".into());
		m.insert("silhouette".into(), "Uses the geometry shader to draw silhouette edges".into());
		m.insert("tess-teapot".into(), "Uses tessellation to draw a teapot".into());
		m.insert("tess-teapot-depth".into(), "Varies the amount of tessellation with depth".into());
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
        | "bez-curve"         => run::<SceneBezCurve>(title),
        | "point-sprite"      => run::<ScenePointSprite>(title),
        | "quad-tess"         => unimplemented!(),
        | "shade-wire"        => run::<SceneShadeWire>(title),
        | "silhouette"        => run::<SceneSilhouette>(title),
        | "tess-teapot"       => unimplemented!(),
        | "tess-teapot-depth" => unimplemented!(),
        | _ => unreachable!(),
    }
}