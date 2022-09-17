use std::str::FromStr;

use anyhow::Error;
use thiserror::Error;

use super::{
    object_parser::{document, Line},
    Triangle, Vertex,
};

#[derive(Debug)]
pub struct Object3d {
    mesh: Vec<Triangle>,
}

impl FromStr for Object3d {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vertices = vec![];
        let mut mesh = vec![];

        for line in document(s)? {
            match line {
                Line::Vertex(x, y, z, w) => {
                    vertices.push(Vertex {
                        x,
                        y,
                        z,
                        w: w.unwrap_or(1.0),
                    });
                }
                Line::Face(vertex_indices) => {
                    if vertex_indices.len() != 3 {
                        return Err(Error::from(ParseError::NotATriangle));
                    }

                    let a = vertices
                        .get(vertex_indices[0].0 - 1)
                        .ok_or(ParseError::UnknownIndices)?;
                    let b = vertices
                        .get(vertex_indices[1].0 - 1)
                        .ok_or(ParseError::UnknownIndices)?;
                    let c = vertices
                        .get(vertex_indices[2].0 - 1)
                        .ok_or(ParseError::UnknownIndices)?;

                    mesh.push(Triangle {
                        a: *a,
                        b: *b,
                        c: *c,
                    });
                }
                _ => {}
            }
        }

        Ok(Self { mesh })
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ParseError {
    #[error("Not a triangle, expected 3 vertices")]
    NotATriangle,
    #[error("Unknown indices")]
    UnknownIndices,
}
