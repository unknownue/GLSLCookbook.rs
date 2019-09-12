
extern crate glsl_cookbook_rs as cookbook;
#[macro_use] extern crate itertools;

mod scenetexture;
mod scenemultitex;
mod scenealphatest;
mod scenenormalmap;
mod sceneparallax;
mod scenereflectcube;
mod scenerefractcube;
mod sceneprojtex;
mod scenerendertotex;
mod scenesamplerobj;
mod scenediffibl;

use scenetexture::SceneTexture;
use scenemultitex::SceneMultiTex;
use scenealphatest::SceneAlphaTest;
use scenenormalmap::SceneNormalMap;
use sceneparallax::SceneParallax;
use scenereflectcube::SceneReflectCube;
use scenerefractcube::SceneRefractCube;
use sceneprojtex::SceneProjTex;
use scenerendertotex::SceneRenderToTex;
use scenesamplerobj::SceneSamplerObj;
use scenediffibl::SceneDiffIbl;

use cookbook::scenerunner::SceneRunner;
use cookbook::scene::Scene;
use cookbook::error::GLResult;

use std::collections::HashMap;
use lazy_static::lazy_static;

const TITLE_PREFIX: &'static str = "Chapter 5 - ";
const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const IS_ENABLE_DEBUG: bool = true;
const MULTISAMPLING: u16 = 4;


lazy_static! {
    static ref HASHMAP: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("alpha-test".into(), "Discard fragments based on an alpha test".into());
		m.insert("multi-tex".into(), "Multiple textures".into());
		m.insert("normal-map".into(), "Normal map".into());
		m.insert("proj-tex".into(), "Projected texture".into());
		m.insert("reflect-cube".into(), "Reflection with a cube map".into());
		m.insert("refract-cube".into(), "Refraction with a cube map".into());
		m.insert("render-to-tex".into(), "Render to a texture using framebuffer objects".into());
		m.insert("sampler-obj".into(), "Sampler objects".into());
		m.insert("texture".into(), "Basic texture mapping".into());
		m.insert("diff-ibl".into(), "Diffuse image based lighting".into());
		m.insert("parallax".into(), "Parallax mapping".into());
        m
    };
}

fn run<S: 'static + Scene>(recipe: String) -> GLResult<()> {
    let title: String = String::from(TITLE_PREFIX) + &recipe;
    SceneRunner::new(title, WINDOW_WIDTH, WINDOW_HEIGHT, IS_ENABLE_DEBUG, MULTISAMPLING)?.run::<S>()
}

fn main() -> GLResult<()> {

    let (recipe, title) = SceneRunner::parse_command_line_args(&HASHMAP)?;

    match recipe.as_ref() {
        | "alpha-test"    => run::<SceneAlphaTest>(title),
        | "multi-tex"     => run::<SceneMultiTex>(title),
        | "normal-map"    => run::<SceneNormalMap>(title),
        | "proj-tex"      => run::<SceneProjTex>(title),
        | "reflect-cube"  => run::<SceneReflectCube>(title),
        | "refract-cube"  => run::<SceneRefractCube>(title),
        | "render-to-tex" => run::<SceneRenderToTex>(title),
        | "sampler-obj"   => run::<SceneSamplerObj>(title),
        | "texture"       => run::<SceneTexture>(title),
        | "diff-ibl"      => run::<SceneDiffIbl>(title),
        | "parallax"      => run::<SceneParallax>(title),
        | _ => unreachable!(),
    }
}
