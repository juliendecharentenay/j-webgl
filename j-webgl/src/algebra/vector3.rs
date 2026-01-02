use std::ops::{Add, Mul};

/// A vector in 3D space with dx, dy, dz components
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    dx: f32,
    dy: f32,
    dz: f32,
}

impl Vector3 {
    /// Creates a new Vector3 with the given dx, dy, dz components
    pub fn new(dx: f32, dy: f32, dz: f32) -> Self {
        Vector3 { dx, dy, dz }
    }

    /// Returns a zero vector (dx=0, dy=0, dz=0)
    pub fn zero() -> Self {
        Vector3 { dx: 0.0, dy: 0.0, dz: 0.0 }
    }

    /// Returns a unit vector along the x-axis (dx=1, dy=0, dz=0)
    pub fn x() -> Self {
        Vector3 { dx: 1.0, dy: 0.0, dz: 0.0 }
    }

    /// Returns a unit vector along the y-axis (dx=0, dy=1, dz=0)
    pub fn y() -> Self {
        Vector3 { dx: 0.0, dy: 1.0, dz: 0.0 }
    }

    /// Returns a unit vector along the z-axis (dx=0, dy=0, dz=1)
    pub fn z() -> Self {
        Vector3 { dx: 0.0, dy: 0.0, dz: 1.0 }
    }

    /// Returns the dx component
    pub fn dx(&self) -> f32 {
        self.dx
    }
    
    /// Returns the dy component
    pub fn dy(&self) -> f32 {
        self.dy
    }
    
    /// Returns the dz component
    pub fn dz(&self) -> f32 {
        self.dz
    }

    /// Returns the norm (magnitude) of the vector
    fn norm(&self) -> f32 {
        (self.dx * self.dx + self.dy * self.dy + self.dz * self.dz).sqrt()
    }

    /// Returns a new normalized vector (unit vector with the same direction)
    /// If the vector has zero length, returns a zero vector
    pub fn normalize(&self) -> Self {
        let n = self.norm();
        if n > 0.0 {
            Vector3 {
                dx: self.dx / n,
                dy: self.dy / n,
                dz: self.dz / n,
            }
        } else {
            Vector3::zero()
        }
    }

    /// Computes the dot product of two vectors
    pub fn dot(&self, other: &Vector3) -> f32 {
        self.dx * other.dx + self.dy * other.dy + self.dz * other.dz
    }

    /// Computes the cross product of two vectors
    pub fn cross(&self, other: &Vector3) -> Self {
        Vector3 {
            dx: self.dy * other.dz - self.dz * other.dy,
            dy: self.dz * other.dx - self.dx * other.dz,
            dz: self.dx * other.dy - self.dy * other.dx,
        }
    }
}

// Implementation for v1 = v2 + v3
impl Add for &Vector3 {
    type Output = Vector3;

    fn add(self, other: Self) -> Vector3 {
        Vector3 {
            dx: self.dx + other.dx,
            dy: self.dy + other.dy,
            dz: self.dz + other.dz,
        }
    }
}

// Implementation for v1 = a * v2 (scalar multiplication from the left)
impl Mul<&Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, vector: &Vector3) -> Vector3 {
        Vector3 {
            dx: self * vector.dx,
            dy: self * vector.dy,
            dz: self * vector.dz,
        }
    }
}

// Implementation for v1 = v2 * a (scalar multiplication from the right)
impl Mul<f32> for &Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f32) -> Vector3 {
        Vector3 {
            dx: self.dx * scalar,
            dy: self.dy * scalar,
            dz: self.dz * scalar,
        }
    }
}