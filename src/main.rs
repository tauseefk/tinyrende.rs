mod grid_position;
mod line;

use std::{fs, fs::File, io::Write, path::Path};

use anyhow::Error;
use clap::{Arg, command};
use rand::RngExt;
use tgar::{BGRA, PixelBGRA};

use crate::grid_position::GridPosition;
use line::line;

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
    let matches = command!()
        .arg(
            Arg::new("obj")
                .help("path to a .obj file to load")
                .required(true)
                .index(1),
        )
        .get_matches();

    let obj_path = matches
        .get_one::<String>("obj")
        .ok_or_else(|| Error::msg("missing obj path"))?;
    let _obj_src = fs::read_to_string(obj_path)?;

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
