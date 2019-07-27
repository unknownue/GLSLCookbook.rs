
use glium::backend::Facade;

use crate::drawable::TriangleMesh;
use crate::error::{GLResult, BufferCreationErrorKind};


#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PlaneVertex {
    VertexPosition: [f32; 3],
    VertexNormal  : [f32; 3],
    VertexTexCoord: [f32; 2],
    VectexTangent : [f32; 4],
}

#[derive(Debug)]
pub struct Plane {
    /// vertex buffer of triangle mesh
    vbuffer: glium::VertexBuffer<PlaneVertex>,
    /// index buffer of triangle mesh
    ibuffer: glium::IndexBuffer<u32>,
}

impl Plane {

    pub fn new(display: &impl Facade, x_size: f32, z_size: f32, x_divs: usize, z_divs: usize, s_max: f32, t_max: f32) -> GLResult<Plane> {

        glium::implement_vertex!(PlaneVertex, VertexPosition, VertexNormal, VertexTexCoord, VectexTangent);

        let vertices = Plane::generate_vertices(x_size, z_size, x_divs, z_divs, s_max, t_max);
        let indices  = Plane::generate_indices(x_divs, z_divs);

        let vbuffer = glium::VertexBuffer::immutable(display, &vertices)
            .map_err(BufferCreationErrorKind::Vertex)?;
        let ibuffer = glium::IndexBuffer::immutable(display, glium::index::PrimitiveType::TrianglesList, &indices)
            .map_err(BufferCreationErrorKind::Index)?;

        let plane = Plane { vbuffer, ibuffer };
        Ok(plane)
    }

    fn generate_vertices(x_size: f32, z_size: f32, x_divs: usize, z_divs: usize, s_max: f32, t_max: f32) -> Vec<PlaneVertex> {

        let n_points = (x_divs + 1) * (z_divs + 1);
        let mut vertices = Vec::with_capacity(n_points);

        let x2 = x_size / 2.0;
        let z2 = z_size / 2.0;

        let i_factor = z_size / z_divs as f32;
        let j_factor = x_size / x_divs as f32;

        let tex_i = s_max / x_divs as f32;
        let tex_j = t_max / z_divs as f32;

        for i in 0..=z_divs {
            let z = i_factor * i as f32 - z2;
            for j in 0..=x_divs {
                let x = j_factor * j as f32 - x2;

                let vertex = PlaneVertex {
                    VertexPosition: [x, 0.0, z],
                    VertexNormal  : [0.0, 1.0, 0.0],
                    VertexTexCoord: [j as f32 * tex_i, (z_divs - i) as f32 * tex_j],
                    VectexTangent : [1.0, 0.0, 0.0, 1.0],
                };
                vertices.push(vertex);
            }
        }

        debug_assert_eq!(vertices.len(), n_points);

        vertices
    }

    fn generate_indices(x_divs: usize, z_divs: usize) -> Vec<u32> {

        let mut indices = Vec::with_capacity(x_divs * z_divs * 6);

        for i in 0..z_divs {
            let row_start = i * (x_divs + 1);
            let next_row_start = (i + 1) * (x_divs + 1);

            for j in 0..x_divs {
                let triangle_indices = [
                    row_start + j,
                    next_row_start + j,
                    next_row_start + j + 1,
                    row_start + j,
                    next_row_start + j + 1,
                    row_start + j + 1,
                ];
                indices.extend(triangle_indices.into_iter().map(|i| *i as u32));
            }
        }

        debug_assert_eq!(indices.len(), x_divs * z_divs * 6);

        indices
    }
}

impl TriangleMesh for Plane {
    type VertexType = PlaneVertex;
    type IndexType  = u32;

    fn buffers(&self) -> (&glium::VertexBuffer<PlaneVertex>, &glium::IndexBuffer<u32>) {
        (&self.vbuffer, &self.ibuffer)
    }
}
