use std::{collections::HashMap, path::Path, fs::File, io::BufReader};
use sdl2::render::Texture;

use super::texture_region::TextureRegion;
use serde::Deserialize;
pub struct TextureAtlas<'a> {
    image: Texture<'a>,
	regions: HashMap<String, Region<'a>>,
}

impl<'a> TextureAtlas<'a> {
	pub fn new(image: Texture) -> Self {
		TextureAtlas {
			image,
			regions: HashMap::new(),
		}
	}
	pub fn load<P: AsRef<Path>>(image: Texture, atlas: P) -> Result<TextureAtlas, String> {
		let mut atlas = TextureAtlas::new(image);
		let raw_regions: HashMap<String, RawRegion> = serde_json::from_reader(BufReader::new(File::open(atlas).map_err(|e| e.to_string())?)).map_err(|e| e.to_string())?;
		
		for (name, region) in raw_regions {
			atlas.regions.insert(name, region.);
		}
		
		println!("{:?}", regions);
		Ok(TextureAtlas {
			image,
			regions
		})
	}
}
#[derive(Deserialize)]
enum RawRegion {
	Single(Rect),
	Animation(Rect, Vec<RawRegion>),
	Atlas(Rect, HashMap<String, RawRegion>),
}
impl RawRegion {
	fn set_image(self, image: Texture) -> Region {

	}
}
#[derive(Deserialize)]
struct Rect {
	x: u32,
	y: u32,
	width: u32,
	height: u32,
} 
impl Into<sdl2::rect::Rect> for Rect {
    fn into(self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(
			self.x as i32,
			self.y as i32,
			self.width,
			self.height
		)
    }
}




#[derive(Debug)]
enum Region<'a> {
	Single(TextureRegion<'a>),
	Animation(TextureRegion<'a>, Vec<Region<'a>>),
	Atlas(TextureRegion<'a>, HashMap<String, Region<'a>>),
}
