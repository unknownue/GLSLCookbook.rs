
use std::path::Path;
use std::fs::File;

use crate::error::{GLResult, GLError, GLErrorKind, BufferCreationErrorKind};

use glium::backend::Facade;
use glium::texture::{RawImage2d, MipmapsOption, CubeLayer};
use glium::texture::texture2d::Texture2d;
use glium::texture::cubemap::Cubemap;
use glium::Surface;

const CUBEMAP_FACES_SUFFIXES: [&str; 6] = [
    "posx", "negx",
    "posy", "negy",
    "posz", "negz",
];
const CUBEMAP_LAYWERS: [CubeLayer; 6] = [
    CubeLayer::PositiveX, CubeLayer::NegativeX,
    CubeLayer::PositiveY, CubeLayer::NegativeY,
    CubeLayer::PositiveZ, CubeLayer::NegativeZ,
];


#[derive(Debug, Clone)]
struct PngContent {
    width : u32,
    height: u32,
    color_type: png::ColorType,
    buffer: Vec<u8>,
}

/// Load the image pixels to a buffer. This function only support PNG file.
fn load_png_pixels(path: impl AsRef<Path>, is_flip: bool) -> GLResult<PngContent> {

    if is_flip {
        return Err(GLError::unimplemented("Load png image with flipping."))
    }

    let png_file = File::open(path.as_ref())
        .map_err(GLError::io)?;
    let decoder = png::Decoder::new(png_file);
    let (info, mut reader) = decoder.read_info()
        .map_err(|e| GLError::custom(e.to_string()))?;

    if info.color_type != png::ColorType::RGB && info.color_type != png::ColorType::RGBA {
        println!("The png file at {:?} is not an RGB or RGBA image.", path.as_ref());
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

/// Load an image file from local to GPU, and return the corresponding Texture2d object.
/// This function only support PNG file.
pub fn load_texture(display: &impl Facade, path: impl AsRef<Path>) -> GLResult<Texture2d> {

    let pixels = load_png_pixels(path, false)?;
    let raw_image = pixels.build_raw_image();

    // The original C++ implementation does not use mipmap, here use mipmap should be generally ok.
    let texture = Texture2d::with_mipmaps(display, raw_image, MipmapsOption::AutoGeneratedMipmaps)
        .map_err(GLErrorKind::CreateTexture)?;

    Ok(texture)
}


#[derive(Debug, Clone, Copy)]
pub enum CubeMapFaceExtension {
    Hdr,
    Png,
}

/// Load cubemap from local to GPU.
/// The file extension must be correctly specified.
/// The path_prefix is the common file path prefix of each face.
pub fn load_cubemap(display: &impl Facade, path_prefix: &str, extension: CubeMapFaceExtension) -> GLResult<Cubemap> {
    match extension {
        | CubeMapFaceExtension::Hdr => load_hdr_cubemap(display, path_prefix),
        | CubeMapFaceExtension::Png => load_png_cubemap(display, path_prefix),
   }
}

fn load_png_cubemap(display: &impl Facade, path_prefix: &str) -> GLResult<Cubemap> {

    // Load pxiels from png files.
    let png_images = load_cubemap_faces(path_prefix, "png", |path| load_png_pixels(path, false))?;

    // Generate Cubemap texture on GPU.
    build_cubemap(display, png_images)
}

fn load_hdr_cubemap(display: &impl Facade, path_prefix: &str) -> GLResult<Cubemap> {

    // Load pxiels from hdr files.
    let hdr_images = load_cubemap_faces(path_prefix, "hdr", |path| {
        let mut hdr_file = File::open(&path)
            .map_err(GLError::io)?;
        hdrldr::load(&mut hdr_file)
            .map_err(|e| GLError::custom(format!("{:?}", e)))
    })?;

    // Generate Cubemap texture on GPU.
    build_cubemap(display, hdr_images)
}

fn load_cubemap_faces<I, T, F>(path_prefix: &str, extension: &str, load_func: F) -> GLResult<Vec<I>>
    where
        I: ImageRawPixels<T>,
        T: Clone,
        F: Fn(String) -> GLResult<I> {

    let mut images: Vec<I> = Vec::with_capacity(6);
    for suffix in CUBEMAP_FACES_SUFFIXES.into_iter() {
        let path: String = path_prefix.to_owned() + "_" + suffix + "." + extension;
        let image = load_func(path)?;

        if image.dimension().0 != image.dimension().1 {
            return Err(GLError::custom("The cubemap face must share the same width and height."))
        }

        images.push(image);
    }

    let dimension = (images[0].dimension().0, images[0].dimension().1);
    if images.iter().any(|image| image.dimension().0 != dimension.0 || image.dimension().1 != dimension.1) {
        return Err(GLError::custom("The image dimension is different among cubemap faces."))
    }

    Ok(images)
}

fn build_cubemap<I, T>(display: &impl Facade, faces_pxiels: Vec<I>) -> GLResult<Cubemap>
    where
        I: ImageRawPixels<T>,
        T: glium::texture::ToClientFormat + glium::texture::PixelValue + Clone {

    // glium currently support upload texture to cubemap faces, so a manual blit is necessary.
    // See https://github.com/glium/glium/issues/644 for detail.
    assert_eq!(faces_pxiels.len(), 6);
    let dimension = faces_pxiels[0].dimension().0;

    let cubemap_texture = Cubemap::empty_with_mipmaps(display, MipmapsOption::EmptyMipmaps, dimension)
        .map_err(GLErrorKind::CreateTexture)?;

    // blit the face data from framebuffer to cubemap faces
    let blit_target = glium::BlitTarget { left: 0, bottom: 0, width: dimension as i32, height: dimension as i32 };

    for (layer, pixels) in CUBEMAP_LAYWERS.into_iter().zip(faces_pxiels.into_iter()) {
        let framebuffer = glium::framebuffer::SimpleFrameBuffer::new(display, cubemap_texture.main_level().image(*layer))
            .map_err(BufferCreationErrorKind::FrameBuffer)?;
        let cubemap_face = Texture2d::new(display, pixels.build_raw_image())
            .map_err(GLErrorKind::CreateTexture)?;

        cubemap_face.as_surface()
            .blit_whole_color_to(&framebuffer, &blit_target, glium::uniforms::MagnifySamplerFilter::Linear);
    }

    unsafe {
        cubemap_texture.generate_mipmaps();
    }

    Ok(cubemap_texture)
}

fn flatten_hdr_pixels(pixels: Vec<hdrldr::RGB>) -> Vec<f32> {

    // let pixels: Vec<f32> = hdr_image.data.into_iter()
    //     .flat_map(|pixel| [pixel.r, pixel.g, pixel.b])
    //     .collect();

    let mut result = Vec::with_capacity(pixels.len() * 3);
    for pixel in pixels {
        result.extend([pixel.r, pixel.g, pixel.b].iter());
    }
    result
}


trait ImageRawPixels<T: Clone> {
    fn dimension(&self) -> (u32, u32);
    fn build_raw_image<'a>(self) -> RawImage2d<'a, T>;
}

impl ImageRawPixels<u8> for PngContent {

    fn dimension(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn build_raw_image<'a>(self) -> RawImage2d<'a, u8> {
        match self.color_type {
            | png::ColorType::RGB  => RawImage2d::from_raw_rgb_reversed (&self.buffer, (self.width, self.height)),
            | png::ColorType::RGBA => RawImage2d::from_raw_rgba_reversed(&self.buffer, (self.width, self.height)),
            | _ => unreachable!()
        }
    }
}

impl ImageRawPixels<f32> for hdrldr::Image {

    fn dimension(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }

    fn build_raw_image<'a>(self) -> RawImage2d<'a, f32> {
        let pixels = flatten_hdr_pixels(self.data);
        RawImage2d::from_raw_rgb_reversed(&pixels, (self.width as u32, self.height as u32))
    }
}
