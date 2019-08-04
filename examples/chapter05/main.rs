
extern crate glsl_cookbook_rs as cookbook;

mod scenetexture;

use scenetexture::SceneTexture;

use cookbook::scenerunner::SceneRunner;
use cookbook::scene::Scene;
use cookbook::error::GLResult;

use std::collections::HashMap;
use lazy_static::lazy_static;

const TITLE_PREFIX: &'static str = "Chapter 5 - ";
const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const IS_ENABLE_DEBUG: bool = true;
const MULTISAMPLING: u16 = 0;


lazy_static! {
    static ref HASHMAP: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("alpha-test".into(), "Discard fragments based on an alpha test".into());
		m.insert("multi-tex".into(), "Normal map".into());
		m.insert("proj-tex".into(), "Projected texture".into());
		m.insert("reflect-cube".into(), "Reflection with a cube mapg".into());
		m.insert("refract-cube".into(), "Refraction with a cube map".into());
		m.insert("render-to-tex".into(), "Render to a texture using framebuffer objects".into());
		m.insert("sampler-obj".into(), "Sampler objects".into());
		m.insert("texture".into(), "Basic texture mapping".into());
		m.insert("diff-ibl".into(), "Diffuse image based lighting".into());
		m.insert("parallax".into(), "Parallax mapping".into());
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
        | "multi-tex"     => unimplemented!(),
        | "normal-map"    => unimplemented!(),
        | "proj-tex"      => unimplemented!(),
        | "reflect-cube"  => unimplemented!(),
        | "refract-cube"  => unimplemented!(),
        | "render-to-tex" => unimplemented!(),
        | "sampler-obj"   => unimplemented!(),
        | "texture"       => run::<SceneTexture>(title),
        | "diff-ibl"      => unimplemented!(),
        | "parallax"      => unimplemented!(),
        | _ => unreachable!(),
    }
}