use tgar::PixelBGRA;

use crate::grid_position::GridPosition;

pub fn line(
    a: GridPosition,
    b: GridPosition,
    pixel_data: &mut [PixelBGRA],
    width: u16,
    color: PixelBGRA,
) {
    // steep slope
    let is_steep_slope = a.x.abs_diff(b.x) < a.y.abs_diff(b.y);

    let (a, b) = if is_steep_slope {
        (a.transpose(), b.transpose())
    } else {
        (a, b)
    };

    let (a, b) = if a.x > b.x { (b, a) } else { (a, b) };

    let mut y: i32 = a.y as i32;
    let sign_y: i32 = if b.y > a.y { 1 } else { -1 };

    let mut ierror: i32 = 0;
    let aby = b.y.abs_diff(a.y) as i32;

    let dx = (b.x - a.x) as i32;

    for x in a.x..=b.x {
        let y_u16 = y as u16;
        let coord = if is_steep_slope {
            GridPosition { x: y_u16, y: x }
        } else {
            GridPosition { x, y: y_u16 }
        };
        pixel_data[coord.to_idx(width)] = color;

        ierror += 2 * aby;
        if ierror > dx {
            y += sign_y;
            ierror -= 2 * dx;
        }
    }
}

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
