use std::{fs::File, io::BufReader, path::Path};

use anyhow::Error;
use tgar::PixelBGRA;

use crate::renderer::line::line;
use crate::{
    WHITE,
    grid_position::GridPosition,
    obj::{self, Vertex},
};

pub fn render_mesh(
    path: &Path,
    frame_buffer: &mut [PixelBGRA],
    width: u16,
    height: u16,
) -> Result<(), Error> {
    let mesh = obj::parse(BufReader::new(File::open(path)?))?;

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
            line(project(a), project(b), frame_buffer, width, WHITE);
        }
    }

    Ok(())
}
