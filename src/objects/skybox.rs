
use glium::backend::Facade;

use crate::drawable::TriangleMesh;
use crate::error::{GLResult, BufferCreationErrorKind};


#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct SkyBoxVertex {
    VertexPosition: [f32; 3],
}

#[derive(Debug)]
pub struct SkyBox {
    /// vertex buffer of triangle mesh
    vbuffer: glium::VertexBuffer<SkyBoxVertex>,
    /// index buffer of triangle mesh
    ibuffer: glium::IndexBuffer<u32>,
}

impl SkyBox {

    pub fn new(display: &impl Facade, size: f32) -> GLResult<SkyBox> {

        glium::implement_vertex!(SkyBoxVertex, VertexPosition);

        let vertices = SkyBox::generate_vertices(size);
        let indices  = SkyBox::generate_indices();

        let vbuffer = glium::VertexBuffer::immutable(display, &vertices)
            .map_err(BufferCreationErrorKind::Vertex)?;
        let ibuffer = glium::IndexBuffer::immutable(display, glium::index::PrimitiveType::TrianglesList, &indices)
            .map_err(BufferCreationErrorKind::Index)?;

        let skybox = SkyBox { vbuffer, ibuffer };
        Ok(skybox)
    }

    fn generate_vertices(size: f32) -> [SkyBoxVertex; 24] {

        let radius = size / 2.0;

        [
            // Front
            SkyBoxVertex { VertexPosition: [-radius, -radius,  radius] },
            SkyBoxVertex { VertexPosition: [ radius, -radius,  radius] },
            SkyBoxVertex { VertexPosition: [ radius,  radius,  radius] },
            SkyBoxVertex { VertexPosition: [-radius,  radius,  radius] },
            // Right
            SkyBoxVertex { VertexPosition: [ radius, -radius,  radius] },
            SkyBoxVertex { VertexPosition: [ radius, -radius, -radius] },
            SkyBoxVertex { VertexPosition: [ radius,  radius, -radius] },
            SkyBoxVertex { VertexPosition: [ radius,  radius,  radius] },
            // Back
            SkyBoxVertex { VertexPosition: [-radius, -radius, -radius] },
            SkyBoxVertex { VertexPosition: [-radius,  radius, -radius] },
            SkyBoxVertex { VertexPosition: [ radius,  radius, -radius] },
            SkyBoxVertex { VertexPosition: [ radius, -radius, -radius] },
            // Left
            SkyBoxVertex { VertexPosition: [-radius, -radius,  radius] },
            SkyBoxVertex { VertexPosition: [-radius,  radius,  radius] },
            SkyBoxVertex { VertexPosition: [-radius,  radius, -radius] },
            SkyBoxVertex { VertexPosition: [-radius, -radius, -radius] },
            // Bottom
            SkyBoxVertex { VertexPosition: [-radius, -radius,  radius] },
            SkyBoxVertex { VertexPosition: [-radius, -radius, -radius] },
            SkyBoxVertex { VertexPosition: [ radius, -radius, -radius] },
            SkyBoxVertex { VertexPosition: [ radius, -radius,  radius] },
            // Top
            SkyBoxVertex { VertexPosition: [-radius,  radius,  radius] },
            SkyBoxVertex { VertexPosition: [ radius,  radius,  radius] },
            SkyBoxVertex { VertexPosition: [ radius,  radius, -radius] },
            SkyBoxVertex { VertexPosition: [-radius,  radius, -radius] },
        ]
    }

    fn generate_indices() -> [u32; 36] {
        [
            0, 2, 1, 0, 3, 2,
            4, 6, 5, 4, 7, 6,
            8, 10, 9, 8, 11, 10,
            12, 14, 13, 12, 15, 14,
            16, 18, 17, 16, 19, 18,
            20, 22, 21, 20, 23, 22,
        ]
    }
}

impl TriangleMesh for SkyBox {
    type VertexType = SkyBoxVertex;
    type IndexType  = u32;

    fn buffers(&self) -> (&glium::VertexBuffer<SkyBoxVertex>, &glium::IndexBuffer<u32>) {
        (&self.vbuffer, &self.ibuffer)
    }
}
