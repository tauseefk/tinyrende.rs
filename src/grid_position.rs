#[derive(Copy, Clone)]
pub struct GridPosition {
    pub x: u16,
    pub y: u16,
}

impl GridPosition {
    pub fn to_idx(&self, width: u16) -> usize {
        (self.y as usize) * width as usize + (self.x as usize)
    }

    pub fn transpose(&self) -> GridPosition {
        GridPosition {
            x: self.y,
            y: self.x,
        }
    }
}
