use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn zero() -> Vector3 {
        Vector3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn len(&self) -> f32 {
        self.norm().sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        let a = self.len().abs();
        Vector3 {
            x: self.x / a,
            y: self.y / a,
            z: self.z / a,
        }
    }

    pub fn dot(&self, oth: &Vector3) -> f32 {
        self.x * oth.x + self.y * oth.y + self.z * oth.z
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, oth: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + oth.x,
            y: self.y + oth.y,
            z: self.z + oth.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, oth: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - oth.x,
            y: self.y - oth.y,
            z: self.z - oth.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, oth: f32) -> Vector3 {
        Vector3 {
            x: self.x * oth,
            y: self.y * oth,
            z: self.z * oth,
        }
    }
}
