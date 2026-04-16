mod grid_position;
mod obj;
mod renderer;

use std::{fs::File, io::Write, path::Path};

use anyhow::Error;
use clap::{Arg, command};
use tgar::{BGRA, PixelBGRA};

use crate::renderer::mesh::render_mesh;

const IMAGE_SIZE: u16 = 512;

const TRANSPARENT: PixelBGRA = PixelBGRA {
    b: 0,
    g: 0,
    r: 0,
    a: 0,
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

    let width: u16 = IMAGE_SIZE;
    let height: u16 = IMAGE_SIZE;

    let mut frame_buffer = vec![TRANSPARENT; width as usize * height as usize];

    render_mesh(Path::new(obj_path), &mut frame_buffer, width, height)?;

    // tgar hard-codes the TGA header's upper-left-origin bit, so viewers
    // treat row 0 as the top of the image. Our projection keeps the mesh's
    // +Y-up convention, so mirror the rows here to compensate for tgar.
    let w = width as usize;
    let h = height as usize;
    for row in 0..h / 2 {
        let top = row * w;
        let bot = (h - 1 - row) * w;
        for col in 0..w {
            frame_buffer.swap(top + col, bot + col);
        }
    }

    let frame_buffer: BGRA = BGRA::new(width, height, &frame_buffer);

    let mut file = File::create(Path::new("framebuffer.tga"))?;
    file.write_all(&frame_buffer.into_data())?;

    Ok(())
}
