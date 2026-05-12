use tgar::PixelBGRA;

#[derive(Debug, Clone, Copy)]
pub struct GridPosition {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

impl GridPosition {
    pub fn to_idx(&self, width: u16) -> usize {
        (self.y as usize) * width as usize + (self.x as usize)
    }

    #[allow(dead_code)]
    pub fn transpose_xy(&self) -> GridPosition {
        GridPosition {
            x: self.y,
            y: self.x,
            z: self.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: GridPosition,
    pub color: PixelBGRA,
}

#[derive(Debug, Clone, Copy)]
pub struct FloatColor {
    pub b: f32,
    pub r: f32,
    pub g: f32,
    #[allow(dead_code)]
    pub a: f32,
}

impl From<PixelBGRA> for FloatColor {
    fn from(color: PixelBGRA) -> Self {
        FloatColor {
            b: color.b as f32,
            r: color.r as f32,
            g: color.g as f32,
            a: color.a as f32,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Translation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Translation {
    pub fn project(&self, width: u16, height: u16) -> GridPosition {
        GridPosition {
            x: ((self.x + 1.0) * (width / 2) as f32) as u16,
            y: ((self.y + 1.0) * (height / 2) as f32) as u16,
            z: ((self.z + 1.0) * 255. / 2.) as u16,
        }
    }
}
