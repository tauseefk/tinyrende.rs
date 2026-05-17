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

    #[allow(dead_code)]
    pub fn identity() -> Self {
        Self {
            data: std::array::from_fn(|i| std::array::from_fn(|j| if i == j { 1.0 } else { 0.0 })),
        }
    }

    #[allow(dead_code)]
    pub fn transpose(&self) -> Self {
        Self {
            data: std::array::from_fn(|i| std::array::from_fn(|j| self.data[j][i])),
        }
    }
}

impl<const N: usize> Mul for Mat<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            data: std::array::from_fn(|i| {
                std::array::from_fn(|j| (0..N).map(|k| self.data[i][k] * rhs.data[k][j]).sum())
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

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f32 = 1e-4;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPS
    }

    fn mat_approx_eq<const N: usize>(a: &Mat<N>, b: &Mat<N>) -> bool {
        (0..N).all(|i| (0..N).all(|j| approx_eq(a.data[i][j], b.data[i][j])))
    }

    fn vec3_approx_eq(a: Vec3, b: Vec3) -> bool {
        approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z)
    }

    fn vec4_approx_eq(a: Vec4, b: Vec4) -> bool {
        approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z) && approx_eq(a.w, b.w)
    }

    fn a3() -> Mat3x3 {
        Mat::new([[1.0, 2.0, 3.0], [0.0, 1.0, 4.0], [5.0, 6.0, 0.0]])
    }
    fn b3() -> Mat3x3 {
        Mat::new([[2.0, 0.0, 1.0], [3.0, 1.0, 0.0], [1.0, 1.0, 1.0]])
    }
    fn c3() -> Mat3x3 {
        Mat::new([[1.0, 1.0, 0.0], [0.0, 2.0, 1.0], [1.0, 0.0, 3.0]])
    }

    fn a4() -> Mat4x4 {
        Mat::new([
            [1.0, 2.0, 3.0, 4.0],
            [0.0, 1.0, 2.0, 3.0],
            [1.0, 0.0, 1.0, 0.0],
            [2.0, 1.0, 0.0, 1.0],
        ])
    }
    fn b4() -> Mat4x4 {
        Mat::new([
            [2.0, 0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0, 2.0],
            [0.0, 3.0, 1.0, 1.0],
            [1.0, 0.0, 2.0, 1.0],
        ])
    }
    fn c4() -> Mat4x4 {
        Mat::new([
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 2.0, 0.0],
            [3.0, 0.0, 1.0, 0.0],
            [0.0, 2.0, 0.0, 1.0],
        ])
    }

    #[test]
    fn mat3_identity_right() {
        let m = a3();
        assert!(mat_approx_eq(&(m * Mat3x3::identity()), &m));
    }

    #[test]
    fn mat3_identity_left() {
        let m = a3();
        assert!(mat_approx_eq(&(Mat3x3::identity() * m), &m));
    }

    #[test]
    fn mat4_identity_right() {
        let m = a4();
        assert!(mat_approx_eq(&(m * Mat4x4::identity()), &m));
    }

    #[test]
    fn mat4_identity_left() {
        let m = a4();
        assert!(mat_approx_eq(&(Mat4x4::identity() * m), &m));
    }

    #[test]
    fn mat3_matmul_associative() {
        let (a, b, c) = (a3(), b3(), c3());
        assert!(mat_approx_eq(&((a * b) * c), &(a * (b * c))));
    }

    #[test]
    fn mat4_matmul_associative() {
        let (a, b, c) = (a4(), b4(), c4());
        assert!(mat_approx_eq(&((a * b) * c), &(a * (b * c))));
    }

    #[test]
    fn mat3_transpose_involution() {
        let m = a3();
        assert!(mat_approx_eq(&m.transpose().transpose(), &m));
    }

    #[test]
    fn mat4_transpose_involution() {
        let m = a4();
        assert!(mat_approx_eq(&m.transpose().transpose(), &m));
    }

    #[test]
    fn mat3_det_of_identity_is_one() {
        assert!(approx_eq(Mat3x3::identity().determinant(), 1.0));
    }

    #[test]
    fn mat3_det_of_product() {
        let (a, b) = (a3(), b3());
        assert!(approx_eq(
            (a * b).determinant(),
            a.determinant() * b.determinant(),
        ));
    }

    #[test]
    fn mat3_cofactor_expansion_row_0() {
        let m = a3();
        let expanded = m.data[0][0] * m.cofactor(0, 0)
            + m.data[0][1] * m.cofactor(0, 1)
            + m.data[0][2] * m.cofactor(0, 2);
        assert!(approx_eq(m.determinant(), expanded));
    }

    #[test]
    fn mat3_cofactor_expansion_row_1() {
        let m = a3();
        let expanded = m.data[1][0] * m.cofactor(1, 0)
            + m.data[1][1] * m.cofactor(1, 1)
            + m.data[1][2] * m.cofactor(1, 2);
        assert!(approx_eq(m.determinant(), expanded));
    }

    #[test]
    fn mat3_cofactor_expansion_row_2() {
        let m = a3();
        let expanded = m.data[2][0] * m.cofactor(2, 0)
            + m.data[2][1] * m.cofactor(2, 1)
            + m.data[2][2] * m.cofactor(2, 2);
        assert!(approx_eq(m.determinant(), expanded));
    }

    #[test]
    fn mat3_invert_transpose_roundtrip() {
        let m = a3();
        let result = m * m.invert_transpose().transpose();
        assert!(mat_approx_eq(&result, &Mat3x3::identity()));
    }

    #[test]
    fn mat4_look_at_maps_center_to_origin() {
        let eye = Vec3::new(3.0, 4.0, 5.0);
        let center = Vec3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let m = Mat4x4::look_at(eye, center, up);
        let result = m * Vec4 {
            x: center.x,
            y: center.y,
            z: center.z,
            w: 1.0,
        };
        assert!(vec4_approx_eq(
            result,
            Vec4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0
            }
        ));
    }

    #[test]
    fn mat4_look_at_eye_distance() {
        let eye = Vec3::new(3.0, 4.0, 5.0);
        let center = Vec3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let m = Mat4x4::look_at(eye, center, up);
        let result = m * Vec4 {
            x: eye.x,
            y: eye.y,
            z: eye.z,
            w: 1.0,
        };
        let dist = (eye - center).length();
        assert!(vec4_approx_eq(
            result,
            Vec4 {
                x: 0.0,
                y: 0.0,
                z: dist,
                w: 1.0
            }
        ));
    }

    #[test]
    fn mat4_perspective_at_focal_distance_is_infinity() {
        let f = 2.0;
        let m = Mat4x4::perspective(f);
        let result = m * Vec4 {
            x: 1.0,
            y: 2.0,
            z: f,
            w: 1.0,
        };
        assert!(approx_eq(result.w, 0.0));
    }

    #[test]
    fn mat4_perspective_at_origin_is_identity_on_xyzw() {
        let f = 2.0;
        let m = Mat4x4::perspective(f);
        let v = Vec4 {
            x: 1.0,
            y: 2.0,
            z: 0.0,
            w: 1.0,
        };
        assert!(vec4_approx_eq(m * v, v));
    }

    #[test]
    fn mat4_viewport_corners() {
        let (w, h) = (800, 600);
        let m = Mat4x4::viewport(0, 0, w, h);
        let bl = m * Vec4 {
            x: -1.0,
            y: -1.0,
            z: 0.0,
            w: 1.0,
        };
        let tr = m * Vec4 {
            x: 1.0,
            y: 1.0,
            z: 0.0,
            w: 1.0,
        };
        assert!(approx_eq(bl.x, 0.0) && approx_eq(bl.y, 0.0));
        assert!(approx_eq(tr.x, w as f32) && approx_eq(tr.y, h as f32));
    }

    #[test]
    fn mat3_vec3_mul_against_explicit() {
        let m = Mat3x3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert!(vec3_approx_eq(m * v, Vec3::new(14.0, 32.0, 50.0)));
    }

    #[test]
    fn mat4_vec4_mul_against_explicit() {
        let m = Mat4x4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let v = Vec4 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        };
        assert!(vec4_approx_eq(
            m * v,
            Vec4 {
                x: 10.0,
                y: 26.0,
                z: 42.0,
                w: 58.0
            },
        ));
    }
}
