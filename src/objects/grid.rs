
use glium::backend::Facade;
use glium::{Surface, Program, DrawParameters};
use glium::uniforms::Uniforms;

use crate::error::{GLResult, GLErrorKind, BufferCreationErrorKind};
use crate::drawable::Drawable;


#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct GridVertex {
    VertexPosition: [f32; 3], _padding1: f32,
}

#[derive(Debug)]
pub struct Grid {
    /// vertex buffer of triangle mesh
    vbuffer: glium::VertexBuffer<GridVertex>,
}

impl Grid {

    pub fn new(display: &impl Facade, size: f32, n_division: usize) -> GLResult<Grid> {

        glium::implement_vertex!(GridVertex, VertexPosition);

        let vertices = Grid::generate_vertex(size, n_division);

        let vbuffer = glium::VertexBuffer::immutable(display, &vertices)
            .map_err(BufferCreationErrorKind::Vertex)?;

        let grid = Grid { vbuffer };
        Ok(grid)
    }

    fn generate_vertex(size: f32, n_divisions: usize) -> Vec<GridVertex> {

        let size2 = size / 2.0;
        let division_size = size / n_divisions as f32;
        let n_vertices = 4 * (n_divisions + 1);

        let mut vertices = Vec::with_capacity(n_vertices);

        for row in 0..=n_divisions {
            let z = (row as f32 * division_size) - size2;

            vertices.push(GridVertex { VertexPosition: [-size2, 0.0, z], ..Default::default() });
            vertices.push(GridVertex { VertexPosition: [ size2, 0.0, z], ..Default::default() });
        }

        for col in 0..=n_divisions {
            let x = (col as f32 * division_size) - size2;

            vertices.push(GridVertex { VertexPosition: [x, 0.0, -size2], ..Default::default() });
            vertices.push(GridVertex { VertexPosition: [x, 0.0,  size2], ..Default::default() });
        }

        debug_assert_eq!(vertices.len(), 4 * (n_divisions + 1));

        vertices
    }
}

impl Drawable for Grid {

    fn render(&self, surface: &mut impl Surface, program: &Program, params: &DrawParameters, uniform: &impl Uniforms) -> GLResult<()> {

        let draw_lines = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
        surface.draw(&self.vbuffer, draw_lines, program, uniform, params)
            .map_err(GLErrorKind::DrawError)?;
        Ok(())
    }

    fn render_instanced(&self, surface: &mut impl Surface, per_instanced: glium::vertex::PerInstance, program: &Program, params: &DrawParameters, uniform: &impl Uniforms) -> GLResult<()> {

        let draw_lines = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
        surface.draw((&self.vbuffer, per_instanced), &draw_lines, program, uniform, params)
            .map_err(GLErrorKind::DrawError)?;
        Ok(())
    }
}
