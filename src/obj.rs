use std::io::BufRead;

use anyhow::Error;

use crate::batteries::Vec3;

#[derive(Debug, Clone)]
pub struct Face {
    pub vertices: Vec<usize>,
    pub vertex_normals: Vec<usize>,
}

#[derive(Debug, Default)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub vertex_normals: Vec<Vec3>,
    pub faces: Vec<Face>,
}

impl Mesh {
    pub fn vertex(&self, face_idx: usize, vertex_idx: usize) -> Vec3 {
        let face = self.faces.get(face_idx).unwrap();
        // faces only store indices of vertices
        // this vertex_id corresponds to the entire model's vertex array
        let vertex_idx = face.vertices.get(vertex_idx).unwrap();
        *self.vertices.get(*vertex_idx).unwrap()
    }

    pub fn vertex_normal(&self, face_idx: usize, vertex_idx: usize) -> Vec3 {
        let face = self.faces.get(face_idx).unwrap();
        let vertex_idx = face.vertex_normals.get(vertex_idx).unwrap();
        *self.vertex_normals.get(*vertex_idx).unwrap()
    }
}

pub fn parse(mut reader: impl BufRead) -> Result<Mesh, Error> {
    let mut mesh = Mesh::default();
    let mut buf = String::new();

    loop {
        buf.clear();
        if reader.read_line(&mut buf)? == 0 {
            break;
        }

        let mut tokens = buf.split_whitespace();
        match tokens.next() {
            Some("v") => {
                let x: f32 = tokens
                    .next()
                    .ok_or_else(|| Error::msg("v: missing x"))?
                    .parse()?;
                let y: f32 = tokens
                    .next()
                    .ok_or_else(|| Error::msg("v: missing y"))?
                    .parse()?;
                let z: f32 = tokens
                    .next()
                    .ok_or_else(|| Error::msg("v: missing z"))?
                    .parse()?;
                mesh.vertices.push(Vec3 { x, y, z });
            }
            Some("vn") => {
                let x: f32 = tokens
                    .next()
                    .ok_or_else(|| Error::msg("v: missing x"))?
                    .parse()?;
                let y: f32 = tokens
                    .next()
                    .ok_or_else(|| Error::msg("v: missing y"))?
                    .parse()?;
                let z: f32 = tokens
                    .next()
                    .ok_or_else(|| Error::msg("v: missing z"))?
                    .parse()?;
                mesh.vertex_normals.push(Vec3 { x, y, z });
            }
            Some("f") => {
                let mut indices = Vec::new();
                let mut vt_indices = Vec::new();
                let mut vn_indices = Vec::new();
                for tok in tokens {
                    let mut parts = tok.split('/');
                    let vert = parts.next().ok_or_else(|| Error::msg("f: empty token"))?;
                    let v_idx: usize = vert.parse()?;
                    if v_idx == 0 {
                        return Err(Error::msg("f: zero index"));
                    }
                    indices.push(v_idx - 1);

                    if let Some(vt) = parts.next() {
                        if !vt.is_empty() {
                            let vt_idx: usize = vt.parse()?;
                            if vt_idx == 0 {
                                return Err(Error::msg("f: zero texture index"));
                            }
                            vt_indices.push(vt_idx - 1);
                        }
                    }
                    if let Some(vn) = parts.next() {
                        if !vn.is_empty() {
                            let vn_idx: usize = vn.parse()?;
                            if vn_idx == 0 {
                                return Err(Error::msg("f: zero normal index"));
                            }
                            vn_indices.push(vn_idx - 1);
                        }
                    }
                }
                mesh.faces.push(Face {
                    vertices: indices,
                    vertex_normals: vn_indices,
                });
            }
            _ => {}
        }
    }

    Ok(mesh)
}
