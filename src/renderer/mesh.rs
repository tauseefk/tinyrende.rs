use std::{fs::File, io::BufReader, path::Path};

use anyhow::Error;
use tgar::PixelBGRA;

use crate::batteries::{Vec3, Vec4, random_color};
use crate::mat::Mat4x4;
use crate::renderer::rasterizer::{Shader, SomeShader, rasterize};
use crate::{DEFAULT_COLOR, obj};

pub struct FrameBuffer {
    pub data: Vec<PixelBGRA>,
    pub width: u16,
    pub height: u16,
}

pub fn render_mesh(
    path: &Path,
    frame_buffer: &mut FrameBuffer,
    depth_buffer: &mut [f32],
) -> Result<(), Error> {
    let mesh = obj::parse(BufReader::new(File::open(path)?))?;
    let eye = Vec3::new(-1., 0., 2.); // camera position
    let center = Vec3::new(0., 0., 0.); // model center
    let up = Vec3::new(0., 1., 0.); // up direction
    let model_view = Mat4x4::look_at(eye, center, up);
    let perspective = Mat4x4::perspective((eye - center).length());
    let viewport = Mat4x4::viewport(
        (frame_buffer.width as i32) / 16,
        (frame_buffer.height as i32) / 16,
        ((frame_buffer.width as i32) * 7) / 8,
        ((frame_buffer.height as i32) * 7) / 8,
    );

    let mut shader: SomeShader = SomeShader {
        model: &mesh,
        model_view,
        perspective,
        color: DEFAULT_COLOR,
        triangle: [Vec4::zero(), Vec4::zero(), Vec4::zero()],
    };

    for (f, _) in mesh.faces.iter().enumerate() {
        let clip_triangle: [Vec4; 3] = [0, 1, 2].map(|v| shader.vertex(f, v));

        shader.color = random_color();

        rasterize(clip_triangle, viewport, &shader, frame_buffer, depth_buffer);
    }

    Ok(())
}
