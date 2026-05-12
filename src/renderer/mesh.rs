use std::{fs::File, io::BufReader, path::Path};

use anyhow::Error;
use tgar::PixelBGRA;

use crate::batteries::{Vertex, random_color};
use crate::renderer::triangle::triangle_filled;
use crate::{ROTATION_ANGLE, TRANSLATION, obj};

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

        let a = a
            .rot_xz(ROTATION_ANGLE)
            .translate(TRANSLATION)
            .persp()
            .project(width, height);
        let b = b
            .rot_xz(ROTATION_ANGLE)
            .translate(TRANSLATION)
            .persp()
            .project(width, height);
        let c = c
            .rot_xz(ROTATION_ANGLE)
            .translate(TRANSLATION)
            .persp()
            .project(width, height);

        let color = random_color();
        triangle_filled(
            Vertex { position: a, color },
            Vertex { position: b, color },
            Vertex { position: c, color },
            frame_buffer,
            depth_buffer,
            width,
            height,
        );
    }

    Ok(())
}
