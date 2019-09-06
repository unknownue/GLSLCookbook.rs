
use glium::texture::texture1d::Texture1d;
use glium::backend::Facade;

use crate::{Vec3F, Mat3F};
use crate::error::{GLResult, GLErrorKind};

/// Make a rotation matrix that rotates the y-axis to be parallel to dir
pub fn make_arbitrary_basis(dir: Vec3F) -> Mat3F {

    let mut v = dir;
    let mut n = Vec3F::unit_x().cross(v);

    if n.magnitude_squared() < 0.001 {
        n = Vec3F::unit_y().cross(v);
    }

    let mut u = v.cross(n);

    u.normalize();
    v.normalize();
    n.normalize();

    Mat3F::new(
        u.x, u.y, u.z,
        v.x, v.y, v.z,
        n.x, n.y, n.z
    )
}

/// Create a 1D texture of random floating point values in the range [0, 1].
/// 
/// size: the number of values
pub fn random_tex_1d(display: &impl Facade, size: usize) -> GLResult<Texture1d> {

    use rand::distributions::Distribution;

    let mut rng = rand::thread_rng();
    let between = rand::distributions::Uniform::from(0.0..1.0_f32);

    let rand_data: Vec<f32> = (0..size)
        .map(|_| between.sample(&mut rng)).collect();
    
    let texture = Texture1d::with_format(display, rand_data, glium::texture::UncompressedFloatFormat::F32, glium::texture::MipmapsOption::NoMipmap)
        .map_err(GLErrorKind::CreateTexture)?;

    Ok(texture)
}
