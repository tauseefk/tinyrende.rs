mod grid_position;
mod line;
mod obj;

use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use anyhow::Error;
use tgar::{BGRA, PixelBGRA};

const BLACK: PixelBGRA = PixelBGRA {
    b: 0,
    g: 0,
    r: 0,
    a: 255,
};

const WHITE: PixelBGRA = PixelBGRA {
    b: 255,
    g: 255,
    r: 255,
    a: 255,
};

const GREEN: PixelBGRA = PixelBGRA {
    b: 0,
    g: 255,
    r: 0,
    a: 255,
};

const RED: PixelBGRA = PixelBGRA {
    b: 0,
    g: 0,
    r: 255,
    a: 255,
};

const BLUE: PixelBGRA = PixelBGRA {
    b: 255,
    g: 128,
    r: 64,
    a: 255,
};

const YELLOW: PixelBGRA = PixelBGRA {
    b: 0,
    g: 200,
    r: 255,
    a: 255,
};

// images render upside down
// compared to the reference implementation
fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let _mesh = obj::parse(stdin.lock())?;

    let width: u16 = 64;
    let height: u16 = 64;

    let frame_buffer = vec![BLACK; (width * height) as usize];
    let frame_buffer: BGRA = BGRA::new(width, height, &frame_buffer);

    let mut file = File::create(Path::new("framebuffer.tga"))?;
    file.write_all(&frame_buffer.into_data())?;

    Ok(())
}
