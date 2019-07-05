
pub mod scene;


pub type Mat4F = vek::Mat4<f32>;
pub type Mat3F = vek::Mat3<f32>;
pub type Vec4F = vek::Vec4<f32>;
pub type Vec3F = vek::Vec3<f32>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
