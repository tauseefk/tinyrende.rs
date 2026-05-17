use tgar::PixelBGRA;

use crate::batteries::{Vec3, Vec4};
use crate::mat4x4::{Mat3x3, Mat4x4};

/// Rasterize a single triangle defined by three clip-space vertices.
///
pub fn rasterize(
    clip: [Vec4; 3],
    viewport: Mat4x4,
    color: PixelBGRA,
    frame_buffer: &mut [PixelBGRA],
    depth_buffer: &mut [f32],
    width: u16,
    height: u16,
) {
    let ndc = clip.map(|v: Vec4| Vec4 {
        x: v.x / v.w,
        y: v.y / v.w,
        z: v.z / v.w,
        w: 1.0,
    });

    // viewport -> screen-space pixel coordinates
    let s0 = viewport * ndc[0];
    let s1 = viewport * ndc[1];
    let s2 = viewport * ndc[2];

    let z0 = ndc[0].z;
    let z1 = ndc[1].z;
    let z2 = ndc[2].z;

    // screen-space triangle as a 3x3 matrix
    let tri_abc = Mat3x3::new([[s0.x, s0.y, 1.0], [s1.x, s1.y, 1.0], [s2.x, s2.y, 1.0]]);

    // `det(ABC)` is the parallelogram area — backface cull / reject
    // sub-pixel triangles per the reference.
    if tri_abc.determinant() < 1.0 {
        return;
    }
    // `invert_transpose` returns cofactor(ABC)/det = (ABC^-1)^T, so
    // multiplying by (px, py, 1) yields the barycentric weights
    let abc_inv_t = tri_abc.invert_transpose();

    let bounding_box = get_screen_bounding_box(s0, s1, s2);
    let min_x = bounding_box.0.x.max(0.0) as i32;
    let max_x = bounding_box.1.x.min((width - 1) as f32) as i32;
    let min_y = bounding_box.0.y.max(0.0) as i32;
    let max_y = bounding_box.1.y.min((height - 1) as f32) as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let barycenter = abc_inv_t * Vec3::new(x as f32, y as f32, 1.0);
            if barycenter.x < 0.0 || barycenter.y < 0.0 || barycenter.z < 0.0 {
                continue;
            }

            // depth interpolation in NDC space
            let z = barycenter.x * z0 + barycenter.y * z1 + barycenter.z * z2;
            let idx = y as usize * width as usize + x as usize;
            if z <= depth_buffer[idx] {
                continue;
            }
            depth_buffer[idx] = z;
            frame_buffer[idx] = color;
        }
    }
}

fn get_screen_bounding_box(a: Vec4, b: Vec4, c: Vec4) -> (Vec4, Vec4) {
    vec![a, b, c].iter().fold((a, b), |acc, curr| {
        (
            Vec4 {
                x: acc.0.x.min(curr.x),
                y: acc.0.y.min(curr.y),
                z: 0.0,
                w: 1.0,
            },
            Vec4 {
                x: acc.1.x.max(curr.x),
                y: acc.1.y.max(curr.y),
                z: 0.0,
                w: 1.0,
            },
        )
    })
}
