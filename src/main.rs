mod batteries;
mod obj;
mod renderer;

use std::{fs::File, io::Write, path::Path};

use anyhow::Error;
use clap::{Arg, Command, command};
use tgar::{BGRA, PixelBGRA};

use crate::batteries::{GridPosition, Vertex};
use crate::renderer::mesh::render_mesh;
use crate::renderer::triangle::triangle_filled;

const IMAGE_SIZE: u16 = 512;

const TRANSPARENT: PixelBGRA = PixelBGRA {
    b: 0,
    g: 0,
    r: 0,
    a: 0,
};

#[allow(dead_code)]
const BLACK: PixelBGRA = PixelBGRA {
    b: 0,
    g: 0,
    r: 0,
    a: 255,
};

#[allow(dead_code)]
const WHITE: PixelBGRA = PixelBGRA {
    b: 255,
    g: 255,
    r: 255,
    a: 255,
};

#[allow(dead_code)]
const YELLOW: PixelBGRA = PixelBGRA {
    b: 0,
    g: 200,
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

    let mut frame_buffer = vec![TRANSPARENT; width as usize * height as usize];
    let mut z_buffer = vec![0_u8; width as usize * height as usize];

    match matches.subcommand() {
        Some(("mesh", sub)) => {
            let path = sub
                .get_one::<String>("path")
                .ok_or_else(|| Error::msg("missing path"))?;
            render_mesh(
                Path::new(path),
                &mut frame_buffer,
                &mut z_buffer,
                width,
                height,
            )?;
        }
        Some(("triangle", _)) => {
            let a = Vertex {
                position: GridPosition {
                    x: 100,
                    y: 50,
                    z: 13,
                },
                color: RED,
            };
            let b = Vertex {
                position: GridPosition {
                    x: 400,
                    y: 450,
                    z: 128,
                },
                color: GREEN,
            };
            let c = Vertex {
                position: GridPosition {
                    x: 50,
                    y: 350,
                    z: 255,
                },
                color: BLUE,
            };
            triangle_filled(a, b, c, &mut frame_buffer, &mut z_buffer, width, height);
        }
        _ => unreachable!(),
    }

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
