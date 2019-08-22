
pub use self::teapot::Teapot;
pub use self::torus::Torus;
pub use self::plane::Plane;
pub use self::objmesh::{ObjMesh, ObjMeshConfiguration};
pub use self::cube::Cube;
pub use self::skybox::SkyBox;
pub use self::quad::Quad;
pub use self::sphere::Sphere;

mod teapot;
mod torus;
mod plane;
mod cube;
mod skybox;
mod quad;
mod sphere;

mod teapot_data;
mod objmesh;
