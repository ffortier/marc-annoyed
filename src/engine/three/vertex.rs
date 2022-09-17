#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl From<(f64, f64, f64)> for Vertex {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self { x, y, z, w: 1.0 }
    }
}
