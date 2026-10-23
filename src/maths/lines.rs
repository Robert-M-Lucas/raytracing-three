use crate::FSize;
use super::vectors::V3;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub point: V3,
    pub vector: V3,
}

#[allow(dead_code)]
impl Line {
    pub fn new(point: &V3, vector: &V3) -> Self {
        Self {
            point: point.clone(),
            vector: vector.clone(),
        }
    }

    pub fn scale(&self, scale: FSize) -> V3 {
        return self.point + (self.vector * scale);
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct SizedLine {
    pub line: Line,
    pub scale: FSize,
}

#[allow(dead_code)]
impl SizedLine {
    pub fn new(point: &V3, vector: &V3, scale: FSize) -> Self {
        Self {
            line: Line::new(point, vector),
            scale,
        }
    }

    pub fn from_line(line: &Line, scale: FSize) -> Self {
        Self {
            line: line.clone(),
            scale,
        }
    }

    pub fn length(&self) -> FSize {
        self.line.vector.magnitude() * self.scale
    }
}
