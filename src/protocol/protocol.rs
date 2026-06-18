pub trait Protocol {
    // Application start-up
    fn new(path: &std::fs::OsStr) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized;

    fn draw_glyph(&mut self, glyph: u32) -> Result<(), Box<dyn std::error::Error>>;
    fn draw_cursor(&mut self, glyph: u32) -> Result<(), Box<dyn std::error::Error>>;

    // Not needed for all protocols. Just KGP?
    fn screen_was_reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    // Erase glyph to make way for other text. Not needed for all protocols
    fn erase_glyph(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}