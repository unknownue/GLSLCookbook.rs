
use glium::backend::Facade;

use crate::aabb::AABB;
use crate::drawable::TriangleMesh;
use crate::error::{GLResult, GLError, BufferCreationErrorKind};
use crate::{Vec3F, Vec2F};

use std::path::Path;


#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct ObjVertex {
    VertexPosition: [f32; 3], _padding1: f32,
    VertexNormal  : [f32; 3], _padding2: f32,
    VertexTexCoord: [f32; 2], _padding3: [f32; 2],
    VertexTangent : [f32; 4],
}

#[derive(Debug)]
pub struct ObjMesh {
    vbuffer: glium::VertexBuffer<ObjVertex>,
    ibuffer: glium::IndexBuffer<u32>,
}

#[derive(Debug, Clone)]
pub struct ObjMeshConfiguration {
    pub is_with_adjacency: bool,
    pub is_gen_tangents: bool,
    pub is_center: bool,
    pub is_print_load_message: bool,
}

impl ObjMesh {

    pub fn load(display: &impl Facade, path: impl AsRef<Path>, config: ObjMeshConfiguration) -> GLResult<ObjMesh> {

        glium::implement_vertex!(ObjVertex, VertexPosition, VertexNormal, VertexTexCoord, VertexTangent);

        let meshes = ObjMeshData::load(path, &config)?;

        let vbuffer = glium::VertexBuffer::immutable(display, &meshes.vertices)
            .map_err(BufferCreationErrorKind::Vertex)?;

        let ibuffer = if config.is_with_adjacency {
            glium::IndexBuffer::immutable(display, glium::index::PrimitiveType::TrianglesListAdjacency, &meshes.indices)
                .map_err(BufferCreationErrorKind::Index)?
        } else {
            glium::IndexBuffer::immutable(display, glium::index::PrimitiveType::TrianglesList, &meshes.indices)
                .map_err(BufferCreationErrorKind::Index)?
        };

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
}

impl ObjMeshData {

    fn load(path: impl AsRef<Path>, config: &ObjMeshConfiguration) -> GLResult<ObjMeshData> {

        use std::fs::File;
        use std::io::BufReader;
        use std::io::prelude::*;

        let obj_file = File::open(path.as_ref())
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

        if normals.is_empty() {
            // Generate normals
            ObjMeshData::generate_normals(&mut vertices, &indices);
        }

        if config.is_with_adjacency {
            ObjMeshData::generate_faces_to_adjancency_format();
        } else if config.is_gen_tangents {
            // Generate tangents
            ObjMeshData::generate_tangents(&mut vertices, &indices);
        }

        if config.is_center {
            ObjMeshData::center(&mut vertices, bounding_box);
        }

        if config.is_print_load_message {
            ObjMeshData::print_help_message(path, &vertices, &indices);
        }

        let mesh = ObjMeshData { vertices, indices };
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
            | (true, false) => {
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
            | (false, true) => {
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

    fn generate_normals(vertices: &mut Vec<ObjVertex>, indices: &[u32]) {

        let mut normals = vec![Vec3F::zero(); vertices.len()];

        for i in (0..indices.len()).step_by(3) {
            let index1 = indices[i] as usize;
            let index2 = indices[i + 1] as usize;
            let index3 = indices[i + 2] as usize;

            let p1 = Vec3F::from(vertices[index1].VertexPosition);
            let p2 = Vec3F::from(vertices[index2].VertexPosition);
            let p3 = Vec3F::from(vertices[index3].VertexPosition);

            let a = p2 - p1;
            let b = p3 - p1;
            let n = a.cross(b).normalized();

            normals[index1] += n;
            normals[index2] += n;
            normals[index3] += n;
        }

        vertices.iter_mut().zip(normals.into_iter())
            .for_each(|(vertex, normal)| {
                vertex.VertexNormal = normal.normalized().into_array();
            });
    }

    fn generate_tangents(vertices: &mut Vec<ObjVertex>, indices: &[u32]) {

        let mut tan1_accum = Vec::with_capacity(vertices.len());
        let mut tan2_accum = Vec::with_capacity(vertices.len());

        // Compute the tangent vector
        for i in (0..indices.len()).step_by(3) {
            let index1 = indices[i] as usize;
            let index2 = indices[i + 1] as usize;
            let index3 = indices[i + 2] as usize;

            let p1 = Vec3F::from(vertices[index1].VertexPosition);
            let p2 = Vec3F::from(vertices[index2].VertexPosition);
            let p3 = Vec3F::from(vertices[index3].VertexPosition);

            let tc1 = Vec2F::from(vertices[index1].VertexTexCoord);
            let tc2 = Vec2F::from(vertices[index2].VertexTexCoord);
            let tc3 = Vec2F::from(vertices[index3].VertexTexCoord);

            let q1 = p2 - p1;
            let q2 = p3 - p1;

            let s1 = tc2.x - tc1.x;
            let s2 = tc3.x - tc1.x;

            let t1 = tc2.y - tc1.y;
            let t2 = tc3.y - tc1.y;

            let r = 1.0 / (s1 * t2 - s2 * t1);

            let tan1 = Vec3F::new(
                (t2 * q1.x - t1 * q2.x) * r,
                (t2 * q1.y - t1 * q2.y) * r,
                (t2 * q1.z - t1 * q2.z) * r,
            );
            let tan2 = Vec3F::new(
                (s2 * q2.x - s1 * q1.x) * r,
                (s2 * q2.y - s1 * q1.y) * r,
                (s2 * q2.z - s1 * q1.z) * r,
            );

            tan1_accum.push(tan1);
            tan2_accum.push(tan2);
        }

        for i in 0..vertices.len() {
            let n = Vec3F::from(vertices[i].VertexNormal);
            let t1 = tan1_accum[i];
            let t2 = tan2_accum[i];

            // Gram-Schmidt orthogonalize
            let xyz = (t1 - n * n.dot(t1)).normalized();
            // Store handedness in w
            let w = if n.cross(t1).dot(t2) < 0.0 { -1.0 } else { 1.0 };

            vertices[i].VertexTangent = [xyz.x, xyz.y, xyz.z, w];
        }
    }

    fn center(vertices: &mut Vec<ObjVertex>, bounding_box: AABB) {

        // Center of the AABB
        let center = bounding_box.center();

        // Translate center of the AABB to the origin
        for vertex in vertices.iter_mut() {
            vertex.VertexPosition[0] -= center.x;
            vertex.VertexPosition[1] -= center.y;
            vertex.VertexPosition[2] -= center.z;
        }
    }

    fn generate_faces_to_adjancency_format() {
        unimplemented!()
    }

    fn print_help_message(path: impl AsRef<Path>, vertices: &[ObjVertex], indices: &[u32]) {
        println!("-------------------------------------------------------------");
        println!("Load mesh from: {}", path.as_ref().to_str().expect("Invalid Path"));
        println!("\t vertices  count: {}", vertices.len());
        println!("\t triangles count: {}", indices.len() / 3);
        println!("-------------------------------------------------------------");
    }
}
