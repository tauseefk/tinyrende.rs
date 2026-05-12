use std::f32::consts::PI;
use std::{fs::File, io::BufReader, path::Path};

use anyhow::Error;
use tgar::PixelBGRA;

use crate::batteries::Vertex;
use crate::renderer::triangle::triangle_filled;
use crate::{BLUE, GREEN, RED, obj};

const THIRTY_DEGREES: f32 = PI / 6.0;

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

        let a = a.rot_xz(THIRTY_DEGREES).project(width, height);
        let b = b.rot_xz(THIRTY_DEGREES).project(width, height);
        let c = c.rot_xz(THIRTY_DEGREES).project(width, height);

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
