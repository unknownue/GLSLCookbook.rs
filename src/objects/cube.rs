
use glium::backend::Facade;

use crate::drawable::TriangleMesh;
use crate::error::{GLResult, BufferCreationErrorKind};


#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CubeVertex {
    VertexPosition: [f32; 3], _padding1: f32,
    VertexNormal  : [f32; 3], _padding2: f32,
    VertexTexCoord: [f32; 2],
}

#[derive(Debug)]
pub struct Cube {
    /// vertex buffer of triangle mesh
    vbuffer: glium::VertexBuffer<CubeVertex>,
    /// index buffer of triangle mesh
    ibuffer: glium::IndexBuffer<u32>,
}

impl Cube {

    pub fn new(display: &impl Facade, size: f32) -> GLResult<Cube> {

        glium::implement_vertex!(CubeVertex, VertexPosition, VertexNormal, VertexTexCoord);

        let vertices = Cube::generate_vertices(size);
        let indices  = Cube::generate_indices();

        let vbuffer = glium::VertexBuffer::immutable(display, &vertices)
            .map_err(BufferCreationErrorKind::Vertex)?;
        let ibuffer = glium::IndexBuffer::immutable(display, glium::index::PrimitiveType::TrianglesList, &indices)
            .map_err(BufferCreationErrorKind::Index)?;

        let cube = Cube { vbuffer, ibuffer };
        Ok(cube)
    }

    fn generate_vertices(size: f32) -> [CubeVertex; 24] {

        let radius = size / 2.0;

        [
            // Front
            CubeVertex { VertexPosition: [-radius, -radius,  radius], VertexNormal: [0.0, 0.0, 1.0], VertexTexCoord: [0.0, 0.0], ..Default::default() },
            CubeVertex { VertexPosition: [ radius, -radius,  radius], VertexNormal: [0.0, 0.0, 1.0], VertexTexCoord: [1.0, 0.0], ..Default::default() },
            CubeVertex { VertexPosition: [ radius,  radius,  radius], VertexNormal: [0.0, 0.0, 1.0], VertexTexCoord: [1.0, 1.0], ..Default::default() },
            CubeVertex { VertexPosition: [-radius,  radius,  radius], VertexNormal: [0.0, 0.0, 1.0], VertexTexCoord: [0.0, 1.0], ..Default::default() },
            // Right
            CubeVertex { VertexPosition: [ radius, -radius,  radius], VertexNormal: [1.0, 0.0, 0.0], VertexTexCoord: [0.0, 0.0], ..Default::default() },
            CubeVertex { VertexPosition: [ radius, -radius, -radius], VertexNormal: [1.0, 0.0, 0.0], VertexTexCoord: [1.0, 0.0], ..Default::default() },
            CubeVertex { VertexPosition: [ radius,  radius, -radius], VertexNormal: [1.0, 0.0, 0.0], VertexTexCoord: [1.0, 1.0], ..Default::default() },
            CubeVertex { VertexPosition: [ radius,  radius,  radius], VertexNormal: [1.0, 0.0, 0.0], VertexTexCoord: [0.0, 1.0], ..Default::default() },
            // Back
            CubeVertex { VertexPosition: [-radius, -radius, -radius], VertexNormal: [0.0, 0.0, -1.0], VertexTexCoord: [0.0, 0.0], ..Default::default() },
            CubeVertex { VertexPosition: [-radius,  radius, -radius], VertexNormal: [0.0, 0.0, -1.0], VertexTexCoord: [1.0, 0.0], ..Default::default() },
            CubeVertex { VertexPosition: [ radius,  radius, -radius], VertexNormal: [0.0, 0.0, -1.0], VertexTexCoord: [1.0, 1.0], ..Default::default() },
            CubeVertex { VertexPosition: [ radius, -radius, -radius], VertexNormal: [0.0, 0.0, -1.0], VertexTexCoord: [0.0, 1.0], ..Default::default() },
            // Left
            CubeVertex { VertexPosition: [-radius, -radius,  radius], VertexNormal: [-1.0, 0.0, 0.0], VertexTexCoord: [0.0, 0.0], ..Default::default() },
            CubeVertex { VertexPosition: [-radius,  radius,  radius], VertexNormal: [-1.0, 0.0, 0.0], VertexTexCoord: [1.0, 0.0], ..Default::default() },
            CubeVertex { VertexPosition: [-radius,  radius, -radius], VertexNormal: [-1.0, 0.0, 0.0], VertexTexCoord: [1.0, 1.0], ..Default::default() },
            CubeVertex { VertexPosition: [-radius, -radius, -radius], VertexNormal: [-1.0, 0.0, 0.0], VertexTexCoord: [0.0, 1.0], ..Default::default() },
            // Bottom
            CubeVertex { VertexPosition: [-radius, -radius,  radius], VertexNormal: [0.0, -1.0, 0.0], VertexTexCoord: [0.0, 0.0], ..Default::default() },
            CubeVertex { VertexPosition: [-radius, -radius, -radius], VertexNormal: [0.0, -1.0, 0.0], VertexTexCoord: [1.0, 0.0], ..Default::default() },
            CubeVertex { VertexPosition: [ radius, -radius, -radius], VertexNormal: [0.0, -1.0, 0.0], VertexTexCoord: [1.0, 1.0], ..Default::default() },
            CubeVertex { VertexPosition: [ radius, -radius,  radius], VertexNormal: [0.0, -1.0, 0.0], VertexTexCoord: [0.0, 1.0], ..Default::default() },
            // Top
            CubeVertex { VertexPosition: [-radius,  radius,  radius], VertexNormal: [0.0, 1.0, 0.0], VertexTexCoord: [0.0, 0.0], ..Default::default() },
            CubeVertex { VertexPosition: [ radius,  radius,  radius], VertexNormal: [0.0, 1.0, 0.0], VertexTexCoord: [1.0, 0.0], ..Default::default() },
            CubeVertex { VertexPosition: [ radius,  radius, -radius], VertexNormal: [0.0, 1.0, 0.0], VertexTexCoord: [1.0, 1.0], ..Default::default() },
            CubeVertex { VertexPosition: [-radius,  radius, -radius], VertexNormal: [0.0, 1.0, 0.0], VertexTexCoord: [0.0, 1.0], ..Default::default() },
        ]
    }

    fn generate_indices() -> [u32; 36] {
        [
            0, 1, 2, 0, 2, 3,
            4, 5, 6, 4, 6, 7,
            8, 9, 10, 8, 10, 11,
            12, 13, 14, 12, 14, 15,
            16, 17, 18, 16, 18, 19,
            20, 21, 22, 20, 22, 23,
        ]
    }
}

impl TriangleMesh for Cube {
    type VertexType = CubeVertex;
    type IndexType  = u32;

    fn buffers(&self) -> (&glium::VertexBuffer<CubeVertex>, &glium::IndexBuffer<u32>) {
        (&self.vbuffer, &self.ibuffer)
    }
}
