
use crate::error::{GLResult, GLErrorKind, BufferCreationErrorKind};

use glium::texture::texture2d::Texture2d;
use glium::framebuffer::{DepthRenderBuffer, SimpleFrameBuffer};
use glium::backend::Facade;

pub use fbo_rentals::{ColorDepthFBO, ColorFBO};

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

rental! {
    mod fbo_rentals {

        #[rental]
        pub struct ColorDepthFBO {
            attachment: Box<super::ColorDepthAttachment>,
            framebuffer: (
                glium::framebuffer::SimpleFrameBuffer<'attachment>,
                &'attachment super::ColorDepthAttachment
            ),
        }

        #[rental]
        pub struct ColorFBO {
            attachment: Box<super::ColorAttachment>,
            framebuffer: (
                glium::framebuffer::SimpleFrameBuffer<'attachment>,
                &'attachment super::ColorAttachment
            ),
        }
    }
}

impl ColorDepthFBO {

    pub fn setup(display: &impl Facade, width: u32, height: u32) -> GLResult<ColorDepthFBO> {

        let color_compoenent = Texture2d::empty(display, width, height)
            .map_err(GLErrorKind::CreateTexture)?;
        let depth_component = DepthRenderBuffer::new(display, glium::texture::DepthFormat::F32, width, height)
            .map_err(BufferCreationErrorKind::RenderBuffer)?;

        // Build the self-referential struct using rental crate.
        let fbo = fbo_rentals::ColorDepthFBO::new(
            Box::new(ColorDepthAttachment { color: color_compoenent, depth: depth_component }),
            // TODO: handle unwrap()
            |attachment| { 
                let framebuffer = SimpleFrameBuffer::with_depth_buffer(display, &attachment.color, &attachment.depth).unwrap();
                (framebuffer, &attachment)
            }
        );

        Ok(fbo)
    }
}

impl ColorFBO {

    pub fn setup(display: &impl Facade, width: u32, height: u32) -> GLResult<ColorFBO> {

        let color_compoenent = Texture2d::empty(display, width, height)
            .map_err(GLErrorKind::CreateTexture)?;

        // Build the self-referential struct using rental crate.
        let fbo = fbo_rentals::ColorFBO::new(
            Box::new(ColorAttachment { color: color_compoenent }),
            |attachment| { 
                // TODO: handle unwrap()
                let framebuffer = SimpleFrameBuffer::new(display, &attachment.color).unwrap();
                (framebuffer, &attachment)
            }
        );

        Ok(fbo)
    }
}
