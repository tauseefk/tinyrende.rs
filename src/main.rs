mod grid_position;
mod line;
mod obj;

use std::{
    fs::File,
    io::{BufReader, Write},
    path::Path,
};

use anyhow::Error;
use clap::{Arg, command};
use tgar::{BGRA, PixelBGRA};

use crate::{grid_position::GridPosition, line::line, obj::Vertex};

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
    let mesh = obj::parse(BufReader::new(File::open(obj_path)?))?;

    let width: u16 = 64;
    let height: u16 = 64;

    let mut frame_buffer = vec![TRANSPARENT; (width * height) as usize];

    let (mut min_x, mut max_x) = (f32::INFINITY, f32::NEG_INFINITY);
    let (mut min_y, mut max_y) = (f32::INFINITY, f32::NEG_INFINITY);
    for v in &mesh.vertices {
        min_x = min_x.min(v.x);
        max_x = max_x.max(v.x);
        min_y = min_y.min(v.y);
        max_y = max_y.max(v.y);
    }
    let span_x = max_x - min_x;
    let span_y = max_y - min_y;

    let project = |v: &Vertex| GridPosition {
        x: ((v.x - min_x) / span_x * (width - 1) as f32) as u16,
        y: ((v.y - min_y) / span_y * (height - 1) as f32) as u16,
    };

    for face in &mesh.faces {
        let n = face.vertices.len();
        for i in 0..n {
            let a = &mesh.vertices[face.vertices[i]];
            let b = &mesh.vertices[face.vertices[(i + 1) % n]];
            line(project(a), project(b), &mut frame_buffer, width, WHITE);
        }
    }

    let frame_buffer: BGRA = BGRA::new(width, height, &frame_buffer);

    let mut file = File::create(Path::new("framebuffer.tga"))?;
    file.write_all(&frame_buffer.into_data())?;

    Ok(())
}
