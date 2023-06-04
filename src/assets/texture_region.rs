use sdl2::rect::Rect;
use sdl2::render::Texture;


pub struct TextureRegion<'asset> {
    pub texture: &'asset Texture<'asset>,
    pub src: Rect,
}

impl<'asset> TextureRegion<'asset> {
    pub(super) fn new(texture: &'asset Texture<'asset>, location: Rect) -> TextureRegion<'asset> {
        TextureRegion {
            texture,
            src: location,
        }
    }
    pub fn as_tuple(&self) -> (&'asset Texture<'asset>, &Rect) {
        (&self.texture, &self.src)
    }
    pub fn crop(&self, subsprite: Rect) -> Result<TextureRegion, String> {
        if subsprite.x < 0 || subsprite.y < 0 || subsprite.x + subsprite.width() as i32 > self.src.width() as i32 || subsprite.y + subsprite.height() as i32  > self.src.height() as i32  {
            return Err(format!("Subsprite out of bounds: {:?}", subsprite));
        }
        Ok(TextureRegion::new(self.texture, sdl2::rect::Rect::new(self.src.x + subsprite.x, self.src.y + subsprite.y, subsprite.width(), subsprite.height())))
    }
}

