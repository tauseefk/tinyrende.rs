use tgar::PixelBGRA;

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
