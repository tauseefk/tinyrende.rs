use std::ops::Mul;

use crate::batteries::{Vec3, Vec4};

#[derive(Debug, Clone, Copy)]
pub struct Mat<const N: usize> {
    data: [[f32; N]; N],
}

pub type Mat3x3 = Mat<3>;
pub type Mat4x4 = Mat<4>;

impl<const N: usize> Mat<N> {
    pub fn new(data: [[f32; N]; N]) -> Self {
        Self { data }
    }
}

impl<const N: usize> Mul for Mat<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            data: std::array::from_fn(|i| {
                std::array::from_fn(|j| {
                    (0..N).map(|k| self.data[i][k] * rhs.data[k][j]).sum()
                })
            }),
        }
    }
}

impl Mat<3> {
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

    pub fn invert_transpose(&self) -> Self {
        let det = self.determinant();
        Self::new(std::array::from_fn(|i| {
            std::array::from_fn(|j| self.cofactor(i, j) / det)
        }))
    }
}

impl Mat<4> {
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

fn transform<const N: usize>(m: [[f32; N]; N], v: [f32; N]) -> [f32; N] {
    std::array::from_fn(|i| (0..N).map(|j| m[i][j] * v[j]).sum())
}

impl Mul<Vec3> for Mat<3> {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        let r = transform(self.data, [rhs.x, rhs.y, rhs.z]);
        Vec3::new(r[0], r[1], r[2])
    }
}

impl Mul<Vec4> for Mat<4> {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        let r = transform(self.data, [rhs.x, rhs.y, rhs.z, rhs.w]);
        Vec4 {
            x: r[0],
            y: r[1],
            z: r[2],
            w: r[3],
        }
    }
}
