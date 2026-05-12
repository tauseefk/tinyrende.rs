use rand::RngExt;
use tgar::PixelBGRA;

pub fn random_color() -> PixelBGRA {
    let mut rng = rand::rng();
    PixelBGRA {
        b: rng.random(),
        g: rng.random(),
        r: rng.random(),
        a: 255,
    }
}

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
}

impl From<PixelBGRA> for FloatColor {
    fn from(color: PixelBGRA) -> Self {
        FloatColor {
            b: color.b as f32,
            r: color.r as f32,
            g: color.g as f32,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Transform {
    pub fn translate(&self, by: Transform) -> Transform {
        Transform {
            x: self.x + by.x,
            y: self.y + by.y,
            z: self.z + by.z,
        }
    }

    pub fn rot_xz(&self, angle: f32) -> Transform {
        let c = angle.cos();
        let s = angle.sin();
        Transform {
            x: self.x * c - self.z * s,
            y: self.y,
            z: self.x * s + self.z * c,
        }
    }

    pub fn div(&self, rhs: f32) -> Transform {
        Transform {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }

    pub fn persp(&self) -> Transform {
        let c: f32 = 3.0;

        self.div(1.0 - self.z / c)
    }

    pub fn project(&self, width: u16, height: u16) -> GridPosition {
        GridPosition {
            x: ((self.x + 1.0) * (width / 2) as f32) as u16,
            y: ((self.y + 1.0) * (height / 2) as f32) as u16,
            z: ((self.z + 1.0) * 255. / 2.) as u16,
        }
    }
}
