use std::{fs::File, io::BufReader, path::Path};

use anyhow::Error;
use tgar::PixelBGRA;

use crate::batteries::Vertex;
use crate::renderer::triangle::triangle_filled;
use crate::{BLUE, GREEN, RED, obj};

pub fn render_mesh(
    path: &Path,
    frame_buffer: &mut [PixelBGRA],
    depth_buffer: &mut [u8],
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

        triangle_filled(
            Vertex {
                position: a,
                color: RED,
            },
            Vertex {
                position: b,
                color: GREEN,
            },
            Vertex {
                position: c,
                color: BLUE,
            },
            frame_buffer,
            depth_buffer,
            width,
        );
    }

    Ok(())
}
