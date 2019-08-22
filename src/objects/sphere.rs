
use glium::backend::Facade;

use crate::drawable::TriangleMesh;
use crate::error::{GLResult, BufferCreationErrorKind};


#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct SphereVertex {
    VertexPosition: [f32; 3], _padding1: f32,
    VertexNormal  : [f32; 3], _padding2: f32,
    VertexTexCoord: [f32; 2],
}

#[derive(Debug)]
pub struct Sphere {
    /// vertex buffer of triangle mesh
    vbuffer: glium::VertexBuffer<SphereVertex>,
    /// index buffer of triangle mesh
    ibuffer: glium::IndexBuffer<u32>,
}

impl Sphere {

    pub fn new(display: &impl Facade, radius: f32, slice_count: usize, stack_count: usize) -> GLResult<Sphere> {

        glium::implement_vertex!(SphereVertex, VertexPosition, VertexNormal, VertexTexCoord);

        let vertices = Sphere::generate_vertices(radius, slice_count, stack_count);
        let indices  = Sphere::generate_indices(slice_count, stack_count);

        let vbuffer = glium::VertexBuffer::immutable(display, &vertices)
            .map_err(BufferCreationErrorKind::Vertex)?;
        let ibuffer = glium::IndexBuffer::immutable(display, glium::index::PrimitiveType::TrianglesList, &indices)
            .map_err(BufferCreationErrorKind::Index)?;

        let sphere = Sphere { vbuffer, ibuffer };
        Ok(sphere)
    }

    fn generate_vertices(radius: f32, slice_count: usize, stack_count: usize) -> Vec<SphereVertex> {

        let vertices_count = (slice_count + 1) * (stack_count + 1);
        let mut vertices = Vec::with_capacity(vertices_count);

        let theta_fac = std::f32::consts::PI * 2.0 / slice_count as f32;
        let phi_fac   = std::f32::consts::PI / stack_count as f32;

        for i in 0..=slice_count {
            let theta = i as f32 * theta_fac;
            let s = i as f32 / slice_count as f32;

            for j in 0..=stack_count {
                let phi = j as f32 * phi_fac;
                let t = j as f32 / stack_count as f32;

                let nx = phi.sin() * theta.cos();
                let ny = phi.sin() * theta.sin();
                let nz = phi.cos();

                let vertex = SphereVertex {
                    VertexPosition: [radius * nx, radius * ny, radius * nz],
                    VertexNormal  : [nx, ny, nz],
                    VertexTexCoord: [s, t], ..Default::default()
                };
                vertices.push(vertex);
            }
        }

        debug_assert_eq!(vertices.len(), vertices_count);
        vertices
    }

    fn generate_indices(slice_count: usize, stack_count: usize) -> Vec<u32> {

        let indices_count = (slice_count * 2) * (stack_count - 1) * 3;
        let mut indices = Vec::with_capacity(indices_count);

        for i in 0..slice_count {
            let stack_start = i * (stack_count + 1);
            let next_stack_start = stack_start + stack_count;

            for j in 0..stack_count {
                if j == 0 {
                    indices.extend([
                        stack_start,
                        stack_start + 1,
                        next_stack_start + 1,
                    ].into_iter().map(|i| *i as u32));
                } else if j == stack_count - 1 {
                    indices.extend([
                        stack_start + j,
                        stack_start + j + 1,
                        next_stack_start + j,
                    ].into_iter().map(|i| *i as u32));
                } else {
                    indices.extend([
                        stack_start + j,
                        stack_start + j + 1,
                        next_stack_start + j + 1,
                        next_stack_start + j,
                        stack_start + j,
                        next_stack_start + j + 1,
                    ].into_iter().map(|i| *i as u32));
                }
            }
        }

        debug_assert_eq!(indices.len(), indices_count);
        indices
    }
}

impl TriangleMesh for Sphere {
    type VertexType = SphereVertex;
    type IndexType  = u32;

    fn buffers(&self) -> (&glium::VertexBuffer<SphereVertex>, &glium::IndexBuffer<u32>) {
        (&self.vbuffer, &self.ibuffer)
    }
}
