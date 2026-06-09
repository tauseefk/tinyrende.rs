use std::ops::Mul;

use tgar::PixelBGRA;

use crate::{
    batteries::{Vec3, Vec4},
    mat::Mat4x4,
    obj::Mesh,
};

pub trait Shader {
    fn vertex(&mut self, face_idx: usize, vert_idx: usize) -> Vec4;

    fn fragment(&self) -> (bool, PixelBGRA);
}

#[allow(dead_code)]
pub struct FlatShder<'rast> {
    pub model: &'rast Mesh,
    pub color: PixelBGRA,
    pub triangle: [Vec3; 3],
    pub model_view: Mat4x4,
    pub perspective: Mat4x4,
}

impl<'rast> Shader for FlatShder<'rast> {
    fn vertex(&mut self, face_idx: usize, vert_idx: usize) -> Vec4 {
        let v: Vec3 = self.model.vertex(face_idx, vert_idx);
        let v: Vec4 = self.model_view.mul(Vec4 {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 1.0,
        });
        // this currently doesn't accomplish anything
        self.triangle[vert_idx] = v.xyz();
        self.perspective.mul(v)
    }

    fn fragment(&self) -> (bool, PixelBGRA) {
        (false, self.color)
    }
}

pub struct PhongShader<'rast> {
    pub model: &'rast Mesh,
    pub light: Vec3,
    pub triangle: [Vec3; 3],
    pub model_view: Mat4x4,
    pub perspective: Mat4x4,
}

impl<'rast> PhongShader<'rast> {
    pub fn new(model: &'rast Mesh, light: Vec3, model_view: Mat4x4, perspective: Mat4x4) -> Self {
        Self {
            model,
            light: model_view
                .mul(Vec4 {
                    x: light.x,
                    y: light.y,
                    z: light.z,
                    w: 1.0,
                })
                .xyz()
                .normalized(),
            triangle: [Vec3::zero(); 3],
            model_view,
            perspective,
        }
    }
}

impl<'rast> Shader for PhongShader<'rast> {
    fn vertex(&mut self, face_idx: usize, vert_idx: usize) -> Vec4 {
        let v: Vec3 = self.model.vertex(face_idx, vert_idx);
        let v: Vec4 = self.model_view.mul(Vec4 {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 1.0,
        });
        self.triangle[vert_idx] = v.xyz();
        self.perspective.mul(v)
    }

    fn fragment(&self) -> (bool, PixelBGRA) {
        let normal: Vec3 = (self.triangle[1] - self.triangle[0])
            .cross(self.triangle[2] - self.triangle[1])
            .normalized();
        let reflect: Vec3 = (normal.mul(2. * normal.dot(self.light)) - self.light).normalized();
        let ambient = 0.3;
        let diff = normal.dot(self.light).max(0.0);
        let spec = reflect.z.max(0.0).powi(35);

        let mul = (ambient + 0.4 * diff + 0.9 * spec).min(1.0);
        let color: PixelBGRA = PixelBGRA {
            b: (255. * mul) as u8,
            g: (255. * mul) as u8,
            r: (255. * mul) as u8,
            a: 255,
        };
        (false, color)
    }
}
