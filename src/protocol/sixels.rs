use std::io::Write;

use anyhow::{Context, bail};
use icy_sixel::EncodeOptions;
use image::ImageReader;

use crate::{NUM_TILES, protocol::Protocol};

pub struct Sixels {
    tile_images: Vec<&'static [u8]>
}

impl Protocol for Sixels {
    fn new(path: &std::ffi::OsStr, tile_width: u32, tile_height: u32) -> anyhow::Result<Self> where Self: Sized {
        let mut this = Sixels {
            tile_images: vec![&[]; NUM_TILES]
        };

        let imagereader = ImageReader::open(path)?;
        let image = imagereader.with_guessed_format().context("Unable to determine tileset format")?.decode()?;

        for y in 0..crate::NUM_ROWS {
            for x in 0..crate::NUM_COLS {
                let tile_image = image.crop_imm(x * tile_width, y * tile_height, tile_width, tile_height);
                let tile_image = tile_image.resize_exact(10, 20, image::imageops::Gaussian);
                let pixels = tile_image.into_rgba8().into_raw();
                let mut sixel = icy_sixel::encoder::sixel_encode(&pixels, 10, 20, &EncodeOptions::default())?;
                crate::sixelfix::remove_newline(&mut sixel);
                this.tile_images[(y * crate::NUM_COLS + x) as usize] = sixel.into_bytes().leak();
            }
        }

        this.tile_images[1469] = b" \x1B[D";
        this.tile_images[1470] = b" \x1B[D";

        Ok(this)
    }

    fn draw_glyph(&mut self, write: &mut dyn Write, glyph: u32) -> anyhow::Result<()> {
        if (glyph as usize) < crate::NUM_TILES {
            write.write_all(self.tile_images[glyph as usize]).context("Unable to write glyph!")?;
            write.write_all(b"\x1B[C")?;
        } else  {
            bail!("Invalid glyph!");
        }

        Ok(())
    }

    fn draw_cursor(&mut self, write: &mut dyn Write, glyph: u32) -> anyhow::Result<()> {
        todo!()
    }
}