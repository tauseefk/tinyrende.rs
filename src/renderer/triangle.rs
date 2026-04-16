use tgar::PixelBGRA;

use crate::BLUE;
use crate::grid_position::GridPosition;
use crate::renderer::line::line;

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

    for y in bounding_box.0.y..bounding_box.1.y {
        for x in bounding_box.0.x..bounding_box.1.x {
            line(
                GridPosition { x, y },
                GridPosition { x: x + 1, y },
                pixel_data,
                width,
                color,
            );
        }
    }
    line(a, b, pixel_data, width, BLUE);
    line(b, c, pixel_data, width, BLUE);
    line(a, c, pixel_data, width, BLUE);
}

pub fn get_bounding_box(
    a: GridPosition,
    b: GridPosition,
    c: GridPosition,
) -> (GridPosition, GridPosition) {
    vec![a, b, c].iter().fold((a, b), |acc, curr| {
        (
            GridPosition {
                x: acc.0.x.min(curr.x),
                y: acc.0.y.min(curr.y),
            },
            GridPosition {
                x: acc.1.x.max(curr.x),
                y: acc.1.y.max(curr.y),
            },
        )
    })
}
