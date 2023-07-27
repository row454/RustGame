use crate::map::tile::{Tile, Tiles};
use crate::maths::transform::Transform;
use sdl2::render::WindowCanvas;
use std::fs;
use std::path::Path;

pub mod tile;
pub struct Map<'map> {
    width: u32,
    height: u32,
    tiles: Vec<Vec<&'map dyn Tile>>,
    entities: Vec<EntitySpawn>,
}
impl Map<'_> {
    pub fn new<P>(path: P, tiles: &Tiles) -> Result<Map<'_>, String>
    where
        P: AsRef<Path>,
    {
        if let Ok(map_string) = fs::read_to_string(path) {
            let map_string = map_string;
            let mut map_tiles = Vec::new();
            for line in map_string.lines() {
                let mut row = Vec::new();
                for tile in line.split(' ') {
                    let tile_id = tile.parse::<usize>().map_err(|e| e.to_string())?;
                    let tile = tiles
                        .tiles
                        .get(tile_id)
                        .expect("Tile id out of bounds")
                        .as_ref();
                    row.push(tile);
                }
                map_tiles.push(row);
            }
            let map = Map {
                width: map_tiles[0].len() as u32,
                height: map_tiles.len() as u32,
                tiles: map_tiles,
                entities: Vec::new(),
            };
            Ok(map)
        } else {
            Err("Could not read map file".to_string())
        }
    }
    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        for y in 0..self.height {
            for x in 0..self.width {
                self.tiles[y as usize][x as usize].render(canvas, x, y, self)?;
            }
        }
        Ok(())
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&dyn Tile> {
        self.tiles.get(y)?.get(x).copied()
    }
}
struct EntitySpawn {
    // entity: Entity,
    x: u32,
    y: u32,
}
