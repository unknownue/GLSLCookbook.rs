
pub mod scene;
pub mod scenerunner;
pub mod error;
pub mod utils;

pub mod torus;

mod timer;
mod drawable;

pub use drawable::Drawable;

pub type Mat4F = vek::Mat4<f32>;
pub type Mat3F = vek::Mat3<f32>;
pub type Vec4F = vek::Vec4<f32>;
pub type Vec3F = vek::Vec3<f32>;
