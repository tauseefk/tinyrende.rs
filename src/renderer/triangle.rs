use tgar::PixelBGRA;

use crate::batteries::{FloatColor, GridPosition, Vec3, Vertex};
use crate::mat4x4::{Mat3x3, Mat4x4, Vec4};

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

pub fn triangle_filled(
    a: Vertex,
    b: Vertex,
    c: Vertex,
    pixel_data: &mut [PixelBGRA],
    depth_data: &mut [u8],
    width: u16,
    height: u16,
) {
    let a_pos = a.position;
    let b_pos = b.position;
    let c_pos = c.position;
    let a_color: FloatColor = a.color.into();
    let b_color: FloatColor = b.color.into();
    let c_color: FloatColor = c.color.into();

    let bounding_box = get_bounding_box(a_pos, b_pos, c_pos);
    let area = signed_triangle_area(a_pos, b_pos, c_pos);
    if area == 0. {
        return;
    }

    let az = a_pos.z as f32;
    let bz = b_pos.z as f32;
    let cz = c_pos.z as f32;

    let min_x = bounding_box.0.x.min(width);
    let max_x = bounding_box.1.x.min(width);
    let min_y = bounding_box.0.y.min(height);
    let max_y = bounding_box.1.y.min(height);

    for y in min_y..max_y {
        for x in min_x..max_x {
            let p = GridPosition { x, y, z: 1 };

            let alpha = signed_triangle_area(a_pos, b_pos, p) / area;
            let beta = signed_triangle_area(p, b_pos, c_pos) / area;
            let gamma = signed_triangle_area(a_pos, p, c_pos) / area;

            if alpha < 0. || beta < 0. || gamma < 0. {
                continue;
            }
            let z = alpha * az + beta * bz + gamma * cz;
            let pixel_idx = p.to_idx(width);
            let existing_z = depth_data[pixel_idx];

            if z > (existing_z as f32) {
                depth_data[pixel_idx] = (z as u8).max(depth_data[pixel_idx]);

                pixel_data[pixel_idx] = PixelBGRA {
                    b: (a_color.b * alpha + b_color.b * beta + c_color.b * gamma) as u8,
                    g: (a_color.g * alpha + b_color.g * beta + c_color.g * gamma) as u8,
                    r: (a_color.r * alpha + b_color.r * beta + c_color.r * gamma) as u8,
                    a: z as u8,
                };
            }
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

fn get_bounding_box(
    a: GridPosition,
    b: GridPosition,
    c: GridPosition,
) -> (GridPosition, GridPosition) {
    vec![a, b, c].iter().fold((a, b), |acc, curr| {
        (
            GridPosition {
                x: acc.0.x.min(curr.x),
                y: acc.0.y.min(curr.y),
                z: acc.0.z.min(curr.z),
            },
            GridPosition {
                x: acc.1.x.max(curr.x),
                y: acc.1.y.max(curr.y),
                z: acc.0.z.max(curr.z),
            },
        )
    })
}

fn signed_triangle_area(a: GridPosition, b: GridPosition, c: GridPosition) -> f32 {
    let ax = a.x as f32;
    let ay = a.y as f32;
    let bx = b.x as f32;
    let by = b.y as f32;
    let cx = c.x as f32;
    let cy = c.y as f32;

    0.5 * ((by - ay) * (bx + ax) + (cy - by) * (cx + bx) + (ay - cy) * (ax + cx))
}
