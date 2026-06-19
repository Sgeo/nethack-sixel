use std::io::Write;


pub mod sixels;
pub mod it2;

pub trait Protocol {
    // Application start-up
    fn new(path: &std::ffi::OsStr, tile_width: u32, tile_height: u32) -> anyhow::Result<Self> where Self: Sized;

    // Automatically advances to next position
    fn draw_glyph(&mut self, write: &mut dyn Write, glyph: u32) -> anyhow::Result<()>;

    fn draw_cursor(&mut self, write: &mut dyn Write, glyph: u32) -> anyhow::Result<()>;

    // Not needed for all protocols. Just KGP?
    fn screen_was_reset(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    // Erase glyph to make way for other text. Not needed for all protocols
    fn erase_glyph(&mut self, write: &mut dyn Write,) -> anyhow::Result<()> {
        Ok(())
    }
}