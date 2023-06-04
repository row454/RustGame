use sdl2::render::{Canvas, RenderTarget};
use crate::assets::texture_region::TextureRegion;
use crate::maths::transform::Transform;

pub mod camera;
pub trait CanvasExt {
    fn copy_transform(&mut self, texture: &TextureRegion, transform: &Transform, camera_offset: &Transform) -> Result<(), String>;
}
impl<T> CanvasExt for Canvas<T>  where T: RenderTarget {
    fn copy_transform(&mut self, texture: &TextureRegion, transform: &Transform, camera_offset: &Transform) -> Result<(), String> {
        let transform = transform / camera_offset;
        let scale = transform.scale.abs();
        self.copy_ex(
            texture.texture,
            texture.src,
            sdl2::rect::Rect::new(
                transform.pos.x as i32,
                transform.pos.y as i32,
                texture.src.width()*scale.x as u32,
                texture.src.height()*scale.y as u32
            ),
            transform.rot as f64,
            None,
            transform.scale.x.is_sign_negative(),
            transform.scale.y.is_sign_negative()
        )
    }
}