use std::rc::Rc;
use sdl2::render::Texture;

#[derive(Clone)]
pub struct TextureRegion<'a> {
	texture: Rc<Texture<'a>>,
	src: sdl2::rect::Rect,
}

impl std::fmt::Debug for TextureRegion<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextureRegion").field("src", &self.src).finish()
    }
}


