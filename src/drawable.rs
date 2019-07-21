
use glium::{Surface, Frame, Program, DrawParameters};
use glium::uniforms::Uniforms;

use crate::error::{GLResult, GLErrorKind};


pub trait Drawable {
    fn render(&self, frame: &mut Frame, program: &Program, params: &DrawParameters, uniform: &impl Uniforms) -> GLResult<()>;
}

pub trait TriangleMesh<Vertex>
    where Vertex: Copy
{
    fn buffers(&self) -> (&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>);
}

impl<Vertex: Copy> Drawable for TriangleMesh<Vertex> {

    fn render(&self, frame: &mut Frame, program: &Program, params: &DrawParameters, uniform: &impl Uniforms) -> GLResult<()> {
        let (vertices, indices) = self.buffers();
        frame.draw(vertices, indices, program, uniform, params)
            .map_err(GLErrorKind::DrawError)?;
        Ok(())
    }
}
