use std::{ffi::OsStr, io::{self, Write}};

use base64::prelude::*;

pub fn upload_image<W: Write>(write: &mut W, id: u32, width: u32, height: u32, data: &[u8]) -> Result<(), io::Error> {

    let b64 = BASE64_STANDARD.encode(data);

    let b64_chunks = b64.as_bytes().chunks(4096).map(|bytes| unsafe { str::from_utf8_unchecked(bytes)}).collect::<Vec<&str>>();

    if b64_chunks.len() == 1 {
        write!(write, "\x1B_Ga=t,f=32,q=2,i={},s={},v={};{}\x1B\\", id, width, height, b64)?;
    } else {
        for (chunk_num, chunk) in b64_chunks.iter().enumerate() {
            if chunk_num == 0 {
                write!(write, "\x1B_Ga=t,f=32,q=2,m=1,i={},s={},v={};{}\x1B\\", id, width, height, chunk)?;
            } else if chunk_num < b64_chunks.len() - 1 {
                write!(write, "\x1B_Gm=1,q=2;{}\x1B\\", chunk)?
            } else {
                write!(write, "\x1B_Gm=0,q=2;{}\x1B\\", chunk)?
            }
        }
    }

    Ok(())


}

pub fn upload_image_hardcoded<W: Write>(write: &mut W) -> Result<(), io::Error> {
    write!(write, "\x1B_Gf=100,a=t,t=f,q=2,i=1;{}\x1B\\",BASE64_STANDARD.encode("D:\\nethack-sixel\\geoduck25x15-2026.png"))
}

pub fn upload_image_filename<W: Write>(write: &mut W, id: u32, filename: &OsStr) -> Result<(), io::Error> {
    write!(write, "\x1B_Gf=100,a=t,t=f,q=2,i={};{}\x1B\\", id, BASE64_STANDARD.encode(filename.as_encoded_bytes()))
}

pub fn place_sprite<W: Write>(write: &mut W, image_id: u32, left: u32, top: u32, width: u32, height: u32) -> Result<(), io::Error> {
    write!(write, "\x1B_Ga=p,q=2,c=1,r=1,C=1,i={},x={},y={},w={},h={}\x1B\\", image_id, left, top, width, height)
    //write!(write, "\x1B_Ga=p,q=2,c=1,r=1,C=1,i={}\x1B\\", image_id)
}