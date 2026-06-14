use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;

use anyhow::Context;

mod vt_tiledata_parser;

mod stdout_no_buffer;

use vt_tiledata_parser::FeedResult;

use std::fs::File;



const NUM_TILES: usize = 60*40;

struct EscTransformer {
    buffer: Vec<u8>,
    inside_esc: bool,
    inside_hidden: bool
}

impl EscTransformer {
    fn new() -> Self {
        EscTransformer { buffer: Vec::new(), inside_esc: false, inside_hidden: false }
    }

    fn transform<R: Read, W: Write>(&mut self, reader: &mut R, writer: &mut W) {
        
    }

}

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let mut tile_images: [&'static [u8]; NUM_TILES] = [&[]; NUM_TILES];
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_str().context("Unable to use to_str")?;
        if file_name.starts_with("tile_") && file_name.ends_with(".sixel") {
            //println!("Found {}", file_name);
            let maybe_number: Result<usize, _> = file_name.strip_prefix("tile_").context("Missing tile_ prefix")?.strip_suffix(".sixel").context("Missing .sixel suffix")?.parse();
            if maybe_number.is_err() {
                continue;
            }
            let number = maybe_number.expect("We skipped err, how did this happen?");

            if number < NUM_TILES {
                let data = fs::read(entry.path())?;
                tile_images[number] = data.leak();
            }

        }
    }
    println!("Loaded sixels!");


    let stdin_lock = io::stdin().lock();
    //let mut stdout_lock = io::stdout().lock();
    let mut stdout_lock = stdout_no_buffer::stdout();
    let mut parser = vt_tiledata_parser::Parser::new();
    for byte_result in stdin_lock.bytes() {
        let byte = byte_result?;
        let result = parser.feed(byte);
        match result {
            FeedResult::Byte(byte) => {
                stdout_lock.write_all(&[byte])?;
            },
            FeedResult::Bytes(bytes) => {
                stdout_lock.write_all(bytes.as_slice())?;
            },
            FeedResult::Glyph(glyph) => {
                if glyph <= NUM_TILES {
                    stdout_lock.write_all(tile_images[glyph])?;
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
