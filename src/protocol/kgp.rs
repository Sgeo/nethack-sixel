use crate::{NUM_COLS, protocol::Protocol};

use std::io::Write;

use base64::prelude::*;

pub struct KGP {
    filename: std::ffi::OsString,
    tile_width: u32,
    tile_height: u32
}

impl KGP {
    fn upload(&self, write: &mut dyn Write) -> anyhow::Result<()> {
        write!(write, "\x1B_Gf=100,a=t,t=f,q=2,i=1;{}\x1B\\", BASE64_STANDARD.encode(self.filename.as_encoded_bytes()))?;
        let cursor = std::fs::canonicalize("cursor.png")?;

        write!(write, "\x1B_Gf=100,a=t,t=f,q=2,i=2;{}\x1B\\", BASE64_STANDARD.encode(cursor.as_os_str().as_encoded_bytes()))?;

        Ok(())
    }
}

impl Protocol for KGP {
    fn new(path: &std::ffi::OsStr, write: &mut dyn Write, tile_width: u32, tile_height: u32) -> anyhow::Result<Self> where Self: Sized {
        let this = KGP {
            filename: std::fs::canonicalize(path)?.into(),
            tile_width,
            tile_height
        };

        this.upload(write)?;

        Ok(this)
    }

    fn draw_glyph(&mut self, write: &mut dyn Write, glyph: u32) -> anyhow::Result<()> {
        let row = glyph / crate::NUM_COLS;
        let col = glyph % NUM_COLS; 
        let x = self.tile_width * col;
        let y = self.tile_height * row;
        write!(write, "\x1B_Ga=p,q=2,c=1,r=1,C=0,i=1,z=-2,x={},y={},w={},h={}\x1B\\", x, y, self.tile_width, self.tile_height)?;

        Ok(())
    }

    fn draw_cursor(&mut self, write: &mut dyn Write) -> anyhow::Result<()> {
        write!(write, "\x1B_Ga=p,q=2,c=1,r=1,C=1,i=2,z=-1,p=1\x1B\\")?;
        //write!(write, "\u{2591}\x1B[D")?;

        Ok(())
    }

    fn undraw_cursor(&mut self, write: &mut dyn Write) -> anyhow::Result<()> {
        //write!(write, " \x1B[D")?;

        Ok(())
    }

    fn screen_was_reset(&mut self, write: &mut dyn Write) -> anyhow::Result<()> {
        self.upload(write)
    }

    fn erase_glyph(&mut self, write: &mut dyn Write) -> anyhow::Result<()> {
        Ok(write!(write, "\x1B_Ga=d,d=c\x1B\\")?)
    }
}