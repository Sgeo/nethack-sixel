use std::io::{Cursor, Write};

use base64::prelude::*;

use anyhow::Context;
use image::ImageReader;

use crate::protocol::Protocol;

pub struct IT2 {
    tiledata: Vec<&'static [u8]>
}

impl Protocol for IT2 {
    fn new(path: &std::ffi::OsStr, tile_width: u32, tile_height: u32) -> anyhow::Result<Self> where Self: Sized {
        let mut this = IT2 {
            tiledata: vec![]
        };

        let imagereader = ImageReader::open(path)?;
        let image = imagereader.with_guessed_format().context("Unable to determine tileset format")?.decode()?;

        for y in 0..crate::NUM_ROWS {
            for x in 0..crate::NUM_COLS {
                let tile_image = image.crop_imm(x * tile_width, y * tile_height, tile_width, tile_height);
                let mut it2_data = Vec::new();
                let image_data = Vec::new();
                let mut cursor = Cursor::new(image_data);
                tile_image.write_to(&mut cursor, image::ImageFormat::Png)?;
                let tile_image_string = BASE64_STANDARD.encode(cursor.into_inner());
                write!(&mut it2_data, "\x1B]1337;File=width=1;height=1;preserveAspectRatio=0;inline=1:{}\x1B\\", tile_image_string)?;
                this.tiledata.push(it2_data.leak());
            }
        }

        Ok(this)

    }

    fn draw_glyph(&mut self, write: &mut dyn std::io::prelude::Write, glyph: u32) -> anyhow::Result<()> {
        write.write_all(self.tiledata[glyph as usize])?;
        //write.write_all(b"\x1B[A")?; // IT2 puts the cursor to the right and below the image. Just need to travel up.
        Ok(())
    }

    fn draw_cursor(&mut self, write: &mut dyn std::io::prelude::Write, glyph: u32) -> anyhow::Result<()> {
        // Unneeded for this protocol once transparency applied
        Ok(())
    }
}