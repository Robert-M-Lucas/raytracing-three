use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

use crate::resolutions::Resolution;

#[derive(Clone)]
pub struct Buffer {
    pub pixel_arena: Arc<[AtomicU32]>,
    width: usize,
    height: usize,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        if width == 0 || height == 0 {
            panic!("Buffer axes cannot be 0");
        }

        let len = width
            .checked_mul(height)
            .expect("buffer dimensions overflow usize");
        let pixel_arena: Arc<[AtomicU32]> = std::iter::repeat_with(|| AtomicU32::new(0xFFFF00FF))
            .take(len)
            .collect();
        Self {
            pixel_arena,
            width,
            height,
        }
    }

    pub fn from_resolution(resolution: Resolution) -> Self {
        Buffer::new(resolution.0 as usize, resolution.1 as usize)
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn resolution(&self) -> Resolution {
        (self.width as u32, self.height as u32)
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
        for (dst, src) in out
            .as_chunks_mut::<4>()
            .0
            .iter_mut()
            .zip(self.pixel_arena.iter())
        {
            dst.copy_from_slice(&src.load(Ordering::Relaxed).to_le_bytes());
        }
    }

    pub fn to_rgba_vec(&self) -> Vec<u8> {
        let mut out = vec![0u8; self.pixel_arena.len() * 4];
        self.copy_to_rgba(&mut out);
        out
    }

    pub fn to_rgba_vec_sdl2(&self) -> (Resolution, Vec<u8>) {
        let res = self.resolution();
        let mut divider = 1;

        while res.0 / divider > 16384 || res.1 / divider > 16384 {
            // SDL2 limits
            divider += 1;
        }

        let res = ((res.0 / divider).max(1), (res.1 / divider.max(1)));
        let mut out = Vec::with_capacity((res.0 * res.1 * 4) as usize);

        let mut y = 0;
        while y < self.width as u32 {
            let mut x = 0;
            while x < self.height as u32 {
                out.extend_from_slice(
                    &self.pixel_arena[(y * self.width as u32 + x) as usize]
                        .load(Ordering::Relaxed)
                        .to_le_bytes(),
                );
                x += divider;
            }
            y += divider;
        }

        (res, out)
    }
}
