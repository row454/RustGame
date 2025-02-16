use const_format::concatcp;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use self::texture_atlas::TextureAtlas;
pub mod texture_atlas;
pub mod texture_region;

const ASSETS_LOCATION: &str = "assets/";
const TEXTURE_LOCATION: &str = concatcp!(ASSETS_LOCATION, "textures/");

pub struct ResourceManager<'asset, K, R, L>
where
    K: Hash + Eq,
    L: ResourceLoader<'asset, R>,
{
    loader: &'asset L,
    cache: HashMap<K, Rc<R>>,
}

impl<'asset, K, R, L> ResourceManager<'asset, K, R, L>
where
    K: Hash + Eq,
    L: ResourceLoader<'asset, R>,
{
    pub fn new(loader: &'asset L) -> Self {
        ResourceManager {
            loader,
            cache: HashMap::new(),
        }
    }

    pub fn load<D>(&mut self, details: &D) -> Result<Rc<R>, String>
    where
        L: ResourceLoader<'asset, R, Args = D>,
        D: Eq + Hash + ?Sized,
        K: Borrow<D> + for<'a> From<&'a D>,
    {
        if let Some(resource) = self.cache.get(details).cloned() {
            return Ok(resource);
        }
        let resource = Rc::new(self.loader.load(details)?);
        self.cache.insert(details.into(), resource.clone());
        Ok(resource)
    }
}
pub type TextureManager<'asset, T> = ResourceManager<'asset, String, Texture, TextureCreator<T>>;

pub type TextureAtlasManager<'asset, T> =
    ResourceManager<'asset, String, TextureAtlas, TextureCreator<T>>;

pub trait ResourceLoader<'asset, R> {
    type Args: ?Sized;
    fn load(&'asset self, data: &Self::Args) -> Result<R, String>;
}

impl<'asset, T> ResourceLoader<'asset, Texture> for TextureCreator<T> {
    type Args = str;

    fn load(&'asset self, data: &Self::Args) -> Result<Texture, String> {
        self.load_texture(data)
    }
}

impl<T> ResourceLoader<'_, TextureAtlas> for TextureCreator<T> {
    type Args = str;

    fn load(&'_ self, data: &Self::Args) -> Result<TextureAtlas, String> {
        let image = self.load_texture(TEXTURE_LOCATION.to_owned() + "sheet_" + data + ".png")?;

        TextureAtlas::load(image, TEXTURE_LOCATION.to_owned() + data + ".json")
    }
}
