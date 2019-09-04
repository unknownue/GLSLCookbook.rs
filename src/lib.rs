
#[macro_use] extern crate rental;
#[macro_use] extern crate itertools;

pub mod scene;
pub mod scenerunner;
pub mod error;
pub mod utils;
pub mod texture;
pub mod framebuffer;

pub mod objects;
pub mod aabb;
pub mod random;
pub mod noise;

mod timer;
mod drawable;

pub use drawable::Drawable;

pub type Mat4F = vek::Mat4<f32>;
pub type Mat3F = vek::Mat3<f32>;
pub type Vec4F = vek::Vec4<f32>;
pub type Vec3F = vek::Vec3<f32>;
pub type Vec2F = vek::Vec2<f32>;
