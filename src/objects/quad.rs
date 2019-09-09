
use glium::backend::Facade;
use glium::{Surface, Program, DrawParameters};
use glium::uniforms::Uniforms;

use crate::drawable::Drawable;
use crate::error::{GLResult, GLErrorKind, BufferCreationErrorKind};


#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct QuadVertex {
    VertexPosition: [f32; 3], _padding1: f32,
    VertexNormal  : [f32; 3], _padding2: f32,
    VertexTexCoord: [f32; 2],
}

#[derive(Debug)]
pub struct Quad {
    /// vertex buffer of triangle mesh
    vbuffer: glium::VertexBuffer<QuadVertex>,
}

impl Quad {

    pub fn new(display: &impl Facade) -> GLResult<Quad> {
        Quad::new_with_texcoord_scale(display, 1.0)
    }

    pub fn new_with_texcoord_scale(display: &impl Facade, scale: f32) -> GLResult<Quad> {

        glium::implement_vertex!(QuadVertex, VertexPosition, VertexNormal, VertexTexCoord);

        let vertices = [
            QuadVertex { VertexPosition: [-1.0, -1.0, 0.0], VertexTexCoord: [0.0 * scale, 0.0 * scale], ..Default::default() },
            QuadVertex { VertexPosition: [ 1.0, -1.0, 0.0], VertexTexCoord: [1.0 * scale, 0.0 * scale], ..Default::default() },
            QuadVertex { VertexPosition: [ 1.0,  1.0, 0.0], VertexTexCoord: [1.0 * scale, 1.0 * scale], ..Default::default() },
            QuadVertex { VertexPosition: [-1.0, -1.0, 0.0], VertexTexCoord: [0.0 * scale, 0.0 * scale], ..Default::default() },
            QuadVertex { VertexPosition: [ 1.0,  1.0, 0.0], VertexTexCoord: [1.0 * scale, 1.0 * scale], ..Default::default() },
            QuadVertex { VertexPosition: [-1.0,  1.0, 0.0], VertexTexCoord: [0.0 * scale, 1.0 * scale], ..Default::default() },
        ];

        let vbuffer = glium::VertexBuffer::immutable(display, &vertices)
            .map_err(BufferCreationErrorKind::Vertex)?;

        let quad = Quad { vbuffer };
        Ok(quad)
    }
}

impl Drawable for Quad {

    fn render(&self, surface: &mut impl Surface, program: &Program, params: &DrawParameters, uniform: &impl Uniforms) -> GLResult<()> {
        let no_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        surface.draw(&self.vbuffer, no_indices, program, uniform, params)
            .map_err(GLErrorKind::DrawError)?;
        Ok(())
    }

    fn render_instanced(&self, surface: &mut impl Surface, per_instanced: glium::vertex::PerInstance, program: &Program, params: &DrawParameters, uniform: &impl Uniforms) -> GLResult<()> {
        let no_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        surface.draw((&self.vbuffer, per_instanced), &no_indices, program, uniform, params)
            .map_err(GLErrorKind::DrawError)?;
        Ok(())
    }
}
