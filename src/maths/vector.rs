use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Default for Vector {
    fn default() -> Vector {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }
}
impl Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl From<(f32, f32)> for Vector {
    fn from(tuple: (f32, f32)) -> Vector {
        Vector {
            x: tuple.0,
            y: tuple.1,
            z: 0.0,
        }
    }
}
impl From<(f32, f32, f32)> for Vector {
    fn from(tuple: (f32, f32, f32)) -> Vector {
        Vector {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, scalar: f32) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}
impl Mul for Vector {
    type Output = Vector;
    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
impl Div for Vector {
    type Output = Vector;
    fn div(self, other: Vector) -> Vector {
        let mut z = 1.0;
        if other.z != 0.0 {
            z = other.z;
        }
        Vector {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / z,
        }
    }
}
impl Div<f32> for Vector {
    type Output = Vector;
    fn div(self, scalar: f32) -> Vector {
        Vector {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Vector {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }
    fn mag_2d(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    fn mag_2d2(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    fn mag(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    fn mag2(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    fn norm(self) -> Vector {
        self * (1.0 / self.mag())
    }
    pub(crate) fn abs(self) -> Vector {
        Vector {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
}
