use sdl2::render::Texture;
use std::{collections::HashMap, fs::File, io::BufReader, path::Path, rc::Rc};

use super::texture_region::TextureRegion;
use serde::Deserialize;
pub struct TextureAtlas {
    image: Rc<Texture>,
    regions: HashMap<String, Rc<Region>>,
}

impl TextureAtlas {
    pub fn new(image: Rc<Texture>) -> Self {
        TextureAtlas {
            image,
            regions: HashMap::new(),
        }
    }
    pub fn load<P: AsRef<Path>>(image: Texture, atlas_json: P) -> Result<TextureAtlas, String> {
        let mut atlas = TextureAtlas::new(Rc::new(image));
        let raw_regions: HashMap<String, RawRegion> = serde_json::from_reader(BufReader::new(
            File::open(atlas_json).map_err(|e| e.to_string())?,
        ))
        .map_err(|e| e.to_string())?;

        for (name, region) in raw_regions {
            atlas
                .regions
                .insert(name, Rc::new(region.set_image(atlas.image.clone(), 0, 0)));
        }

        println!("{:?}", atlas.regions);
        Ok(atlas)
    }
    pub fn get_region(&self, details: &str) -> Option<Rc<Region>> {
        self.regions.get(details).cloned()
    }
}
#[derive(Deserialize)]
enum RawRegion {
    Single(Rect),
    Animation(Rect, Vec<RawRegion>),
    Atlas(Rect, HashMap<String, RawRegion>),
}
impl RawRegion {
    fn set_image(self, texture: Rc<Texture>, x_offset: u32, y_offset: u32) -> Region {
        match self {
            Self::Single(mut src) => Region::Single({
                src.x += x_offset;
                src.y += y_offset;

                TextureRegion {
                    texture,
                    src: src.into(),
                }
            }),
            Self::Animation(mut src, raw_frames) => {
                src.x += x_offset;
                src.y += y_offset;
                let mut frames = Vec::new();
                for frame in raw_frames {
                    frames.push(frame.set_image(texture.clone(), src.x, src.y));
                }

                Region::Animation(frames)
            }
            Self::Atlas(mut src, raw_atlas) => {
                src.x += x_offset;
                src.y += y_offset;
                let mut atlas = HashMap::new();
                for (name, region) in raw_atlas {
                    atlas.insert(
                        name,
                        region.set_image(texture.clone(), x_offset + src.x, y_offset + src.y),
                    );
                }

                Region::Atlas(atlas)
            }
        }
    }
}
#[derive(Deserialize)]
struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}
impl From<Rect> for sdl2::rect::Rect {
    fn from(val: Rect) -> Self {
        sdl2::rect::Rect::new(val.x as i32, val.y as i32, val.width, val.height)
    }
}

#[derive(Debug, Clone)]
pub enum Region {
    Single(TextureRegion),
    Animation(Vec<Region>),
    Atlas(HashMap<String, Region>),
}
impl Region {
    pub fn expect_single(&self, reason: &'static str) -> TextureRegion {
        if let Self::Single(region) = self {
            region.to_owned()
        } else {
            panic!("{reason}: {self:?}");
        }
    }
    pub fn expect_animation(&self, reason: &'static str) -> Vec<Region> {
        if let Self::Animation(frames) = self {
            frames.to_owned()
        } else {
            panic!("{reason}: {self:?}");
        }
    }
    pub fn expect_atlas(&self, reason: &'static str) -> HashMap<String, Region> {
        if let Self::Atlas(atlas) = self {
            atlas.to_owned()
        } else {
            panic!("{reason}: {self:?}");
        }
    }
    pub fn unwrap_single(&self) -> TextureRegion {
        if let Self::Single(region) = self {
            region.to_owned()
        } else {
            panic!("unwrap_single failed, was given: {self:?}");
        }
    }
    pub fn unwrap_animation(&self) -> Vec<Region> {
        if let Self::Animation(frames) = self {
            frames.to_owned()
        } else {
            panic!("unwrap_animation failed, was given: {self:?}");
        }
    }
    pub fn unwrap_atlas(&self) -> HashMap<String, Region> {
        if let Self::Atlas(atlas) = self {
            atlas.to_owned()
        } else {
            panic!("unwrap_atlas failed, was given: {self:?}");
        }
    }
}
