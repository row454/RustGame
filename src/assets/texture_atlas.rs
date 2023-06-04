use std::rc::Rc;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use crate::assets::texture_region::TextureRegion;

pub struct TextureAtlas<'asset> {
    pub image: Texture<'asset>,
    sprite_width: u8,
    sprite_height: u8,
    sheet_width: u8,
    sheet_height: u8,
}

pub struct SpriteSheetError(String);

pub struct CropError(String);
impl std::fmt::Display for SpriteSheetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SpriteSheet Error: {}", self.0)
    }
}

impl std::fmt::Display for CropError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Crop Error: {}", self.0)
    }
}
impl TextureAtlas<'_> {
    pub fn new(image: Texture, sprite_width: u8, sprite_height: u8) -> Result<TextureAtlas, SpriteSheetError> {
        let query = image.query();
        if query.width & (query.width - 1) != 0 || query.height & (query.height - 1) != 0 {
            return Err(SpriteSheetError("Sheet dimensions must be a power of 2".to_string()));
        }
        if sprite_width == 0 || sprite_height == 0 {
            return Err(SpriteSheetError("Sprite dimensions must not be 0".to_string()));
        }
        if query.width % sprite_width as u32 != 0 || query.height % sprite_height as u32 != 0 {
            return Err(SpriteSheetError("Sprite dimensions must be a factor of sheet dimensions".to_string()));
        }
        Ok(TextureAtlas {
            image,
            sprite_width,
            sprite_height,
            sheet_width: (query.width / sprite_width as u32) as u8,
            sheet_height: (query.height / sprite_height as u32) as u8,
        })
    }

    pub fn crop(&self, vrow: u8, hrow: u8, width: u8, height: u8) -> Result<TextureRegion, CropError> {

        if vrow+width > self.sheet_width || hrow+height > self.sheet_height  {
            return Err(CropError("Sprite is out of bounds".to_string()));
        }
        Ok(TextureRegion {
            texture: &self.image,
            src: sdl2::rect::Rect::new(
                (vrow * self.sprite_width) as i32,
                (hrow * self.sprite_height) as i32,
                (width * self.sprite_width) as u32,
                (height * self.sprite_height) as u32,
            ),})
    }
}