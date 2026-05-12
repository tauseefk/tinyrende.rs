use tgar::PixelBGRA;

use crate::batteries::{FloatColor, GridPosition, Vertex};

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
