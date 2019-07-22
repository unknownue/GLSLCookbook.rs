
use glium::{Surface, Frame, Program, DrawParameters};
use glium::index::Index;
use glium::uniforms::Uniforms;

use crate::error::{GLResult, GLErrorKind};


pub trait Drawable {
    fn render(&self, frame: &mut Frame, program: &Program, params: &DrawParameters, uniform: &impl Uniforms) -> GLResult<()>;
}

pub trait TriangleMesh {
    type VertexType: Copy;
    type IndexType : Index;

    fn buffers(&self) -> (&glium::VertexBuffer<Self::VertexType>, &glium::IndexBuffer<Self::IndexType>);
}

impl<T, V, I> Drawable for T
    where
        T: TriangleMesh<VertexType = V, IndexType = I>,
        V: Copy,
        I: Index {

    fn render(&self, frame: &mut Frame, program: &Program, params: &DrawParameters, uniform: &impl Uniforms) -> GLResult<()> {
        let (vertices, indices) = self.buffers();
        frame.draw(vertices, indices, program, uniform, params)
            .map_err(GLErrorKind::DrawError)?;
        Ok(())
    }
}
