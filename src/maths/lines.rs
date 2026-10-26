use super::vector::V3;
use crate::FSize;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub origin: V3,
    pub direction: V3,
}

#[allow(dead_code)]
impl Line {
    pub fn new(point: V3, vector: V3) -> Self {
        Self {
            origin: point,
            direction: vector,
        }
    }

    pub fn scale(&self, scale: FSize) -> V3 {
        self.origin + (self.direction * scale)
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
    pub fn new(point: V3, vector: V3, scale: FSize) -> Self {
        Self {
            line: Line::new(point, vector),
            scale,
        }
    }

    pub fn from_line(line: &Line, scale: FSize) -> Self {
        Self { line: *line, scale }
    }

    pub fn length(&self) -> FSize {
        self.line.direction.magnitude() * self.scale
    }
}
