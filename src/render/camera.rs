use std::fmt::Debug;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use crate::maths::transform::Transform;
use crate::maths::vector::Vector;

pub struct Camera<'camera> {
    // divide all rendered transforms by the transform of the camera to apply the offset
    pub transform: Transform,
    pub camera_size: (u32, u32),
    pub target: Texture<'camera>,
}
impl Camera<'_> {
    pub fn new(transform: Transform, camera_size: (u32, u32), texture_creator: &TextureCreator<WindowContext>) -> Result<Camera, String> {

        Ok(Camera {
            transform,
            camera_size,
            target: texture_creator.create_texture_target(None, camera_size.0, camera_size.1).map_err(|e| e.to_string())?
        })
    }

    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let viewport = canvas.viewport();
        let x_scale = viewport.width() as f32 / self.camera_size.0 as f32;
        let y_scale = viewport.height() as f32 / self.camera_size.1 as f32;
        let scale = if x_scale < y_scale { x_scale } else { y_scale };
        let dst = Rect::new(
            (viewport.width() as f32 / 2.0 - self.camera_size.0 as f32 / 2.0 * scale) as i32,
            (viewport.height() as f32 / 2.0 - self.camera_size.1 as f32 / 2.0 * scale) as i32,
            (self.camera_size.0 as f32 * scale) as u32,
            (self.camera_size.1 as f32 * scale) as u32,
        );
        canvas.copy(&self.target, None, dst)?;
        Ok(())
    }

    pub fn update() {
        // nothing to update currently!!
    }

    pub fn center(&mut self, pos: Vector) {
        self.transform.pos = Vector::new(pos.x-self.camera_size.0 as f32/2.0, pos.y-self.camera_size.1 as f32/2.0, pos.z);
    }

}
