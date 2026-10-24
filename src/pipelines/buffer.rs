use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

#[derive(Clone)]
pub struct Buffer {
    pub pixel_arena: Arc<[AtomicU32]>,
    width: usize,
    height: usize,
}

impl Buffer {
    /// Creates a zeroed (transparent black) buffer.
    pub fn new(width: usize, height: usize) -> Self {
        let len = width
            .checked_mul(height)
            .expect("buffer dimensions overflow usize");
        let pixel_arena: Arc<[AtomicU32]> = std::iter::repeat_with(|| AtomicU32::new(0))
            .take(len)
            .collect();
        Self {
            pixel_arena,
            width,
            height,
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    fn index(&self, x: usize, y: usize) -> usize {
        assert!(
            x < self.width && y < self.height,
            "pixel ({x}, {y}) out of bounds"
        );
        y * self.width + x
    }

    #[inline]
    pub fn set_pixel(&self, x: usize, y: usize, rgba: [u8; 4]) {
        let i = self.index(x, y);
        self.pixel_arena[i].store(u32::from_le_bytes(rgba), Ordering::Relaxed);
    }

    #[inline]
    pub fn get_pixel(&self, x: usize, y: usize) -> [u8; 4] {
        let i = self.index(x, y);
        self.pixel_arena[i].load(Ordering::Relaxed).to_le_bytes()
    }

    pub fn write_row(&self, y: usize, row: &[[u8; 4]]) {
        assert!(y < self.height, "row {y} out of bounds");
        assert_eq!(row.len(), self.width, "row length must match buffer width");
        let start = y * self.width;
        for (slot, px) in self.pixel_arena[start..start + self.width].iter().zip(row) {
            slot.store(u32::from_le_bytes(*px), Ordering::Relaxed);
        }
    }
    pub fn fill(&self, rgba: [u8; 4]) {
        let v = u32::from_le_bytes(rgba);
        for slot in self.pixel_arena.iter() {
            slot.store(v, Ordering::Relaxed);
        }
    }

    /// Not an atomic snapshot!
    pub fn copy_to_rgba(&self, out: &mut [u8]) {
        assert_eq!(
            out.len(),
            self.pixel_arena.len() * 4,
            "output length mismatch"
        );
        for (dst, src) in out.as_chunks_mut::<4>().0.iter_mut().zip(self.pixel_arena.iter()) {
            dst.copy_from_slice(&src.load(Ordering::Relaxed).to_le_bytes());
        }
    }

    pub fn to_rgba_vec(&self) -> Vec<u8> {
        let mut out = vec![0u8; self.pixel_arena.len() * 4];
        self.copy_to_rgba(&mut out);
        out
    }
}
