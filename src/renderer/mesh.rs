use std::{fs::File, io::BufReader, path::Path};

use anyhow::Error;
use tgar::PixelBGRA;

use crate::batteries::{Vec3, Vec4, random_color};
use crate::mat::Mat4x4;
use crate::obj;
use crate::renderer::rasterizer::rasterize;

pub fn render_mesh(
    path: &Path,
    frame_buffer: &mut [PixelBGRA],
    depth_buffer: &mut [f32],
    width: u16,
    height: u16,
) -> Result<(), Error> {
    let mesh = obj::parse(BufReader::new(File::open(path)?))?;
    let eye = Vec3::new(-1., 0., 2.); // camera position
    let center = Vec3::new(0., 0., 0.); // model center
    let up = Vec3::new(0., 1., 0.); // up direction
    let model_view = Mat4x4::look_at(eye, center, up);
    let projection = Mat4x4::perspective((eye - center).length());
    let viewport = Mat4x4::viewport(
        (width as i32) / 16,
        (height as i32) / 16,
        ((width as i32) * 7) / 8,
        ((height as i32) * 7) / 8,
    );

    let view_proj = projection * model_view;

    for face in &mesh.faces {
        // Assemble the primitive in clip space; the rasterizer handles the
        // perspective divide, viewport mapping, and depth test.
        let clip: [Vec4; 3] = [0, 1, 2].map(|i| {
            let v = mesh.vertices[face.vertices[i]];
            view_proj
                * Vec4 {
                    x: v.x,
                    y: v.y,
                    z: v.z,
                    w: 1.0,
                }
        });

        let color = random_color();
        rasterize(
            clip,
            viewport,
            color,
            frame_buffer,
            depth_buffer,
            width,
            height,
        );
    }

    Ok(())
}
