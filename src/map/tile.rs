use std::collections::HashMap;

use crate::assets;
use crate::assets::texture_atlas::{Region, TextureAtlas};
use crate::assets::texture_region::TextureRegion;
use crate::maths::transform::Transform;
use crate::maths::vector::Vector;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget, Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;

use super::Map;

pub const TILE_SIZE: u32 = 16;
pub trait Tile {
    fn is_solid(&self) -> bool;
    fn get_id(&self) -> usize;
    fn render(&self, canvas: &mut WindowCanvas, x: u32, y: u32, map: &Map) -> Result<(), String>;
}

pub struct Tiles {
    pub tiles: Vec<Box<dyn Tile>>,
}

impl Tiles {
    pub fn init(tiles_atlas: &TextureAtlas) -> Tiles {
        let mut tiles = Tiles { tiles: Vec::new() };
        tiles.tiles.push(Box::new(ConnectingTile::new(
            tiles_atlas.get_region("sheet_wall").unwrap().unwrap_atlas(),
            false,
            0,
        )));
        tiles.tiles.push(Box::new(BasicTile::new(
            tiles_atlas.get_region("floor").unwrap().unwrap_single(),
            true,
            1,
        )));
        tiles
    }
}

struct BasicTile {
    texture: TextureRegion,
    solid: bool,
    id: usize,
}
impl BasicTile {
    fn new(texture: TextureRegion, solid: bool, id: usize) -> BasicTile {
        BasicTile { texture, solid, id }
    }
}
impl Tile for BasicTile {
    fn is_solid(&self) -> bool {
        self.solid
    }
    fn render(&self, canvas: &mut WindowCanvas, x: u32, y: u32, _map: &Map) -> Result<(), String> {
        canvas.copy(
            &self.texture.texture,
            self.texture.src,
            Rect::new(
                (x * TILE_SIZE) as i32,
                (y * TILE_SIZE) as i32,
                TILE_SIZE,
                TILE_SIZE,
            ),
        )
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

struct ConnectingTile {
    solid: bool,
    id: usize,
    texture: ConnectingTexture,
}
struct ConnectingTexture {
    pub all: TextureRegion,
    pub corners: TextureRegion,
    pub horizontal: TextureRegion,
    pub none: TextureRegion,
    pub vertical: TextureRegion,
}

impl ConnectingTexture {
    pub fn from(atlas: HashMap<String, Region>) -> Result<ConnectingTexture, String> {
        Ok(ConnectingTexture {
            all: atlas.get("all").unwrap().unwrap_single(),
            corners: atlas.get("corners").unwrap().unwrap_single(),
            horizontal: atlas.get("horizontal").unwrap().unwrap_single(),
            none: atlas.get("none").unwrap().unwrap_single(),
            vertical: atlas.get("vertical").unwrap().unwrap_single(),
        })
    }
}

impl ConnectingTile {
    fn new(subtiles: HashMap<String, Region>, solid: bool, id: usize) -> ConnectingTile {
        ConnectingTile {
            texture: ConnectingTexture::from(subtiles).unwrap(),
            solid,
            id,
        }
    }
}

impl Tile for ConnectingTile {
    fn is_solid(&self) -> bool {
        self.solid
    }
    fn render(&self, canvas: &mut WindowCanvas, x: u32, y: u32, map: &Map) -> Result<(), String> {
        let mut neighbours = [[false; 3]; 3];

        for y_offset in -1i32..=1i32 {
            for x_offset in -1i32..=1i32 {
                if (x_offset | y_offset) == 0 {
                    continue;
                }
                let same: bool;
                if let Some(neighbour) = map.get_tile(
                    (x_offset + x as i32) as usize,
                    (y_offset + y as i32) as usize,
                ) {
                    same = neighbour.get_id() == self.id;
                } else {
                    same = false;
                }
                neighbours[(y_offset + 1) as usize][(x_offset + 1) as usize] = same;
            }
        }

        const SUBTILE_SIZE: u32 = TILE_SIZE / 2;

        let top_left: &TextureRegion;
        if neighbours[0][1] {
            if neighbours[1][0] {
                if neighbours[0][0] {
                    top_left = &self.texture.none;
                } else {
                    top_left = &self.texture.corners;
                }
            } else {
                top_left = &self.texture.vertical;
            }
        } else if neighbours[1][0] {
            top_left = &self.texture.horizontal;
        } else {
            top_left = &self.texture.all;
        }
        canvas.copy(
            &top_left.texture,
            Rect::new(
                top_left.src.x,
                top_left.src.y,
                top_left.src.width() / 2,
                top_left.src.height() / 2,
            ),
            Rect::new(
                (x * TILE_SIZE) as i32,
                (y * TILE_SIZE) as i32,
                SUBTILE_SIZE,
                SUBTILE_SIZE,
            ),
        )?;

        let top_right: &TextureRegion;
        if neighbours[0][1] {
            if neighbours[1][2] {
                if neighbours[0][2] {
                    top_right = &self.texture.none;
                } else {
                    top_right = &self.texture.corners;
                }
            } else {
                top_right = &self.texture.vertical;
            }
        } else if neighbours[1][2] {
            top_right = &self.texture.horizontal;
        } else {
            top_right = &self.texture.all;
        }
        canvas.copy(
            &top_right.texture,
            Rect::new(
                top_right.src.x + top_right.src.width() as i32 / 2,
                top_right.src.y,
                top_right.src.width() / 2,
                top_right.src.height() / 2,
            ),
            Rect::new(
                (x * TILE_SIZE) as i32 + SUBTILE_SIZE as i32,
                (y * TILE_SIZE) as i32,
                SUBTILE_SIZE,
                SUBTILE_SIZE,
            ),
        )?;

        let bottom_left: &TextureRegion;
        if neighbours[2][1] {
            if neighbours[1][0] {
                if neighbours[2][0] {
                    bottom_left = &self.texture.none;
                } else {
                    bottom_left = &self.texture.corners;
                }
            } else {
                bottom_left = &self.texture.vertical;
            }
        } else if neighbours[1][0] {
            bottom_left = &self.texture.horizontal;
        } else {
            bottom_left = &self.texture.all;
        }
        canvas.copy(
            &bottom_left.texture,
            Rect::new(
                bottom_left.src.x,
                bottom_left.src.y + bottom_left.src.height() as i32 / 2,
                bottom_left.src.width() / 2,
                bottom_left.src.height() / 2,
            ),
            Rect::new(
                (x * TILE_SIZE) as i32,
                (y * TILE_SIZE) as i32 + SUBTILE_SIZE as i32,
                SUBTILE_SIZE,
                SUBTILE_SIZE,
            ),
        )?;

        let bottom_right: &TextureRegion;
        if neighbours[2][1] {
            if neighbours[1][2] {
                if neighbours[2][2] {
                    bottom_right = &self.texture.none;
                } else {
                    bottom_right = &self.texture.corners;
                }
            } else {
                bottom_right = &self.texture.vertical;
            }
        } else if neighbours[1][2] {
            bottom_right = &self.texture.horizontal;
        } else {
            bottom_right = &self.texture.all;
        }
        canvas.copy(
            &bottom_right.texture,
            Rect::new(
                bottom_right.src.x + bottom_right.src.width() as i32 / 2,
                bottom_right.src.y + bottom_right.src.height() as i32 / 2,
                bottom_right.src.width() / 2,
                bottom_right.src.height() / 2,
            ),
            Rect::new(
                (x * TILE_SIZE) as i32 + SUBTILE_SIZE as i32,
                (y * TILE_SIZE) as i32 + SUBTILE_SIZE as i32,
                SUBTILE_SIZE,
                SUBTILE_SIZE,
            ),
        )?;
        Ok(())
    }

    fn get_id(&self) -> usize {
        self.id
    }
}
