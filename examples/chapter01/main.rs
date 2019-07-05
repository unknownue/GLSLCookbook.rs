
// Use an alias name for glsl_cookbook_rs crate.
extern crate glsl_cookbook_rs as cookbook;

mod scenebasic;

use scenebasic::SceneBasic;
use cookbook::scene::{Scene, SceneData};

fn main() {

    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let mut closed = false;

    let scene_data = SceneData::new(800, 600);
    let mut scene = SceneBasic::new(&display, scene_data);

    while !closed {
        scene.render(&display);
        scene.update(0.0);

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
