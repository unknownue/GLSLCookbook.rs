
use glium::{Surface, Program, DrawParameters};
use glium::index::Index;
use glium::uniforms::Uniforms;

use crate::error::{GLResult, GLErrorKind};


pub trait Drawable {
    fn render(&self, surface: &mut impl Surface, program: &Program, params: &DrawParameters, uniform: &impl Uniforms) -> GLResult<()>;
    fn render_instanced(&self, surface: &mut impl Surface, per_instanced: glium::vertex::PerInstance, program: &Program, params: &DrawParameters, uniform: &impl Uniforms) -> GLResult<()>;
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

    fn render(&self, surface: &mut impl Surface, program: &Program, params: &DrawParameters, uniform: &impl Uniforms) -> GLResult<()> {
        let (vertices, indices) = self.buffers();
        surface.draw(vertices, indices, program, uniform, params)
            .map_err(GLErrorKind::DrawError)?;
        Ok(())
    }

    fn render_instanced(&self, surface: &mut impl Surface, per_instanced: glium::vertex::PerInstance, program: &Program, params: &DrawParameters, uniform: &impl Uniforms) -> GLResult<()> {
        let (vertices, indices) = self.buffers();
        surface.draw((vertices, per_instanced), indices, program, uniform, params)
            .map_err(GLErrorKind::DrawError)?;
        Ok(())
    }
}
