
use glium::{Surface, Frame, Program, DrawParameters};
use glium::uniforms::Uniforms;

use crate::error::{GLResult, GLErrorKind};


pub trait Drawable {
    fn render(&self, frame: &mut Frame, program: &Program, params: &DrawParameters, uniform: &impl Uniforms) -> GLResult<()>;
}

pub trait TriangleMesh {
    type VertexType: Copy;

    fn buffers(&self) -> (&glium::VertexBuffer<Self::VertexType>, &glium::IndexBuffer<u32>);
}

impl<T, V> Drawable for T
    where
        T: TriangleMesh<VertexType=V>,
        V: Copy {

    fn render(&self, frame: &mut Frame, program: &Program, params: &DrawParameters, uniform: &impl Uniforms) -> GLResult<()> {
        let (vertices, indices) = self.buffers();
        frame.draw(vertices, indices, program, uniform, params)
            .map_err(|e| { println!("{}", e); GLErrorKind::DrawError(e) })?;
            // .map_err(GLErrorKind::DrawError)?;
        Ok(())
    }
}
