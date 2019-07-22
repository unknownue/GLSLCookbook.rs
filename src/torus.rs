
use glium::backend::Facade;

use crate::drawable::TriangleMesh;
use crate::error::{GLResult, BufferCreationErrorKind};
use crate::Vec3F;


#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TorusVertex {
    VertexPosition: [f32; 3],
    VertexNormal  : [f32; 3],
    VertexTexCoord: [f32; 2],
}

#[derive(Debug)]
pub struct Torus {
    /// vertex buffer of triangle mesh
    vbuffer: glium::VertexBuffer<TorusVertex>,
    /// index buffer of triangle mesh
    ibuffer: glium::IndexBuffer<u32>,
}

impl Torus {

    pub fn new(display: &impl Facade, outer_raidus: f32, inner_radius: f32, n_sides: usize, n_rings: usize) -> GLResult<Torus> {

        glium::implement_vertex!(TorusVertex, VertexPosition, VertexNormal, VertexTexCoord);

        let vertices = Torus::generate_vertices(outer_raidus, inner_radius, n_sides, n_rings);
        let indices  = Torus::generate_indices(n_sides, n_rings);

        let vbuffer = glium::VertexBuffer::immutable(display, &vertices)
            .map_err(BufferCreationErrorKind::Vertex)?;
        let ibuffer = glium::IndexBuffer::immutable(display, glium::index::PrimitiveType::TrianglesList, &indices)
            .map_err(BufferCreationErrorKind::Index)?;

        let torus = Torus { vbuffer, ibuffer };
        Ok(torus)
    }

    fn generate_vertices(outer_raidus: f32, inner_radius: f32, n_sides: usize, n_rings: usize) -> Vec<TorusVertex> {

        const TWO_PI: f32 = std::f32::consts::PI * 2.0;

        let n_vertices = n_sides * (n_rings + 1); // One extra ring to duplicate first ring

        let mut vertices = Vec::with_capacity(n_vertices);

        // Generate the vertex data
        let ring_factor = TWO_PI / n_rings as f32;
        let side_factor = TWO_PI / n_sides as f32;

        for ring in 0..=n_rings {
            let u = ring as f32 * ring_factor;
            let cos_u = u.cos();
            let sin_u = u.sin();

            for side in 0..n_sides {
                let v = side as f32 * side_factor;
                let cos_v = v.cos();
                let sin_v = v.sin();

                let r = outer_raidus + inner_radius * cos_v;

                let vertex = TorusVertex {
                    VertexPosition: [r * cos_u, r * sin_u, inner_radius * sin_v],
                    VertexNormal: Vec3F::new(cos_v * cos_u * r, cos_v * sin_u * r, sin_v * r)
                        .normalized().into_array(),
                    VertexTexCoord: [u / TWO_PI, v / TWO_PI],
                };
                vertices.push(vertex);
            }
        }

        vertices
    }

    fn generate_indices(n_sides: usize, n_rings: usize) -> Vec<u32> {

        let faces = n_sides * n_rings;
        let mut indices  = Vec::with_capacity(6 * faces);

        for ring in 0..n_rings {
            let ring_start = ring * n_sides;
            let next_ring_start = (ring + 1) * n_sides;

            for side in 0..n_sides {
                let next_side = (side + 1) % n_sides;
                // The quad
                let vertex_indices = [
                    ring_start + side,
                    next_ring_start + side,
                    next_ring_start + next_side,
                    ring_start + side,
                    next_ring_start + next_side,
                    ring_start + next_side,
                ];
                indices.extend(vertex_indices.into_iter().map(|index| *index as u32));
            }
        }

        indices
    }
}

impl TriangleMesh for Torus {
    type VertexType = TorusVertex;
    type IndexType  = u32;

    fn buffers(&self) -> (&glium::VertexBuffer<TorusVertex>, &glium::IndexBuffer<u32>) {
        (&self.vbuffer, &self.ibuffer)
    }
}
