use std::rc::Rc;

use sdl2::render::Texture;

#[derive(Clone)]
pub struct TextureRegion {
    pub texture: Rc<Texture>,
    pub src: sdl2::rect::Rect,
}

impl std::fmt::Debug for TextureRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TextureRegion")
            .field("src", &self.src)
            .finish()
    }
}
