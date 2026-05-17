use std::ops::Mul;

use crate::batteries::{Vec3, Vec4};

#[derive(Debug, Clone, Copy)]
pub struct Mat3x3 {
    data: [[f32; 3]; 3],
}

impl Mat3x3 {
    pub fn new(data: [[f32; 3]; 3]) -> Self {
        Self { data }
    }

    pub fn determinant(&self) -> f32 {
        let m = &self.data;
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    }

    pub fn cofactor(&self, rows: usize, cols: usize) -> f32 {
        let m = &self.data;
        let i0 = (rows + 1) % 3;
        let i1 = (rows + 2) % 3;
        let j0 = (cols + 1) % 3;
        let j1 = (cols + 2) % 3;
        m[i0][j0] * m[i1][j1] - m[i0][j1] * m[i1][j0]
    }

    pub fn invert_transpose(&self) -> Mat3x3 {
        let det = self.determinant();
        Mat3x3::new([
            [
                self.cofactor(0, 0) / det,
                self.cofactor(0, 1) / det,
                self.cofactor(0, 2) / det,
            ],
            [
                self.cofactor(1, 0) / det,
                self.cofactor(1, 1) / det,
                self.cofactor(1, 2) / det,
            ],
            [
                self.cofactor(2, 0) / det,
                self.cofactor(2, 1) / det,
                self.cofactor(2, 2) / det,
            ],
        ])
    }
}

impl Mul<Vec3> for Mat3x3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.data[0][0] * rhs.x + self.data[0][1] * rhs.y + self.data[0][2] * rhs.z,
            self.data[1][0] * rhs.x + self.data[1][1] * rhs.y + self.data[1][2] * rhs.z,
            self.data[2][0] * rhs.x + self.data[2][1] * rhs.y + self.data[2][2] * rhs.z,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Mat4x4 {
    data: [[f32; 4]; 4],
}

impl Mat4x4 {
    pub fn viewport(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            data: [
                [width as f32 / 2., 0., 0., (x + width / 2) as f32],
                [0.0, height as f32 / 2., 0.0, (y + height / 2) as f32],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn perspective(focal_dist: f32) -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, -1.0 / focal_dist, 1.0],
            ],
        }
    }

    pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        let n = (eye - center).normalized();
        let l = up.cross(n).normalized();
        let m = n.cross(l).normalized();

        let inverse_coordinate_transform_mat = Self {
            data: [
                [l.x, l.y, l.z, 0.0],
                [m.x, m.y, m.z, 0.0],
                [n.x, n.y, n.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        let translation_mat = Self {
            data: [
                [1.0, 0.0, 0.0, -center.x],
                [0.0, 1.0, 0.0, -center.y],
                [0.0, 0.0, 1.0, -center.z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        inverse_coordinate_transform_mat * translation_mat
    }
}

impl Mul for Mat4x4 {
    type Output = Mat4x4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut data = [[0.0f32; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                data[i][j] = self.data[i][0] * rhs.data[0][j]
                    + self.data[i][1] * rhs.data[1][j]
                    + self.data[i][2] * rhs.data[2][j]
                    + self.data[i][3] * rhs.data[3][j];
            }
        }
        Self::Output { data }
    }
}

impl Mul<Vec4> for Mat4x4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Self::Output {
            x: self.data[0][0] * rhs.x
                + self.data[0][1] * rhs.y
                + self.data[0][2] * rhs.z
                + self.data[0][3] * rhs.w,
            y: self.data[1][0] * rhs.x
                + self.data[1][1] * rhs.y
                + self.data[1][2] * rhs.z
                + self.data[1][3] * rhs.w,
            z: self.data[2][0] * rhs.x
                + self.data[2][1] * rhs.y
                + self.data[2][2] * rhs.z
                + self.data[2][3] * rhs.w,
            w: self.data[3][0] * rhs.x
                + self.data[3][1] * rhs.y
                + self.data[3][2] * rhs.z
                + self.data[3][3] * rhs.w,
        }
    }
}
