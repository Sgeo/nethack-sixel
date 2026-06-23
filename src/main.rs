use std::ffi::OsString;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;

use anyhow::{Context, bail};
use image::ImageReader;
use icy_sixel::EncodeOptions;

mod vt_tiledata_parser;

mod stdout_no_buffer;

mod sixelfix;

mod protocol;

use vt_push_parser::VTPushParser;
use vt_push_parser::event::CSI;
use vt_push_parser::event::VTEvent;
use vt_push_parser::event::VTEvent::Csi;
use vt_tiledata_parser::FeedResult;

use std::fs::File;

use crate::protocol::Protocol;



const NUM_ROWS: u32 = 60;
const NUM_COLS: u32 = 40;
const NUM_TILES: usize = (NUM_ROWS * NUM_COLS) as usize;

fn generate_cursor_pixels(r: u8, b: u8, g: u8, width: usize, height: usize) -> Vec<u8> {
    let mut pixels: Vec<u8> = vec![0x00; width * height * 4];
    for i in 0..width {
        pixels[4 * i] = r;
        pixels[4 * i + 1] = g;
        pixels[4 * i + 2] = b;
        pixels[4 * i + 3] = 255;

        pixels[4 * width * (height - 1) + 4 * i] = r;
        pixels[4 * width * (height - 1) + 4 * i + 1] = g;
        pixels[4 * width * (height - 1) + 4 * i + 2] = b;
        pixels[4 * width * (height - 1) + 4 * i + 3] = 255;
    }

    for i in 0..height {
        pixels[4 * width * i] = r;
        pixels[4 * width * i + 1] = g;
        pixels[4 * width * i + 2] = b;
        pixels[4 * width * i + 3] = 255;

        pixels[4 * width * i + 4 * (width - 1)] = r;
        pixels[4 * width * i + 4 * (width - 1) + 1] = g;
        pixels[4 * width * i + 4 * (width - 1) + 2] = b;
        pixels[4 * width * i + 4 * (width - 1) + 3] = 255;
    }

    pixels
}


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

    let tiles_filename: OsString = args.get(1).context("Missing tileset argument")?.into();

    
    // let mut tile_images: [&'static [u8]; NUM_TILES] = [&[]; NUM_TILES];
    
    
    // let mut green_cursor = icy_sixel::encoder::sixel_encode(&generate_cursor_pixels(0, 0, 255, 10 as usize, 20 as usize), 10 as usize, 20 as usize, &EncodeOptions::default())?;
    // sixelfix::remove_newline(&mut green_cursor);
    
    // let mut black_cursor = icy_sixel::encoder::sixel_encode(&generate_cursor_pixels(0, 0, 0, 10 as usize, 20 as usize), 10 as usize, 20 as usize, &EncodeOptions::default())?;
    // sixelfix::remove_newline(&mut black_cursor);
    
    println!("Loaded sixels!");
    
    
    let stdin_lock = io::stdin().lock();
    //let mut stdout_lock = io::stdout().lock();
    let mut stdout_lock = stdout_no_buffer::stdout();
    let mut parser = VTPushParser::new();
    
    let mut protocol = protocol::kgp::KGP::new(&tiles_filename, &mut stdout_lock, tile_width, tile_height)?;
    // Testing purposes only

    // stdout_lock.write_all(tile_images[0])?;
    // stdout_lock.write_all(b"\x1B[C")?;
    // stdout_lock.write_all(tile_images[1])?;

    let mut current_glyph: Option<u32> = None;


    for byte_result in stdin_lock.bytes() {
        //std::thread::sleep(std::time::Duration::from_millis(1));
        let byte = byte_result?;
        match current_glyph {
            None => {
                parser.feed_with(&[byte], |event: VTEvent| {
                    protocol.undraw_cursor(&mut stdout_lock);
                    match event {
                        VTEvent::Raw(bytes) => {
                            for byte in bytes {
                                if &32 <= byte && byte <= &127 {
                                    protocol.erase_glyph(&mut stdout_lock);
                                }
                                stdout_lock.write_all(&[*byte]);
                            }
                        },
                        VTEvent::Csi(CSI {final_byte: b'J', ..})  => {
                            protocol.screen_was_reset(&mut stdout_lock);
                        },
                        VTEvent::Csi(csi@CSI {final_byte: b'z', ..}) => {
                            let params: Vec<Result<u32, _>> = csi.params.into_iter().map(|param| String::from_utf8_lossy(param).parse()).collect();

                            if let (Some(Ok(1)), Some(Ok(0)), Some(Ok(glyph_num)), Some(Ok(flags))) = (params.get(0), params.get(1), params.get(2), params.get(3)) {
                                current_glyph = Some(*glyph_num as u32);
                            }
                        },
                        event => {
                            event.write_to(&mut stdout_lock);
                        }
                    }
                    protocol.draw_cursor(&mut stdout_lock);
                });
            },
            Some(glyph) => {
                parser.feed_with(&[byte], |event: VTEvent| {
                    match event {
                        VTEvent::Csi(csi@CSI {final_byte: b'z', ..}) => {
                            let params: Vec<Result<u32, _>> = csi.params.into_iter().map(|param| String::from_utf8_lossy(param).parse()).collect();
                            if let (Some(Ok(1)), Some(Ok(1))) = (params.get(0), params.get(1)) {
                                current_glyph = None;
                                if glyph <= NUM_TILES as u32 {
                                    protocol.draw_glyph(&mut stdout_lock, glyph);
                                }
                            }
                        },
                        _ => {}
                    }
                });
            }
        }

        //stdout_lock.write_all(&black_cursor.as_bytes())?;
    }

    Ok(())
}
