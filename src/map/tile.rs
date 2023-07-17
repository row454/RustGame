use sdl2::render::WindowCanvas;
use crate::assets::{self, TextureManager};
use crate::maths::transform::Transform;
use crate::maths::vector::Vector;
use crate::render::CanvasExt;

pub const TILE_SIZE: u32 = 16;
pub trait Tile {
    fn is_solid(&self) -> bool;
    fn render(&self, canvas: &mut WindowCanvas, camera_offset: &Transform, x: u32, y: u32) -> Result<(), String>;
}

pub struct Tiles<'asset> {
    pub tiles: Vec<Box<dyn Tile + 'asset>>,
}

impl Tiles<'_> {
    pub fn init<'asset>(texture_manager: TextureManager<>) -> Tiles<'asset> {
        let mut tiles = Tiles { tiles: Vec::new() };
        tiles.tiles.push(Box::new(BasicTile::new(assets.crop_sheet(assets::TILES, 1, 0, 1, 1).unwrap(), false)));
        tiles.tiles.push(Box::new(BasicTile::new(assets.crop_sheet(assets::TILES, 0, 0, 1, 1).unwrap(), true)));
        tiles

    }
}

struct BasicTile {
    texture: TextureRegion<'asset>,
    solid: bool,
}
impl BasicTile {
    const fn new(texture: TextureRegion, solid: bool) -> BasicTile {
        BasicTile {
            texture,
            solid,
        }
    }
}
impl Tile for BasicTile<'_> {
    fn is_solid(&self) -> bool {
        self.solid
    }
    fn render(&self, canvas: &mut WindowCanvas, camera_offset: &Transform, x: u32, y: u32) -> Result<(), String> {
        canvas.copy_transform(&self.texture, &Transform::new(Vector::new((x * TILE_SIZE) as f32, (y * TILE_SIZE) as f32, -1.0), 0.0, (1.0, 1.0).into()), camera_offset)
    }
}

struct ConnectingTile<'asset> {
    sprite: TextureRegion<'asset>,
    solid: bool,
    subtiles: [TextureRegion<'asset>; 16]
}

impl Tile for ConnectingTile<'_> {
    fn is_solid(&self) -> bool {
        self.solid
    }
    fn render(&self, canvas: &mut WindowCanvas, camera_offset: &Transform, x: u32, y: u32) -> Result<(), String> {
        Ok(())
    }
}

