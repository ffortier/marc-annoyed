use super::Vector;

#[derive(Debug, Default)]
pub struct Triangle {
    pub a: Vector,
    pub b: Vector,
    pub c: Vector,
}

impl Triangle {
    pub fn new(a: Vector, b: Vector, c: Vector) -> Self {
        Self { a, b, c }
    }
}
