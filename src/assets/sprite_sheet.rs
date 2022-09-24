use sdl2::surface::Surface;

struct SpriteSheet<'init> {
    image: Surface<'init>,
    sprite_width: u8,
    sprite_height: u8,
}

enum SpriteSheetError {
    InvalidSpriteSize,
    SheetNotPOT,
}
enum CropError {
    SpriteOutOfBounds,
    SDLError(String),
}
impl From<String> for CropError {
    fn from(error: String) -> Self {
        CropError::SDLError(error)
    }
}
impl SpriteSheet<'_> {
    pub fn new(image: Surface, sprite_width: u8, sprite_height: u8) -> Result<SpriteSheet, SpriteSheetError> {
        if image.width() & (image.width() - 1) != 0 || image.height() & (image.height() - 1) != 0 {
            return Err(SpriteSheetError::SheetNotPOT);
        }
        if sprite_width == 0 || sprite_height == 0 {
            return Err(SpriteSheetError::InvalidSpriteSize);
        }
        if image.width() % sprite_width as u32 != 0 || image.height() % sprite_height as u32 != 0 {
            return Err(SpriteSheetError::InvalidSpriteSize);
        }

        Ok(SpriteSheet {
            image,
            sprite_width,
            sprite_height,
        })
    }

    pub fn crop<'asset>(self, vrow: u32, hrow: u32) -> Result<Surface<'asset>, CropError> {
        if vrow*self.sprite_width as u32 > self.image.width() || hrow*self.sprite_height as u32 > self.image.height()  {
            return Err(CropError::SpriteOutOfBounds)
        }
        let mut out: Surface = Surface::new(self.sprite_width as u32, self.sprite_height as u32, self.image.pixel_format_enum())?;
        self.image.blit(None, &mut out, None)?;
        Ok(out)

    }

}