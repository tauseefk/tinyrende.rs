use std::{fs::File, io::Write, path::Path};

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
    g: 128,
    r: 64,
    a: 255,
};

struct GridPosition {
    x: u16,
    y: u16,
}

impl GridPosition {
    pub fn to_idx(&self, width: u16) -> usize {
        (self.y as usize) * width as usize + (self.x as usize)
    }
}

// images render upside down
// compared to the reference implementation
fn main() -> Result<(), Error> {
    let width: u16 = 64;
    let height: u16 = 64;

    let mut pixel_data = vec![BLACK; (width * height) as usize];
    let a = GridPosition { x: 7, y: 7 };
    let b = GridPosition { x: 12, y: 37 };
    let c = GridPosition { x: 62, y: 53 };

    pixel_data[a.to_idx(width)] = WHITE;
    pixel_data[b.to_idx(width)] = WHITE;
    pixel_data[c.to_idx(width)] = WHITE;

    let frame_buffer: BGRA = BGRA::new(width, height, &pixel_data);

    let mut file = File::create(Path::new("framebuffer.tga"))?;
    file.write(&frame_buffer.into_data()).unwrap();

    Ok(())
}
