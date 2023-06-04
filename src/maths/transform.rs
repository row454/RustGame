use std::ops::{Div, Mul};
use sdl2::render::{Canvas, RenderTarget, Texture};
use crate::assets::texture_region::TextureRegion;
use crate::maths::vector::Vector;
use crate::render::camera::Camera;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub pos: Vector,
    pub rot: f32,
    pub scale: Vector,
}
impl Default for Transform {
    fn default() -> Transform {
        Transform {
            pos: Vector::default(),
            rot: 0.0,
            scale: (1.0, 1.0).into(),
        }
    }
}



impl Transform {
    pub(crate) fn new(pos: Vector, rot: f32, scale: Vector) -> Transform {
        Transform {
            pos,
            rot,
            scale,
        }
    }
    pub fn pos(mut self, pos: Vector) -> Transform {
        self.pos = pos;
        self
    }
    pub fn rot(mut self, rot: f32) -> Transform {
        self.rot = rot;
        self
    }
    pub fn scale(mut self, scale: Vector) -> Transform {
        self.scale = scale;
        self
    }
}
impl Mul for &Transform {
    type Output = Transform;
    fn mul(self, other: &Transform) -> Transform {
        Transform {
            pos: self.pos + other.pos,
            rot: self.rot + other.rot,
            scale: self.scale * other.scale,
        }
    }
}
impl Div for &Transform {
    type Output = Transform;
    fn div(self, other: &Transform) -> Transform {
        Transform {
            pos: self.pos - other.pos,
            rot: self.rot - other.rot,
            scale: self.scale / other.scale,
        }
    }
}
