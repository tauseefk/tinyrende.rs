mod batteries;
mod mat;
mod obj;
mod renderer;

use std::{fs::File, io::Write, path::Path};

use anyhow::Error;
use clap::{Arg, Command, command};
use tgar::{BGRA, PixelBGRA};

use crate::renderer::mesh::{FrameBuffer, render_mesh};

const IMAGE_SIZE: u16 = 512;

const TRANSPARENT: PixelBGRA = PixelBGRA {
    b: 0,
    g: 0,
    r: 0,
    a: 0,
};

const DEFAULT_COLOR: PixelBGRA = PixelBGRA {
    b: 255,
    g: 100,
    r: 0,
    a: 255,
};

fn main() -> Result<(), Error> {
    let matches = command!()
        .subcommand_required(true)
        .subcommand(
            Command::new("mesh")
                .about("render wireframe mesh")
                .arg(Arg::new("path").required(true).index(1)),
        )
        .subcommand(Command::new("triangle").about("render filled triangle"))
        .get_matches();

    let width: u16 = IMAGE_SIZE;
    let height: u16 = IMAGE_SIZE;

    let mut frame_buffer = FrameBuffer {
        data: vec![TRANSPARENT; width as usize * height as usize],
        width,
        height,
    };

    match matches.subcommand() {
        Some(("mesh", sub)) => {
            let path = sub
                .get_one::<String>("path")
                .ok_or_else(|| Error::msg("missing path"))?;
            let mut depth_buffer = vec![f32::NEG_INFINITY; width as usize * height as usize];
            render_mesh(Path::new(path), &mut frame_buffer, &mut depth_buffer)?;
        }
        _ => unreachable!(),
    }

    // y-flip to accomodate tgar's y-down convention
    let w = width as usize;
    let h = height as usize;
    for row in 0..h / 2 {
        let top = row * w;
        let bot = (h - 1 - row) * w;
        for col in 0..w {
            frame_buffer.data.swap(top + col, bot + col);
        }
    }

    let frame_buffer: BGRA = BGRA::new(width, height, &frame_buffer.data);

    let mut file = File::create(Path::new("framebuffer.tga"))?;
    file.write_all(&frame_buffer.into_data())?;

    Ok(())
}
