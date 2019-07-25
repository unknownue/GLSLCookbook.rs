
use glium::backend::Facade;

use crate::aabb::AABB;
use crate::drawable::TriangleMesh;
use crate::error::{GLResult, GLError, BufferCreationErrorKind};
use crate::Vec3F;

use std::path::Path;


#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct ObjVertex {
    VertexPosition: [f32; 3],
    VertexNormal  : [f32; 3],
    VertexTexCoord: [f32; 2],
    VertexTangent : [f32; 3],
}

#[derive(Debug)]
pub struct ObjMesh {
    vbuffer: glium::VertexBuffer<ObjVertex>,
    ibuffer: glium::IndexBuffer<u32>,
}

impl ObjMesh {

    pub fn load(display: &impl Facade, path: impl AsRef<Path>) -> GLResult<ObjMesh> {

        glium::implement_vertex!(ObjVertex, VertexPosition, VertexNormal, VertexTexCoord, VertexTangent);

        let meshes = ObjMeshData::load(path)?;

        let vbuffer = glium::VertexBuffer::immutable(display, &meshes.vertices)
            .map_err(BufferCreationErrorKind::Vertex)?;
        let ibuffer = glium::IndexBuffer::immutable(display, glium::index::PrimitiveType::TrianglesList, &meshes.indices)
            .map_err(BufferCreationErrorKind::Index)?;

        let obj_mesh = ObjMesh { vbuffer, ibuffer };
        Ok(obj_mesh)
    }
}

impl TriangleMesh for ObjMesh {
    type VertexType = ObjVertex;
    type IndexType  = u32;

    fn buffers(&self) -> (&glium::VertexBuffer<ObjVertex>, &glium::IndexBuffer<u32>) {
        (&self.vbuffer, &self.ibuffer)
    }
}


struct ObjMeshData {
    vertices: Vec<ObjVertex>,
    indices : Vec<u32>,
    bounding_box: AABB,
}

impl ObjMeshData {

    fn load(path: impl AsRef<Path>) -> GLResult<ObjMeshData> {

        use std::fs::File;
        use std::io::BufReader;
        use std::io::prelude::*;

        let obj_file = File::open(path)
            .map_err(GLError::io)?;
        let reader = BufReader::new(obj_file);

        let mut texcoords = Vec::new();
        let mut normals   = Vec::new();
        let mut bounding_box: AABB = Default::default();

        let mut vertices = Vec::new();
        let mut indices  = Vec::new();

        for line in reader.lines() {
            // TODO: Handle unwrap()
            let line = line.unwrap();

            let mut line_splits = line.split_ascii_whitespace();
            if let Some(property) = line_splits.next() {
                match property {
                    | "v" => {
                        let mut vertex: ObjVertex = Default::default();
                        let mut pos = Vec3F::zero();
                        // TODO: Handle unwrap()
                        pos.x = line_splits.next().and_then(|x| x.parse().ok()).unwrap();
                        pos.y = line_splits.next().and_then(|y| y.parse().ok()).unwrap();
                        pos.z = line_splits.next().and_then(|z| z.parse().ok()).unwrap();
                        bounding_box.enclose(&pos);
                        vertex.VertexPosition = pos.into_array();
                        vertices.push(vertex);
                    },
                    | "vt" => {
                        let mut tex: [f32; 2] = Default::default();
                        tex[0] = line_splits.next().and_then(|u| u.parse().ok()).unwrap();
                        tex[1] = line_splits.next().and_then(|v| v.parse().ok()).unwrap();
                        texcoords.push(tex);
                    },
                    | "vn" => {
                        let mut nor: [f32; 3] = Default::default();
                        nor[0] = line_splits.next().and_then(|x| x.parse().ok()).unwrap();
                        nor[1] = line_splits.next().and_then(|y| y.parse().ok()).unwrap();
                        nor[2] = line_splits.next().and_then(|z| z.parse().ok()).unwrap();
                        normals.push(nor);
                    },
                    | "f" => {
                        let triangle_indices = ObjMeshData::read_face(&mut line_splits, &mut vertices, &texcoords, &normals)?;
                        indices.extend(&triangle_indices);
                    },
                    | _ => {}
                }
            }
        }

        let mesh = ObjMeshData { vertices, indices, bounding_box };
        Ok(mesh)
    }

    fn read_face<'a>(line_splits: &mut impl Iterator<Item=&'a str>, vertices: &mut Vec<ObjVertex>, texcoords: &[[f32; 2]], normals: &[[f32; 3]]) -> GLResult<[u32; 3]> {

        let mut triangle_indices: [u32; 3] = Default::default();

        match (texcoords.is_empty(), normals.is_empty()) {
            | (false, false) => {
                for i in 0..3 {
                    // TODO: Handle unwrap()
                    let mut indices_split = line_splits.next().unwrap().split('/');
                    let pos_index = indices_split.next().and_then(|i| i.parse::<usize>().ok()).unwrap() - 1;
                    let tex_index = indices_split.next().and_then(|i| i.parse::<usize>().ok()).unwrap() - 1;
                    let nor_index = indices_split.next().and_then(|i| i.parse::<usize>().ok()).unwrap() - 1;

                    let dest_vertex = &mut vertices[pos_index];
                    dest_vertex.VertexNormal   =   normals[nor_index];
                    dest_vertex.VertexTexCoord = texcoords[tex_index];
                    triangle_indices[i] = pos_index as u32;
                }
            },
            | (false, true) => {
                for i in 0..3 {
                    // TODO: Handle unwrap()
                    let mut indices_split = line_splits.next().unwrap().split('/');
                    let pos_index = indices_split.next().and_then(|i| i.parse::<usize>().ok()).unwrap() - 1;
                    let nor_index = indices_split.next().and_then(|i| i.parse::<usize>().ok()).unwrap() - 1;

                    let dest_vertex = &mut vertices[pos_index];
                    dest_vertex.VertexNormal = normals[nor_index];
                    triangle_indices[i] = pos_index as u32;
                }
            },
            | (true, false) => {
                for i in 0..3 {
                    // TODO: Handle unwrap()
                    let mut indices_split = line_splits.next().unwrap().split('/');
                    let pos_index = indices_split.next().and_then(|i| i.parse::<usize>().ok()).unwrap() - 1;
                    let tex_index = indices_split.next().and_then(|i| i.parse::<usize>().ok()).unwrap() - 1;

                    let dest_vertex = &mut vertices[pos_index];
                    dest_vertex.VertexTexCoord = texcoords[tex_index];
                    triangle_indices[i] = pos_index as u32;
                }
            },
            | (true, true) => {
                for i in 0..3 {
                    // TODO: Handle unwrap()
                    let mut indices_split = line_splits.next().unwrap().split('/');
                    let pos_index = indices_split.next().and_then(|i| i.parse::<usize>().ok()).unwrap() - 1;
                    triangle_indices[i] = pos_index as u32;
                }
            },
        }

        Ok(triangle_indices)
    }
}
