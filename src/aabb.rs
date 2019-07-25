
use crate::Vec3F;

use std::fmt;
use std::ops::Add;


/// Axis-aligned bounding box
#[derive(Debug, Clone)]
pub struct AABB {
    min: Vec3F,
    max: Vec3F,
}

impl Default for AABB {

    fn default() -> AABB {
        use std::f32::{MAX, MIN};
        AABB {
            min: Vec3F::new(MAX, MAX, MAX),
            max: Vec3F::new(MIN, MIN, MIN),
        }
    }
}

impl AABB {

    /// Create a AABB box between two points.
    pub fn new(point1: &Vec3F, point2: &Vec3F) -> AABB {
        let mut aabb: AABB = Default::default();
        aabb.enclose(point1);
        aabb.enclose(point2);
        aabb
    }

    /// Expend the this bounding box to enclose the point.
    pub fn enclose(&mut self, point: &Vec3F) {
        self.min.x = self.min.x.min(point.x);
        self.min.y = self.min.y.min(point.y);
        self.min.z = self.min.z.min(point.z);

        self.max.x = self.max.x.max(point.x);
        self.max.y = self.max.y.max(point.y);
        self.max.z = self.max.z.max(point.z);
    }

    pub fn diagonal(&self) -> Vec3F {
        self.max - self.min
    }

    pub fn reset(&mut self) {
        (*self) = Default::default();
    }

    pub fn center(&self) -> Vec3F {
        (self.max + self.min) * 0.5
    }
}

impl Add for AABB {
    type Output = Self;

    fn add(self, other: AABB) -> AABB {
        AABB {
            min: Vec3F::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            max: Vec3F::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            )
        }
    }
}

impl fmt::Display for AABB {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AABB: min = ({}, {}, {}), max = ({}, {}, {})", self.min.x, self.min.y, self.min.z, self.max.x, self.max.y, self.max.z)
    }
}
