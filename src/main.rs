use std::io;
use std::io::Read;
use std::io::Write;

use anyhow::{Context, bail};
use image::ImageReader;

mod vt_tiledata_parser;

mod stdout_no_buffer;

mod sixelfix;

mod kgp;

use vt_tiledata_parser::FeedResult;




const NUM_ROWS: u32 = 60;
const NUM_COLS: u32 = 40;
const NUM_TILES: usize = (NUM_ROWS * NUM_COLS) as usize;


fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let args = std::env::args().collect::<Vec<String>>();
    let size_string = args.get(2).context("Tile size needs to be provided")?;
    let sizes = size_string.split("x").collect::<Vec<&str>>();
    if sizes.len() != 2  {
        bail!("Tile size should be WxH");
    }
    let tile_width = sizes[0].parse::<u32>().context("Width needs to be a number")?;
    let tile_height = sizes[1].parse::<u32>().context("Height needs to be a number")?;

    let tileset_filename = args.get(1).context("Missing tileset argument")?;

    let tileset_filename = std::fs::canonicalize(tileset_filename)?.into_os_string();

    // let imagereader = ImageReader::open(?)?;
    // let image = imagereader.with_guessed_format().context("Unable to determine tileset format")?.decode()?;
    // let image_width = image.width();
    // let image_height = image.height();

    // let tileset_pixels = image.into_rgba8().into_raw();

    
    
    
    let stdin_lock = io::stdin().lock();
    //let mut stdout_lock = io::stdout().lock();
    let mut stdout_lock = stdout_no_buffer::stdout();
    let mut parser = vt_tiledata_parser::Parser::new();
    
    kgp::upload_image_filename(&mut stdout_lock, 1, &tileset_filename)?;
    //kgp::upload_image_hardcoded(&mut stdout_lock)?;
    println!("Loaded tileset!");

    // Testing purposes only

    kgp::place_sprite(&mut stdout_lock, 1, 0, 0, tile_width, tile_height)?;
    stdout_lock.write_all(b"\x1B[C")?;

    for byte_result in stdin_lock.bytes() {
        //std::thread::sleep(std::time::Duration::from_micros(100));
        let byte = byte_result?;
        let result = parser.feed(byte);
        match result {
            FeedResult::Byte(byte) => {
                stdout_lock.write_all(&[byte])?;
            },
            FeedResult::Bytes(bytes) => {
                stdout_lock.write_all(bytes.as_slice())?;
                //eprintln!("{:?}", bytes);
                if bytes.starts_with(b"\x1B[") && bytes.ends_with(b"J") {
                    kgp::upload_image_filename(&mut stdout_lock, 1, &tileset_filename)?;
                }
            },
            FeedResult::Glyph(glyph) => {
                if glyph <= NUM_TILES {
                    let tile_row = glyph / (NUM_COLS as usize);
                    let tile_col = glyph % (NUM_COLS as usize);
                    kgp::place_sprite(&mut stdout_lock, 1, tile_width * tile_col as u32, tile_height * tile_row as u32, tile_width, tile_height)?;
                    //kgp::place_sprite(&mut stdout_lock, 1, 0, 0, tile_width, tile_height)?;
                    stdout_lock.write_all(b"\x1B[C")?;
                    //stdout_lock.write_all(&[byte])?;
                } else {
                    stdout_lock.write_all(&[b'?'])?;
                }
            },
            FeedResult::Unknown => {}
        }
    }

    Ok(())
}
