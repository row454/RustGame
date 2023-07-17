use std::rc::Rc;

use sdl2::render::Texture;

#[derive(Clone)]
pub struct TextureRegion<T: Drawable> {
	texture: Rc<T>,
	src: sdl2::rect::Rect,
}

pub trait Drawable {
	fn get_texture(&self) -> &Texture;
}

impl Drawable for Texture<'_> {
	fn get_texture(&self) -> &Texture {
		self
	}
}
impl<T: Drawable> Drawable for TextureRegion<T> {
	fn get_texture(&self) -> &Texture {
		self.texture.get_texture()
	}
}
