use std::{char::UNICODE_VERSION, fs::File, io::Write, path::Path};

use anyhow::Error;
use rand::RngExt;
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

    pub fn transpose(&self) -> GridPosition {
        GridPosition {
            x: self.y,
            y: self.x,
        }
    }
}

fn line(
    a: GridPosition,
    b: GridPosition,
    pixel_data: &mut [PixelBGRA],
    width: u16,
    color: PixelBGRA,
) {
    // steep slope
    let steep = a.x.abs_diff(b.x) < a.y.abs_diff(b.y);

    let (a, b) = if steep {
        (a.transpose(), b.transpose())
    } else {
        (a, b)
    };

    let (a, b) = if a.x > b.x { (b, a) } else { (a, b) };

    let mut y = a.y;

    let mut ierror = 0;
    let aby = b.y.abs_diff(a.y);

    for x in a.x..=b.x {
        let coord = if steep {
            GridPosition { x: y, y: x }
        } else {
            GridPosition { x, y }
        };
        pixel_data[coord.to_idx(width)] = color;

        ierror += 2 * aby;
        if ierror > (b.x - a.x) {
            y = if b.y > a.y { y + 1 } else { y - 1 };

            ierror -= 2 * (b.x - a.x);
        }
    }
}

// images render upside down
// compared to the reference implementation
fn main() -> Result<(), Error> {
    let width: u16 = 64;
    let height: u16 = 64;

    let mut frame_buffer = vec![BLACK; (width * height) as usize];

    let mut rng = rand::rng();

    for _ in 0..16_000_000 {
        let a = GridPosition {
            x: rng.random::<u16>() % width,
            y: rng.random::<u16>() % height,
        };
        let b = GridPosition {
            x: rng.random::<u16>() % width,
            y: rng.random::<u16>() % height,
        };

        line(
            a,
            b,
            &mut frame_buffer,
            width,
            PixelBGRA {
                b: rng.random::<u8>(),
                g: rng.random::<u8>(),
                r: rng.random::<u8>(),
                a: 255,
            },
        );
    }

    let frame_buffer: BGRA = BGRA::new(width, height, &frame_buffer);

    let mut file = File::create(Path::new("framebuffer.tga"))?;
    file.write(&frame_buffer.into_data()).unwrap();

    Ok(())
}
