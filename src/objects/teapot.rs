
use glium::backend::Facade;

use crate::drawable::TriangleMesh;
use crate::error::{GLResult, BufferCreationErrorKind};
use crate::{Vec3F, Vec4F, Mat3F, Mat4F};


#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TeapotVertex {
    VertexPosition: [f32; 3],
    VertexNormal  : [f32; 3],
    VertexTexCoord: [f32; 2],
}

#[derive(Debug)]
pub struct Teapot {
    /// vertex buffer of triangle mesh
    vbuffer: glium::VertexBuffer<TeapotVertex>,
    /// index buffer of triangle mesh
    ibuffer: glium::IndexBuffer<u32>,
}

impl Teapot {

    pub fn new(display: &impl Facade, grid: usize, lid_transform: &Mat4F) -> GLResult<Teapot> {

        glium::implement_vertex!(TeapotVertex, VertexPosition, VertexNormal, VertexTexCoord);

        let (mut vertices, indices) = Teapot::generate_patches(grid);
        Teapot::move_lid(grid, &mut vertices, lid_transform);

        let vbuffer = glium::VertexBuffer::immutable(display, &vertices)
            .map_err(BufferCreationErrorKind::Vertex)?;
        let ibuffer = glium::IndexBuffer::immutable(display, glium::index::PrimitiveType::TrianglesList, &indices)
            .map_err(BufferCreationErrorKind::Index)?;

        let teapot = Teapot { vbuffer, ibuffer };
        Ok(teapot)
    }

    fn generate_patches(grid: usize) -> (Vec<TeapotVertex>, Vec<u32>) {

        #[allow(non_snake_case)]
        // Pre-compute the basis functions  (Bernstein polynomials) and their derivatives.
        let (B, dB) = Teapot::compute_basic_functions(grid);
        debug_assert_eq!( B.len(), 4 * (grid + 1));
        debug_assert_eq!(dB.len(), 4 * (grid + 1));

        let mut vertices = Vec::with_capacity(32 * (grid + 1) * (grid + 1));
        let mut indices  = Vec::with_capacity(32 * grid * grid * 6);

        // Build each patch
        // The rim
        Teapot::build_patch_reflect(0, &B, &dB, grid, true, true, &mut vertices, &mut indices);
        // The body
        Teapot::build_patch_reflect(1, &B, &dB, grid, true, true, &mut vertices, &mut indices);
        Teapot::build_patch_reflect(2, &B, &dB, grid, true, true, &mut vertices, &mut indices);
        // The lid
        Teapot::build_patch_reflect(3, &B, &dB, grid, true, true, &mut vertices, &mut indices);
        Teapot::build_patch_reflect(4, &B, &dB, grid, true, true, &mut vertices, &mut indices);
        // The bottom
        Teapot::build_patch_reflect(5, &B, &dB, grid, true, true, &mut vertices, &mut indices);
        // The handle
        Teapot::build_patch_reflect(6, &B, &dB, grid, false, true, &mut vertices, &mut indices);
        Teapot::build_patch_reflect(7, &B, &dB, grid, false, true, &mut vertices, &mut indices);
        // The spout
        Teapot::build_patch_reflect(8, &B, &dB, grid, false, true, &mut vertices, &mut indices);
        Teapot::build_patch_reflect(9, &B, &dB, grid, false, true, &mut vertices, &mut indices);

        debug_assert_eq!(vertices.len(), 32 * (grid + 1) * (grid + 1));
        debug_assert_eq!(indices.len(), 32 * grid * grid * 6);

        (vertices, indices)
    }

    #[allow(non_snake_case)]
    fn compute_basic_functions(grid: usize) -> (Vec<f32>, Vec<f32>) {

        let inc = 1.0 / grid as f32;

        let mut B : Vec<f32> = Vec::with_capacity(4 * (grid + 1)); // Pre-computed Bernstein basis functions
        let mut dB: Vec<f32> = Vec::with_capacity(4 * (grid + 1)); // Pre-computed derivitives of basis functions

        for i in 0..=grid {
            let t = i as f32 * inc;
            let t_sqr = t * t;
            let one_minus_t = 1.0 - t;
            let one_minus_t2 = one_minus_t * one_minus_t;

            B.extend(&[
                one_minus_t * one_minus_t2,
                3.0 * one_minus_t2 * t,
                3.0 * one_minus_t  * t_sqr,
                t * t_sqr,
            ]);

            dB.extend(&[
                -3.0 * one_minus_t2,
                -6.0 * t * one_minus_t + 3.0 * one_minus_t2,
                -3.0 * t_sqr + 6.0 * t * one_minus_t,
                 3.0 * t_sqr,
            ]);
        }

        (B, dB)
    }

    #[allow(non_snake_case)]
    fn build_patch_reflect(patch_num: usize, B: &[f32], dB: &[f32], grid: usize, reflect_x: bool, reflect_y: bool, vertices: &mut Vec<TeapotVertex>, indices: &mut Vec<u32>) {

        let patch       = Teapot::get_patch(patch_num, false);
        let patch_rev_v = Teapot::get_patch(patch_num, true);

        // Patch without modification
        Teapot::build_patch(&patch, B, dB, grid, &Mat3F::identity(), true, vertices, indices);

        // Patch reflected in x
        if reflect_x {
            let reflect_mat = Mat3F::from_col_arrays([
                [-1.0, 0.0, 0.0],
                [ 0.0, 1.0, 0.0],
                [ 0.0, 0.0, 1.0],
            ]);
            Teapot::build_patch(&patch_rev_v, B, dB, grid, &reflect_mat, false, vertices, indices);
        }

        // Patch reflected in y
        if reflect_y {
            let reflect_mat = Mat3F::from_col_arrays([
                [ 1.0,  0.0, 0.0],
                [ 0.0, -1.0, 0.0],
                [ 0.0,  0.0, 1.0],
            ]);
            Teapot::build_patch(&patch_rev_v, B, dB, grid, &reflect_mat, false, vertices, indices);
        }

        // Patch reflected in x and y
        if reflect_x && reflect_y {
            let reflect_mat = Mat3F::from_col_arrays([
                [-1.0,  0.0, 0.0],
                [ 0.0, -1.0, 0.0],
                [ 0.0,  0.0, 1.0],
            ]);
            Teapot::build_patch(&patch, B, dB, grid, &reflect_mat, true, vertices, indices);
        }

    }

    fn get_patch(patch_num: usize, reverse_v: bool) -> [[Vec3F; 4]; 4] {

        use crate::objects::teapot_data::{TEAPOT_CP_DATA, TEAPOT_PATCH_DATA};

        let mut patch: [[Vec3F; 4]; 4] = Default::default();

        if reverse_v {
            for u in 0..4 {
                for v in 0..4 {
                    patch[u][v] = Vec3F::from(TEAPOT_CP_DATA[TEAPOT_PATCH_DATA[patch_num][u * 4 + (3 - v)]]);
                }
            }
        } else {
            for u in 0..4 {
                for v in 0..4 {
                    patch[u][v] = Vec3F::from(TEAPOT_CP_DATA[TEAPOT_PATCH_DATA[patch_num][u * 4 + v]]);
                }
            }
        }

        patch
    }

    #[allow(non_snake_case)]
    fn build_patch(patch: &[[Vec3F; 4]; 4], B: &[f32], dB: &[f32], grid: usize, reflect: &Mat3F, invert_normal: bool, vertices: &mut Vec<TeapotVertex>, indices: &mut Vec<u32>) {

        let tc_factor = 1.0 / grid as f32;
        let base_vertices = vertices.len();

        for i in 0..=grid {
            for j in 0..=grid {

                let pt = (*reflect) * Teapot::evaluate(i, j, B, patch);
                let mut norm = (*reflect) * Teapot::evaluate_normal(i, j, B, dB, patch);

                if invert_normal {
                    norm = -norm;
                }

                let vertex = TeapotVertex {
                    VertexPosition: pt.into_array(),
                    VertexNormal  : norm.into_array(),
                    VertexTexCoord: [i as f32 * tc_factor, j as f32 * tc_factor],
                };
                vertices.push(vertex);
            }
        }

        for i in 0..grid {
            let      i_start = i       * (grid + 1) + base_vertices;
            let next_i_start = (i + 1) * (grid + 1) + base_vertices;

            for j in 0..grid {
                let vertices_index = [
                    i_start + j,
                    next_i_start + j + 1,
                    next_i_start + j,

                    i_start + j,
                    i_start + j + 1,
                    next_i_start + j + 1,
                ];
                indices.extend(vertices_index.into_iter().map(|i| *i as u32));
            }
        }

    }

    #[allow(non_snake_case)]
    fn evaluate(grid_u: usize, grid_v: usize, B: &[f32], patch: &[[Vec3F; 4]; 4]) -> Vec3F {
        let mut p = Vec3F::zero();
        for i in 0..4 {
            for j in 0..4 {
                p += patch[i][j] * B[grid_u * 4 + i] * B[grid_v * 4 + j];
            }
        }
        p
    }

    #[allow(non_snake_case)]
    fn evaluate_normal(grid_u: usize, grid_v: usize, B: &[f32], dB: &[f32], patch: &[[Vec3F; 4]; 4]) -> Vec3F {

        let mut du = Vec3F::zero();
        let mut dv = Vec3F::zero();

        for i in 0..4 {
            for j in 0..4 {
                du += patch[i][j] * dB[grid_u * 4 + i] * B[grid_v * 4 + j];
                dv += patch[i][j] * B[grid_u * 4 + i] * dB[grid_v * 4 + j];
            }
        }

        let norm = du.cross(dv);
        if norm.magnitude_squared() > 0.0 {
            norm.normalized()
        } else {
            norm
        }
    }

    fn move_lid(grid: usize, vertices: &mut Vec<TeapotVertex>, lib_transform: &Mat4F) {

        let start = 12 * (grid + 1) * (grid + 1);
        let end   = 20 * (grid + 1) * (grid + 1);

        for i in start..end {
            let dest_vert = &mut vertices[i].VertexPosition;
            let mut vert = Vec4F::new(dest_vert[0], dest_vert[1], dest_vert[2], 1.0);
            vert = (*lib_transform) * vert;
            *dest_vert = [vert.x, vert.y, vert.z];
        }
    }
}

impl TriangleMesh for Teapot {
    type VertexType = TeapotVertex;
    type IndexType  = u32;

    fn buffers(&self) -> (&glium::VertexBuffer<TeapotVertex>, &glium::IndexBuffer<u32>) {
        (&self.vbuffer, &self.ibuffer)
    }
}
