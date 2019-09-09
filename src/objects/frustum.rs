
use glium::backend::Facade;

use crate::drawable::TriangleMesh;
use crate::error::{GLResult, BufferCreationErrorKind};
use crate::{Vec3F, Mat4F};


#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FrustumVertex {
    VertexPosition: [f32; 3], _padding1: f32,
}

#[derive(Debug)]
pub struct Frustum {

    center: Vec3F,
    u: Vec3F,
    v: Vec3F,
    n: Vec3F,

    near : f32,
    far  : f32,
    fovy : f32,
    ar   : f32,

    /// vertex buffer of triangle mesh
    vbuffer: glium::VertexBuffer<FrustumVertex>,
    /// index buffer of triangle mesh
    ibuffer: glium::IndexBuffer<u32>,
}

impl Frustum {

    pub fn new(display: &impl Facade) -> GLResult<Frustum> {

        glium::implement_vertex!(FrustumVertex, VertexPosition);

        let vbuffer = glium::VertexBuffer::empty_immutable(display, 9)
            .map_err(BufferCreationErrorKind::Vertex)?;
        let ibuffer = glium::IndexBuffer::empty_immutable(display, glium::index::PrimitiveType::LinesList, 24)
            .map_err(BufferCreationErrorKind::Index)?;

        let mut frustum = Frustum {
            vbuffer, ibuffer,
            center: Vec3F::zero(), u: Vec3F::zero(), v: Vec3F::zero(), n: Vec3F::zero(),
            fovy: 0.0, ar: 0.0, near: 0.0, far: 0.0,
        };

        frustum.orient(Vec3F::unit_z(), Vec3F::zero(), Vec3F::unit_y());
        frustum.set_perspective(50.0, 1.0, 0.5, 100.0);

        Ok(frustum)
    }

    pub fn orient(&mut self, pos: Vec3F, at: Vec3F, up: Vec3F) {

        self.n = (pos - at).normalized();
        self.u = self.n.cross(up).normalized();
        self.v = self.n.cross(self.u).normalized();

        self.center = pos;
    }

    pub fn set_perspective(&mut self, fovy: f32, ar: f32, near: f32, far: f32) {

        self.fovy = fovy;
        self.ar   = ar;
        self.near = near;
        self.far  = far;

        let dy  = near * (fovy.to_radians() / 2.0).tan();
        let dx  = ar * dy;
        let fdy = far * (fovy.to_radians() / 2.0).tan();
        let fdx = ar * fdy;

        let vertices = [
            FrustumVertex { VertexPosition: [0.0, 0.0, 0.0], ..Default::default() },

            FrustumVertex { VertexPosition: [ dx,  dy, -near], ..Default::default() },
            FrustumVertex { VertexPosition: [-dx,  dy, -near], ..Default::default() },
            FrustumVertex { VertexPosition: [-dx, -dy, -near], ..Default::default() },
            FrustumVertex { VertexPosition: [ dx, -dy, -near], ..Default::default() },
            
            FrustumVertex { VertexPosition: [ fdx,  fdy, -far], ..Default::default() },
            FrustumVertex { VertexPosition: [-fdx,  fdy, -far], ..Default::default() },
            FrustumVertex { VertexPosition: [-fdx, -fdy, -far], ..Default::default() },
            FrustumVertex { VertexPosition: [ fdx, -fdy, -far], ..Default::default() },
        ];

        let indices = [
            0, 5, 0, 6, 0, 7, 0, 8,
            // The near plane
            1, 2, 2, 3, 3, 4, 4, 1,
            // The far plane
            5, 6, 6, 7, 7, 8, 8, 5,
        ];

        self.vbuffer.write(&vertices);
        self.ibuffer.write(&indices);
    }

    pub fn get_view_matrix(&self) -> Mat4F {
        let rot = Mat4F::new(
            self.u.x, self.v.x, self.n.x, 0.0,
            self.u.y, self.v.y, self.n.y, 0.0,
            self.u.z, self.v.z, self.n.z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        let trans = Mat4F::translation_3d(-self.center);
        rot * trans
    }

    pub fn get_inverse_view_matrix(&self) -> Mat4F {
        let rot = Mat4F::new(
            self.u.x, self.u.y, self.u.z, 0.0,
            self.v.x, self.v.y, self.v.z, 0.0,
            self.n.x, self.n.y, self.n.z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        let trans = Mat4F::translation_3d(self.center);
        trans * rot
    }

    pub fn get_projection_matrix(&self) -> Mat4F {
        Mat4F::perspective_rh_zo(self.fovy.to_radians(), self.ar, self.near, self.far)
    }
    
    pub fn get_origin(&self) -> Vec3F {
        self.center
    }
}

impl TriangleMesh for Frustum {
    type VertexType = FrustumVertex;
    type IndexType  = u32;

    fn buffers(&self) -> (&glium::VertexBuffer<FrustumVertex>, &glium::IndexBuffer<u32>) {
        (&self.vbuffer, &self.ibuffer)
    }
}
