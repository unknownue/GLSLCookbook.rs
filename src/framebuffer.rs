
use crate::error::{GLResult, GLErrorKind, BufferCreationErrorKind};

use glium::texture::texture2d::Texture2d;
use glium::framebuffer::{DepthRenderBuffer, SimpleFrameBuffer, MultiOutputFrameBuffer};
use glium::texture::{MipmapsOption, UncompressedFloatFormat};
use glium::backend::Facade;

pub use fbo_rentals::{GLFrameBuffer, GLDeferredFrameBuffer};

// Note: Since glium::framebuffer::SimpleFrameBuffer is need for Texture Rendering, but contains referential member,
//     there rental crate is used to avoid the self-reference conflit in Rust.
//     It makes the code more uglier, but it works.
// See https://github.com/glium/glium/blob/master/examples/deferred.rs for an example of this use case.

/// Attachment with Color and Depth components used for single output framebuffer rendering.
pub struct ColorDepthAttachment {
    pub color: Texture2d,
    pub depth: DepthRenderBuffer,
}

/// Attachment with only Color components used for single output framebuffer rendering.
pub struct ColorAttachment {
    pub color: Texture2d,
}

/// Attachment with only Color components with high-resolution used for single output framebuffer rendering.
pub struct HdrColorAttachment {
    pub color: Texture2d, // F32F32F32
}

/// Attachment with Color and Depth components with high-resolution used for single output framebuffer rendering.
pub struct HdrColorDepthAttachment {
    pub color: Texture2d,
    pub depth: DepthRenderBuffer,
}

pub trait GLAttachment: Sized {
    fn new_attachment(display: &impl Facade, width: u32, height: u32) -> GLResult<Self>;
    fn new_framebuffer<'a>(display: &impl Facade, attachment: &'a Self) -> GLResult<SimpleFrameBuffer<'a>>;
}



impl GLAttachment for ColorAttachment {

    fn new_attachment(display: &impl Facade, width: u32, height: u32) -> GLResult<ColorAttachment> {

        let color_compoenent = Texture2d::empty_with_mipmaps(display, MipmapsOption::NoMipmap, width, height)
            .map_err(GLErrorKind::CreateTexture)?;
        let attachment = ColorAttachment { color: color_compoenent };
        Ok(attachment)
    }

    fn new_framebuffer<'a>(display: &impl Facade, attachment: &'a ColorAttachment) -> GLResult<SimpleFrameBuffer<'a>> {
        let framebuffer = SimpleFrameBuffer::new(display, &attachment.color)
            .map_err(BufferCreationErrorKind::FrameBuffer)?;
        Ok(framebuffer)
    }
}

impl GLAttachment for ColorDepthAttachment {

    fn new_attachment(display: &impl Facade, width: u32, height: u32) -> GLResult<ColorDepthAttachment> {

        let color_compoenent = Texture2d::empty_with_mipmaps(display, MipmapsOption::NoMipmap, width, height)
            .map_err(GLErrorKind::CreateTexture)?;
        let depth_component = DepthRenderBuffer::new(display, glium::texture::DepthFormat::F32, width, height)
            .map_err(BufferCreationErrorKind::RenderBuffer)?;
        let attachment = ColorDepthAttachment { color: color_compoenent, depth: depth_component };
        Ok(attachment)
    }

    fn new_framebuffer<'a>(display: &impl Facade, attachment: &'a ColorDepthAttachment) -> GLResult<SimpleFrameBuffer<'a>> {
        let framebuffer = SimpleFrameBuffer::with_depth_buffer(display, &attachment.color, &attachment.depth)
            .map_err(BufferCreationErrorKind::FrameBuffer)?;
        Ok(framebuffer)
    }
}

impl GLAttachment for HdrColorAttachment {

    fn new_attachment(display: &impl Facade, width: u32, height: u32) -> GLResult<HdrColorAttachment> {

        let color_compoenent = Texture2d::empty_with_format(display, UncompressedFloatFormat::F32F32F32, MipmapsOption::NoMipmap, width, height)
            .map_err(GLErrorKind::CreateTexture)?;
        let attachment = HdrColorAttachment { color: color_compoenent };
        Ok(attachment)
    }

    fn new_framebuffer<'a>(display: &impl Facade, attachment: &'a HdrColorAttachment) -> GLResult<SimpleFrameBuffer<'a>> {
        let framebuffer = SimpleFrameBuffer::new(display, &attachment.color)
            .map_err(BufferCreationErrorKind::FrameBuffer)?;
        Ok(framebuffer)
    }
}

impl GLAttachment for HdrColorDepthAttachment {

    fn new_attachment(display: &impl Facade, width: u32, height: u32) -> GLResult<HdrColorDepthAttachment> {

        let color_compoenent = Texture2d::empty_with_mipmaps(display, MipmapsOption::NoMipmap, width, height)
            .map_err(GLErrorKind::CreateTexture)?;
        let depth_component = DepthRenderBuffer::new(display, glium::texture::DepthFormat::F32, width, height)
            .map_err(BufferCreationErrorKind::RenderBuffer)?;
        let attachment = HdrColorDepthAttachment { color: color_compoenent, depth: depth_component };
        Ok(attachment)
    }

    fn new_framebuffer<'a>(display: &impl Facade, attachment: &'a HdrColorDepthAttachment) -> GLResult<SimpleFrameBuffer<'a>> {
        let framebuffer = SimpleFrameBuffer::with_depth_buffer(display, &attachment.color, &attachment.depth)
            .map_err(BufferCreationErrorKind::FrameBuffer)?;
        Ok(framebuffer)
    }
}

impl GLDeferredAttachment for DeferredPNCAttachment {

   fn new_attachment(display: &impl Facade, width: u32, height: u32) -> GLResult<DeferredPNCAttachment> {

        let position = Texture2d::empty_with_format(display, UncompressedFloatFormat::F32F32F32, MipmapsOption::NoMipmap, width, height)
            .map_err(GLErrorKind::CreateTexture)?;
        let normal   = Texture2d::empty_with_format(display, UncompressedFloatFormat::F32F32F32, MipmapsOption::NoMipmap, width, height)
            .map_err(GLErrorKind::CreateTexture)?;
        let color    = Texture2d::empty_with_format(display, UncompressedFloatFormat::F32F32F32, MipmapsOption::NoMipmap, width, height)
            .map_err(GLErrorKind::CreateTexture)?;
        let depth = DepthRenderBuffer::new(display, glium::texture::DepthFormat::F32, width, height)
            .map_err(BufferCreationErrorKind::RenderBuffer)?;
        let attachment = DeferredPNCAttachment { position, normal, color, depth };
        Ok(attachment)
    }

    fn new_framebuffer<'a>(display: &impl Facade, attachment: &'a DeferredPNCAttachment) -> GLResult<MultiOutputFrameBuffer<'a>> {

        // https://github.com/glium/glium/blob/master/examples/deferred.rs
        let framebuffer = MultiOutputFrameBuffer::with_depth_buffer(display, [
            ("PositionData", &attachment.position),
            ("NormalData",   &attachment.normal),
            ("ColorData",    &attachment.color),
        ].into_iter().cloned(), &attachment.depth).map_err(BufferCreationErrorKind::FrameBuffer)?;
        Ok(framebuffer)
    }
}


/// Attachment with Position, Normal, Color components used for deferred rendering.
pub struct DeferredPNCAttachment {
    pub position: Texture2d, // RGBF32
    pub normal  : Texture2d, // RGBF32
    pub color   : Texture2d, // RGB8
    pub depth   : DepthRenderBuffer,
}

pub trait GLDeferredAttachment: Sized {
    fn new_attachment(display: &impl Facade, width: u32, height: u32) -> GLResult<Self>;
    fn new_framebuffer<'a>(display: &impl Facade, attachment: &'a Self) -> GLResult<MultiOutputFrameBuffer<'a>>;
}

rental! {
    mod fbo_rentals {

        #[rental]
        pub struct GLFrameBuffer<A: 'static> {
            attachment: Box<A>,
            framebuffer: (
                glium::framebuffer::SimpleFrameBuffer<'attachment>,
                &'attachment A,
            ),
        }

        #[rental]
        pub struct GLDeferredFrameBuffer<A: 'static> {
            attachment: Box<A>,
            framebuffer: (
                glium::framebuffer::MultiOutputFrameBuffer<'attachment>,
                &'attachment A,
            ),
        }
    }
}

impl<A> GLFrameBuffer<A>
    where
        A: 'static + GLAttachment {

    pub fn setup(display: &impl Facade, width: u32, height: u32) -> GLResult<GLFrameBuffer<A>> {

        // Build the self-referential struct using rental crate.
        let fbo = fbo_rentals::GLFrameBuffer::new(
            Box::new(A::new_attachment(display, width, height)?),
            |attachment| { 
                // TODO: handle unwrap()
                let framebuffer = A::new_framebuffer(display, &attachment).unwrap();
                (framebuffer, &attachment)
            }
        );

        Ok(fbo)
    }
}

impl<A> GLDeferredFrameBuffer<A>
    where
        A: 'static + GLDeferredAttachment {

    pub fn setup(display: &impl Facade, width: u32, height: u32) -> GLResult<GLDeferredFrameBuffer<A>> {

        // Build the self-referential struct using rental crate.
        let fbo = fbo_rentals::GLDeferredFrameBuffer::new(
            Box::new(A::new_attachment(display, width, height)?),
            |attachment| { 
                // TODO: handle unwrap()
                let framebuffer = A::new_framebuffer(display, &attachment).unwrap();
                (framebuffer, &attachment)
            }
        );

        Ok(fbo)
    }
}
