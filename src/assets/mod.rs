use sdl2::image::{LoadSurface, LoadTexture};
use sdl2::render::TextureCreator;
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use crate::assets::texture_region::TextureRegion;
use crate::assets::texture_atlas::TextureAtlas;
mod texture_atlas;
pub mod texture_region;
const SPRITE_SIZE: u8 = 16;
pub const ENTITIES: usize = 0;
pub const ITEMS: usize = 1;
pub const TILES: usize = 2;


pub struct Assets<'asset> {
    sprite_sheets: Vec<TextureAtlas<'asset>>,
}
impl Assets<'_> {
    pub fn init(texture_creator: &TextureCreator<WindowContext>) -> Result<Assets, String> {
        Ok(Assets {
            sprite_sheets: vec! [
                TextureAtlas::new(texture_creator.load_texture("assets/textures/sheet_entities.png")?, SPRITE_SIZE, SPRITE_SIZE).map_err(|e| e.to_string())?,
                TextureAtlas::new(texture_creator.load_texture("assets/textures/sheet_items.png")?, SPRITE_SIZE, SPRITE_SIZE).map_err(|e| e.to_string())?,
                TextureAtlas::new(texture_creator.load_texture("assets/textures/sheet_tiles.png")?, SPRITE_SIZE, SPRITE_SIZE).map_err(|e| e.to_string())?,
        ] })

    }
    pub fn crop_sheet(&self, sheet: usize, vrow: u8, hrow: u8, width: u8, height: u8) -> Result<TextureRegion, String> {
        self.sprite_sheets.get(sheet).ok_or("Out of bounds sheet used, please use the sheet constants")?.crop(vrow, hrow, width, height).map_err(|e| e.to_string())
    }
}

