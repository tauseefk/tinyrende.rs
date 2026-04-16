use std::{fs::File, io::BufReader, path::Path};

use anyhow::Error;
use tgar::PixelBGRA;

use crate::renderer::triangle::triangle_filled;
use crate::{
    grid_position::GridPosition,
    obj::{self, Vertex},
};

fn project(v: &Vertex, width: u16, height: u16) -> GridPosition {
    GridPosition {
        x: ((v.x + 1.0) * (width / 2) as f32) as u16,
        y: ((v.y + 1.0) * (height / 2) as f32) as u16,
    }
}

pub fn render_mesh(
    path: &Path,
    frame_buffer: &mut [PixelBGRA],
    width: u16,
    height: u16,
) -> Result<(), Error> {
    let mesh = obj::parse(BufReader::new(File::open(path)?))?;

    for face in &mesh.faces {
        let a = &mesh.vertices[face.vertices[0]];
        let b = &mesh.vertices[face.vertices[1]];
        let c = &mesh.vertices[face.vertices[2]];

        let a = project(a, width, height);
        let b = project(b, width, height);
        let c = project(c, width, height);

        let color = PixelBGRA {
            b: rand::random(),
            g: rand::random(),
            r: rand::random(),
            a: 255,
        };

        triangle_filled(a, b, c, frame_buffer, width, color);
    }

    Ok(())
}
