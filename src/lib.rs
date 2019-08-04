
pub mod scene;
pub mod scenerunner;
pub mod error;
pub mod utils;
pub mod texture;

pub mod objects;
pub mod aabb;

mod timer;
mod drawable;

pub use drawable::Drawable;

pub type Mat4F = vek::Mat4<f32>;
pub type Mat3F = vek::Mat3<f32>;
pub type Vec4F = vek::Vec4<f32>;
pub type Vec3F = vek::Vec3<f32>;
pub type Vec2F = vek::Vec2<f32>;
