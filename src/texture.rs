
use std::path::Path;
use std::fs::File;

use crate::error::{GLResult, GLError, GLErrorKind};

use glium::backend::Facade;
use glium::texture::RawImage2d;
use glium::texture::texture2d::Texture2d;


#[derive(Debug, Clone)]
struct PngContent {
    width : u32,
    height: u32,
    color_type: png::ColorType,
    buffer: Vec<u8>,
}

/// Load the image pixels to a buffer. This function only support PNG file.
fn load_pixels(path: impl AsRef<Path>, is_flip: bool) -> GLResult<PngContent> {

    if is_flip {
        return Err(GLError::unimplemented("Load png image with flipping."))
    }

    let png_file = File::open(path.as_ref())
        .map_err(GLError::io)?;
    let decoder = png::Decoder::new(png_file);
    let (info, mut reader) = decoder.read_info()
        .map_err(|e| GLError::custom(e.to_string()))?;

    if info.color_type != png::ColorType::RGB && info.color_type != png::ColorType::RGBA {
        println!("The png file at {:?} is not an RGB image.", path.as_ref());
    }

    let mut content = PngContent {
        width : info.width,
        height: info.height,
        color_type: info.color_type,
        buffer: vec![0; info.buffer_size()],
    };

    reader.next_frame(&mut content.buffer)
        .map_err(|e| GLError::custom(e.to_string()))?;

    Ok(content)
}

pub fn load_texture(display: &impl Facade, path: impl AsRef<Path>) -> GLResult<Texture2d> {

    let pixels = load_pixels(path, false)?;
    let raw_image = match pixels.color_type {
        | png::ColorType::RGB  => RawImage2d::from_raw_rgb (pixels.buffer, (pixels.width, pixels.height)),
        | png::ColorType::RGBA => RawImage2d::from_raw_rgba(pixels.buffer, (pixels.width, pixels.height)),
        | _ => unreachable!()
    };

    let texture = Texture2d::with_mipmaps(display, raw_image, glium::texture::MipmapsOption::NoMipmap)
        .map_err(GLErrorKind::CreateTexture)?;

    Ok(texture)
}
