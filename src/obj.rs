use std::io::BufRead;

use anyhow::Error;

use crate::grid_position::Vertex;

#[derive(Debug, Clone)]
pub struct Face {
    pub vertices: Vec<usize>,
}

#[derive(Debug, Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
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
                mesh.vertices.push(Vertex { x, y, z });
            }
            Some("f") => {
                let mut indices = Vec::new();
                for tok in tokens {
                    let vert = tok
                        .split('/')
                        .next()
                        .ok_or_else(|| Error::msg("f: empty token"))?;
                    let idx: usize = vert.parse()?;
                    if idx == 0 {
                        return Err(Error::msg("f: zero index"));
                    }
                    indices.push(idx - 1);
                }
                mesh.faces.push(Face { vertices: indices });
            }
            _ => {}
        }
    }

    Ok(mesh)
}
