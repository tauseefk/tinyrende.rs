use std::{fs::File, io::BufReader, path::Path};

use anyhow::Error;
use tgar::PixelBGRA;

use crate::obj;
use crate::renderer::triangle::triangle_filled;

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

        let a = a.project(width, height);
        let b = b.project(width, height);
        let c = c.project(width, height);

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
