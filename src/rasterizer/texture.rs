use crate::vec3::Color;
use anyhow::Result;
use image::io::Reader as ImageReader;
use image::{ImageFormat, RgbImage};
use std::io::Cursor;

#[derive(Debug, PartialEq, Clone)]
pub struct Texture {
    img: RgbImage,
}

impl Texture {
    pub fn from_bytes(bytes: &[u8]) -> Result<Texture> {
        let img = ImageReader::with_format(Cursor::new(bytes), ImageFormat::Jpeg)
            .decode()?
            .into_rgb8();
        Ok(Texture { img })
    }

    pub fn color_at(&self, u: f64, v: f64) -> Color {
        let x = u * (self.img.width() as f64 - 1.0);
        let y = v * (self.img.height() as f64 - 1.0);
        let [r, g, b] = self.img.get_pixel(x as u32, y as u32).0;
        Color::new(r as f64, g as f64, b as f64)
    }
}
