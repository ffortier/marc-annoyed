use std::ops::{Index, IndexMut};

/// Vector in a 3D space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// Extra component required by some transformations
    pub w: f64,
}

impl Default for Vector {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, None)
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64, w: Option<f64>) -> Self {
        Self {
            x,
            y,
            z,
            w: w.unwrap_or(1.0),
        }
    }
}

impl From<(f64, f64, f64)> for Vector {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self::new(x, y, z, None)
    }
}

impl From<(f64, f64, f64, f64)> for Vector {
    fn from((x, y, z, w): (f64, f64, f64, f64)) -> Self {
        Self::new(x, y, z, Some(w))
    }
}

impl Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Invalid index"),
        }
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Invalid index"),
        }
    }
}
