use std::collections::HashMap;
use sdl2::render::Texture;

use super::texture_region::{TextureRegion, Drawable};

pub struct TextureAtlas<T: Drawable> {
    image: T,
	regions: HashMap<String, AtlasRegion<T>>,
}

impl<T: Drawable> TextureAtlas<T> {
	pub fn new(image: T) -> Self {
		TextureAtlas {
			image,
			regions: HashMap::new(),
		}
	}
}
enum AtlasRegion<T: Drawable> {
	Single(TextureRegion<T>),
	Animation(Vec<TextureRegion<T>>),
	Atlas(TextureAtlas<T>),
}
