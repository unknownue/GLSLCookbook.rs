
use crate::error::{GLResult, GLErrorKind, BufferCreationErrorKind};

use glium::texture::texture2d::Texture2d;
use glium::framebuffer::{DepthRenderBuffer, SimpleFrameBuffer};
use glium::backend::Facade;

pub use fbo_rentals::GLFrameBuffer;

// Note: Since glium::framebuffer::SimpleFrameBuffer is need for Texture Rendering, but contains referential member,
//     there rental crate is used to avoid the self-reference conflit in Rust.
//     It makes the code more uglier, but it works.
// See https://github.com/glium/glium/blob/master/examples/deferred.rs for an example of this use case.

pub struct ColorDepthAttachment {
    pub color: Texture2d,
    pub depth: DepthRenderBuffer,
}

pub struct ColorAttachment {
    pub color: Texture2d,
}

pub trait GLAttachment: Sized {
    fn new_attachment(display: &impl Facade, width: u32, height: u32) -> GLResult<Self>;
    fn new_framebuffer<'a>(display: &impl Facade, attachment: &'a Self) -> GLResult<SimpleFrameBuffer<'a>>;
}

impl GLAttachment for ColorAttachment {

    fn new_attachment(display: &impl Facade, width: u32, height: u32) -> GLResult<ColorAttachment> {

        let color_compoenent = Texture2d::empty(display, width, height)
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

        let color_compoenent = Texture2d::empty(display, width, height)
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
