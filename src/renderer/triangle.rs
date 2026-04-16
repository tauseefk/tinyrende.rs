use tgar::PixelBGRA;

use crate::grid_position::GridPosition;
use crate::renderer::line::line;

#[allow(dead_code)]
pub fn triangle(
    a: GridPosition,
    b: GridPosition,
    c: GridPosition,
    pixel_data: &mut [PixelBGRA],
    width: u16,
    color: PixelBGRA,
) {
    line(a, b, pixel_data, width, color);
    line(b, c, pixel_data, width, color);
    line(c, a, pixel_data, width, color);
}

pub fn triangle_filled(
    a: GridPosition,
    b: GridPosition,
    c: GridPosition,
    pixel_data: &mut [PixelBGRA],
    width: u16,
    color: PixelBGRA,
) {
    let bounding_box = get_bounding_box(a, b, c);
    let area = signed_triangle_area(a, b, c);
    if area == 0. {
        return;
    }

    let az = a.z as f32;
    let bz = b.z as f32;
    let cz = c.z as f32;

    for y in bounding_box.0.y..bounding_box.1.y {
        for x in bounding_box.0.x..bounding_box.1.x {
            let p = GridPosition { x, y, z: 1 };
            let alpha = signed_triangle_area(a, b, p) / area;
            let beta = signed_triangle_area(p, b, c) / area;
            let gamma = signed_triangle_area(a, p, c) / area;

            if alpha < 0. || beta < 0. || gamma < 0. {
                continue;
            }
            let z = alpha * az + beta * bz + gamma * cz;
            pixel_data[p.to_idx(width)] = PixelBGRA {
                b: color.b,
                g: color.g,
                r: color.r,
                a: z as u8,
            };
        }
    }

    // triangle(a, b, c, pixel_data, width, BLACK);
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
