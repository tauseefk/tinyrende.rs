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
    g: 200,
    r: 255,
    a: 255,
};

#[derive(Copy, Clone)]
struct GridPosition {
    x: u16,
    y: u16,
}

impl GridPosition {
    pub fn to_idx(&self, width: u16) -> usize {
        (self.y as usize) * width as usize + (self.x as usize)
    }
}

fn line(
    a: GridPosition,
    b: GridPosition,
    pixel_data: &mut [PixelBGRA],
    width: u16,
    color: PixelBGRA,
) {
    let mut t = 0.;
    for x in a.x..b.x {
        let coord = GridPosition {
            x: ((a.x as f32) + ((b.x as f32) - (a.x as f32)) * t).round() as u16,
            y: ((a.y as f32) + ((b.y) as f32 - (a.y as f32)) * t).round() as u16,
        };
        pixel_data[coord.to_idx(width)] = color;

        t = ((x as f32) - (a.x as f32)) / (b.x as f32 - a.x as f32);
    }
}

// images render upside down
// compared to the reference implementation
fn main() -> Result<(), Error> {
    let width: u16 = 64;
    let height: u16 = 64;

    let mut frame_buffer = vec![BLACK; (width * height) as usize];
    let a = GridPosition { x: 7, y: 7 };
    let b = GridPosition { x: 12, y: 37 };
    let c = GridPosition { x: 62, y: 53 };

    line(a, b, &mut frame_buffer, width, BLUE);
    line(c, b, &mut frame_buffer, width, GREEN);
    line(c, a, &mut frame_buffer, width, YELLOW);
    line(a, c, &mut frame_buffer, width, RED);

    frame_buffer[a.to_idx(width)] = WHITE;
    frame_buffer[b.to_idx(width)] = WHITE;
    frame_buffer[c.to_idx(width)] = WHITE;

    let frame_buffer: BGRA = BGRA::new(width, height, &frame_buffer);

    let mut file = File::create(Path::new("framebuffer.tga"))?;
    file.write(&frame_buffer.into_data()).unwrap();

    Ok(())
}
