use std::ops::{Add, Sub};
use super::vector3::Vector3;

/// A point in 3D space with x, y, z coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Point3 {
    /// Create a new Point3 representing the origin
    pub fn origin() -> Self {
        Point3::new(0.0, 0.0, 0.0)
    }

    /// Creates a new Point3 with the given x, y, z coordinates
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point3 { x, y, z }
    }

    /// Returns the x coordinate
    pub fn x(&self) -> f32 {
        self.x
    }

    /// Returns the y coordinate
    pub fn y(&self) -> f32 {
        self.y
    }

    /// Returns the z coordinate
    pub fn z(&self) -> f32 {
        self.z
    }
}

// Implementation for v1 = p2 - p1 (vector from p1 to p2)
impl Sub<&Point3> for &Point3 {
    type Output = Vector3;

    fn sub(self, other: &Point3) -> Vector3 {
        Vector3::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

// Implementation for p1 = p2 + v1 (point plus vector)
impl Add<&Vector3> for &Point3 {
    type Output = Point3;

    fn add(self, vector: &Vector3) -> Point3 {
        Point3 {
            x: self.x + vector.dx(),
            y: self.y + vector.dy(),
            z: self.z + vector.dz(),
        }
    }
}