
use glium::texture::texture2d::Texture2d;
use glium::backend::Facade;
use glium::texture::{UncompressedFloatFormat, MipmapsOption};

use crate::texture::load_custom_texture;
use crate::error::GLResult;
use crate::{Vec2F, Vec4F};


pub fn generate_periodic_2d_texture(display: &impl Facade, base_freq: f32, persist: f32, w: u32, h: u32, mipmaps: MipmapsOption) -> GLResult<Texture2d> {
    generate_2d_tex(display, base_freq, persist, w, h, mipmaps, true)
}

pub fn generate_2d_texture(display: &impl Facade, base_freq: f32, persist: f32, w: u32, h: u32, mipmaps: MipmapsOption) -> GLResult<Texture2d> {
    generate_2d_tex(display, base_freq, persist, w, h, mipmaps, false)
}

fn generate_2d_tex(display: &impl Facade, base_freq: f32, persist: f32, w: u32, h: u32, mipmaps: MipmapsOption, is_periodit: bool) -> GLResult<Texture2d> {

    println!("Generating noise texture...");
    let width  = w as usize;
    let height = h as usize;

    let bytes = generate_2d_tex_bytes(base_freq, persist, width, height, is_periodit);
    let texture = load_custom_texture(display, bytes, width, height, mipmaps, UncompressedFloatFormat::U8U8U8U8)?;

    println!("Done noise texture generation...");
    Ok(texture)
}

fn generate_2d_tex_bytes(base_freq: f32, persistence: f32, width: usize, height: usize, periodit: bool) -> Vec<u8> {

    let mut data = Vec::with_capacity(width * height * 4);

    let x_factor = 1.0 / (width  - 1) as f32;
    let y_factor = 1.0 / (height - 1) as f32;

    for (row, column) in iproduct!(0..width, 0..height) {

        let x = x_factor * column as f32;
        let y = y_factor * row as f32;
        
        let mut sum = 0.0;
        let mut freq = base_freq;
        let mut persist = persistence;

        for _ in 0..4 {
            let p = Vec2F::new(x * freq, y * freq);

            let val = if periodit {
                perlin2(p, Vec2F::broadcast(freq)) * persist
            } else {
                perlin1(p) * persist
            };

            sum += val;

            // Clamp strictly between 0 and 1
            let result = clamp((sum + 1.0) / 2.0, 0.0, 1.0);

            // Store in texture
            data.push((result * 255.0) as u8);

            freq *= 2.0;
            persist *= persistence;
        }
    }

    data
}

// Plain translation to perlin function in glm.
fn perlin1(position: Vec2F) -> f32 {

    let pi = floor(Vec4F::new(position.x, position.y, position.x, position.y)) + Vec4F::new(0.0, 0.0, 1.0, 1.0);
    let pf = fract(Vec4F::new(position.x, position.y, position.x, position.y)) - Vec4F::new(0.0, 0.0, 1.0, 1.0);

    // To avoid truncation effects in permutation
    let pi = modulus(pi, Vec4F::broadcast(289.0));

    perlin(pi, pf)
}

// Plain translation to perlin function in glm.
fn perlin2(position: Vec2F, rep: Vec2F) -> f32 {

    let pi = floor(Vec4F::new(position.x, position.y, position.x, position.y)) + Vec4F::new(0.0, 0.0, 1.0, 1.0);
    let pf = fract(Vec4F::new(position.x, position.y, position.x, position.y)) - Vec4F::new(0.0, 0.0, 1.0, 1.0);

    // To create noise with explicit period
    let pi = modulus(modulus(pi, Vec4F::new(rep.x, rep.y, rep.x, rep.y)), Vec4F::broadcast(289.0));
    // let pi = Vec4F::new(pi.x % rep.x % 289.0, pi.y % rep.y % 289.0, pi.x % rep.x % 289.0, pi.y % rep.y % 289.0);

    perlin(pi, pf)
}

fn perlin(pi: Vec4F, pf: Vec4F) -> f32 {
   
    let ix = Vec4F::new(pi.x, pi.z, pi.x, pi.z);
    let iy = Vec4F::new(pi.y ,pi.y, pi.w, pi.w);
    let fx = Vec4F::new(pf.x, pf.z, pf.x, pf.z);
    let fy = Vec4F::new(pf.y, pf.y, pf.w, pf.w);

    let i = permute(permute(ix) + iy);

    let mut gx = fract(Vec4F::new(i.x / 41.0, i.y / 41.0, i.z / 41.0, i.w / 41.0)) * 2.0 - Vec4F::one();
    let gy = abs(gx) - Vec4F::broadcast(0.5);
    let tx = floor(gx + Vec4F::broadcast(0.5));
    gx = gx - tx;

    let mut g00 = Vec2F::new(gx.x, gy.x);
    let mut g10 = Vec2F::new(gx.y, gy.y);
    let mut g01 = Vec2F::new(gx.z, gy.z);
    let mut g11 = Vec2F::new(gx.w, gy.w);

    let norm = taylor_inv_sqrt(Vec4F::new(g00.dot(g00), g01.dot(g01), g10.dot(g10), g11.dot(g11)));

    g00 = g00 * norm.x;
    g01 = g01 * norm.y;
    g10 = g10 * norm.z;
    g11 = g11 * norm.w;

    let n00: f32 = g00.dot(Vec2F::new(fx.x, fy.x));
    let n10: f32 = g10.dot(Vec2F::new(fx.y, fy.y));
    let n01: f32 = g01.dot(Vec2F::new(fx.z, fy.z));
    let n11: f32 = g11.dot(Vec2F::new(fx.w, fy.w));

    let fade_xy = fade(Vec2F::new(pf.x, pf.y));
    let n_x_x = mix(n00, n10, fade_xy.x);
    let n_x_y = mix(n01, n11, fade_xy.x);

    let n_xy = mix(n_x_x, n_x_y, fade_xy.y);

    2.3 * n_xy
}

fn abs(v: Vec4F) -> Vec4F {
    Vec4F::new(v.x.abs(), v.y.abs(), v.z.abs(), v.w.abs())
}

fn floor(v: Vec4F) -> Vec4F {
    Vec4F::new(v.x.floor(), v.y.floor(), v.z.floor(), v.w.floor())
}

fn fract(v: Vec4F) -> Vec4F {
    Vec4F::new(v.x.fract(), v.y.fract(), v.z.fract(), v.w.fract())
}

fn mod289(x: Vec4F) -> Vec4F {
    x - floor(x * (1.0 / 289.0)) * 289.0
}

fn permute(x: Vec4F) -> Vec4F {
    mod289((x * 34.0 + 1.0) * x)
}

fn taylor_inv_sqrt(r: Vec4F) -> Vec4F {
    Vec4F::broadcast(1.79284291400159) - Vec4F::broadcast(0.85373472095314) * r
}

fn fade(t: Vec2F) -> Vec2F {
    (t * t * t) * (t * (t * 6.0 - Vec2F::broadcast(15.0)) + Vec2F::broadcast(10.0))
}

fn mix(a: f32, b: f32, r: f32) -> f32 {
    a * (1.0 - r) + b * r
}

fn clamp(v: f32, min: f32, max: f32) -> f32 {
    v.min(max).max(min)
}

fn modulus(v: Vec4F, m: Vec4F) -> Vec4F {

    let modules_ops = |x: f32, y: f32| { x - y * (x / y).floor() };

    Vec4F::new(modules_ops(v.x, m.x), modules_ops(v.y, m.y), modules_ops(v.z, m.z), modules_ops(v.w, m.w))
}
