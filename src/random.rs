
use crate::Vec3F;

use rand::distributions::Distribution;

pub fn uniform_hemisphere(rng: &mut rand::rngs::ThreadRng) -> Vec3F {

    let between = rand::distributions::Uniform::from(0.0..1.0_f32);

    let x1 = between.sample(rng);
    let x2 = between.sample(rng);

    let s = (1.0 - x1 * x1).sqrt();

    Vec3F::new(
        (std::f32::consts::PI * 2.0 * x2).cos() * s,
        (std::f32::consts::PI * 2.0 * x2).sin() * s,
        x1,
    )
}

pub fn uniform_circle(rng: &mut rand::rngs::ThreadRng) -> Vec3F {

    let between = rand::distributions::Uniform::from(0.0..1.0_f32);

    let x = between.sample(rng);

    Vec3F::new(
        (std::f32::consts::PI * 2.0 * x).cos(),
        (std::f32::consts::PI * 2.0 * x).sin(),
        0.0,
    )
}
