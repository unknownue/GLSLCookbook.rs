
use glium::backend::Facade;
use glium::{Surface, Program, DrawParameters};
use glium::uniforms::Uniforms;

use crate::drawable::Drawable;
use crate::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use crate::{Vec3F, Mat3F};


#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TeapotPatchVertex {
    VertexPosition: [f32; 3],
}

#[derive(Debug)]
pub struct TeapotPatch {
    /// vertex buffer of triangle mesh
    vbuffer: glium::VertexBuffer<TeapotPatchVertex>,
}

impl TeapotPatch {

    pub fn new(display: &impl Facade) -> GLResult<TeapotPatch> {

        glium::implement_vertex!(TeapotPatchVertex, VertexPosition);

        let vertices = TeapotPatch::generate_patches();

        let vbuffer = glium::VertexBuffer::immutable(display, &vertices)
            .map_err(BufferCreationErrorKind::Vertex)?;

        let teapot = TeapotPatch { vbuffer };
        Ok(teapot)
    }

    fn generate_patches() -> Vec<TeapotPatchVertex> {

        let mut pts = Vec::with_capacity(32 * 16);

        // Build each patch
        // The rim
        TeapotPatch::build_patch_reflect(0, &mut pts, true, true);
        // The body
        TeapotPatch::build_patch_reflect(1, &mut pts, true, true);
        TeapotPatch::build_patch_reflect(2, &mut pts, true, true);
        // The lid
        TeapotPatch::build_patch_reflect(3, &mut pts, true, true);
        TeapotPatch::build_patch_reflect(4, &mut pts, true, true);
        // The bottom
        TeapotPatch::build_patch_reflect(5, &mut pts, true, true);
        // The handle
        TeapotPatch::build_patch_reflect(6, &mut pts, false, true);
        TeapotPatch::build_patch_reflect(7, &mut pts, false, true);
        // The spout
        TeapotPatch::build_patch_reflect(8, &mut pts, false, true);
        TeapotPatch::build_patch_reflect(9, &mut pts, false, true);

        debug_assert_eq!(pts.len(), 32 * 16);

        pts
    }

    fn build_patch_reflect(patch_num: usize, p: &mut Vec<TeapotPatchVertex>, reflect_x: bool, reflect_y: bool) {

        let patch       = TeapotPatch::get_patch(patch_num, false);
        let patch_rev_v = TeapotPatch::get_patch(patch_num, true);

        // Patch without modification
        TeapotPatch::build_patch(&patch_rev_v, p, Mat3F::identity());

        // Patch reflected in x
        if reflect_x {
            let reflect_mat = Mat3F::from_col_arrays([
                [-1.0, 0.0, 0.0],
                [ 0.0, 1.0, 0.0],
                [ 0.0, 0.0, 1.0],
            ]);
            TeapotPatch::build_patch(&patch, p, reflect_mat);
        }

        // Patch reflected in y
        if reflect_y {
            let reflect_mat = Mat3F::from_col_arrays([
                [ 1.0,  0.0, 0.0],
                [ 0.0, -1.0, 0.0],
                [ 0.0,  0.0, 1.0],
            ]);
            TeapotPatch::build_patch(&patch, p, reflect_mat);
        }

        // Patch reflected in x and y
        if reflect_x && reflect_y {
            let reflect_mat = Mat3F::from_col_arrays([
                [-1.0,  0.0, 0.0],
                [ 0.0, -1.0, 0.0],
                [ 0.0,  0.0, 1.0],
            ]);
            TeapotPatch::build_patch(&patch_rev_v, p, reflect_mat);
        }
    }

    fn get_patch(patch_num: usize, reverse_v: bool) -> [[Vec3F; 4]; 4] {

        use crate::objects::teapot_data::{TEAPOT_CP_DATA, TEAPOT_PATCH_DATA};

        let mut patch: [[Vec3F; 4]; 4] = Default::default();

        if reverse_v {
            for (u, v) in iproduct!(0..4, 0..4) {
                patch[u][v] = Vec3F::from(TEAPOT_CP_DATA[TEAPOT_PATCH_DATA[patch_num][u * 4 + (3 - v)]]);
            }
        } else {
            for (u, v) in iproduct!(0..4, 0..4) {
                patch[u][v] = Vec3F::from(TEAPOT_CP_DATA[TEAPOT_PATCH_DATA[patch_num][u * 4 + v]]);
            }
        }

        patch
    }

    fn build_patch(patch: &[[Vec3F; 4]; 4], pts: &mut Vec<TeapotPatchVertex>, reflect: Mat3F) {

        for (i, j) in iproduct!(0..4, 0..4) {

            let vertex = TeapotPatchVertex {
                VertexPosition: (reflect * patch[i][j]).into_array(),
            };

            pts.push(vertex);
        }
    }
}

impl Drawable for TeapotPatch {

    fn render(&self, surface: &mut impl Surface, program: &Program, params: &DrawParameters, uniform: &impl Uniforms) -> GLResult<()> {

        let draw_patches = glium::index::NoIndices(glium::index::PrimitiveType::Patches { vertices_per_patch: 16 });

        surface.draw(&self.vbuffer, draw_patches, program, uniform, params)
            .map_err(GLErrorKind::DrawError)?;
        Ok(())
    }
}
