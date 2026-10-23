use std::{fmt, ops};
use crate::FSize;

#[macro_export]
macro_rules! m {
    (r[$($x:expr),+ $(,)?]; $n:expr) => {
        $crate::maths::matrices::M::new_m([[$($x as $crate::FSize),+]; $n])
    };
    (c[$($x:expr),+ $(,)?]; $n:expr) => {
        $crate::maths::matrices::M::new_m([$([$x as $crate::FSize; $n]),+])
    };
    ($elem:expr; $r:expr, $c:expr) => {
        $crate::maths::matrices::M::new_m([[$elem as $crate::FSize; $c]; $r])
    };
    ($([$($x:expr),+ $(,)?]),+ $(,)?) => {
        $crate::maths::matrices::M::new_m([
            $([$($x as $crate::FSize),+]),+
        ])
    };
}

#[derive(Clone, Copy, PartialEq)]
pub struct M<const R: usize, const C: usize> {
    pub data: [[FSize; C]; R],
}

pub type M4 = M<4,4>;

impl<const R: usize, const C: usize> M<R, C> {
    pub const ZERO: Self = M {
        data: [[0.0; C]; R],
    };

    pub const ONE: Self = M {
        data: [[1.0; C]; R],
    };

    pub const fn new_m(data: [[FSize; C]; R]) -> Self {
        Self { data }
    }
}

impl<const R: usize, const C: usize> Default for M<R,C> {
    fn default() -> Self {
        Self::new_m([[0 as FSize; C]; R])
    }
}

const DEFAULT_DISPLAY_PRECISION: usize = 3;

impl<const R: usize, const C: usize> fmt::Debug for M<R, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "M<{R},{C}>")?;
        f.write_str("[")?;
        for (i, row) in self.data.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            f.write_str("[")?;
            for (j, x) in row.iter().enumerate() {
                if j > 0 {
                    f.write_str(", ")?;
                }
                match f.precision() {
                    Some(p) => write!(f, "{:.*}", p, x)?,
                    None => write!(f, "{}", x)?,
                }
            }
            f.write_str("]")?;
        }
        f.write_str("]")
    }
}

fn fmt_cell(x: FSize, precision: usize) -> String {
    let s = format!("{:.*}", precision, x);
    match s.strip_prefix('-') {
        Some(rest) if rest.bytes().all(|b| b == b'0' || b == b'.') => rest.to_owned(),
        _ => s,
    }
}

impl<const R: usize, const C: usize> fmt::Display for M<R, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let precision = f.precision().unwrap_or(DEFAULT_DISPLAY_PRECISION);

        let cells: [[String; C]; R] = std::array::from_fn(|i| {
            std::array::from_fn(|j| fmt_cell(self.data[i][j], precision))
        });

        let mut widths = [0usize; C];
        for row in &cells {
            for (w, s) in widths.iter_mut().zip(row) {
                *w = (*w).max(s.len());
            }
        }

        for (i, row) in cells.iter().enumerate() {
            if i == 0 {
                f.write_str("[")?;
            }
            else {
                writeln!(f)?;
                f.write_str(" ")?;
            }

            for (j, (s, w)) in row.iter().zip(widths).enumerate() {
                if j > 0 {
                    f.write_str("  ")?;
                }
                write!(f, "{:>w$}", s)?;
            }

            if i == R - 1 {
                f.write_str("]")?;
            }
            else {
                f.write_str(" ")?;
            }
        }
        Ok(())
    }
}

impl<const R: usize, const K: usize> M<R, K> {
    /// Matrix product: (R x K) * (K x C) -> (R x C)
    pub fn cross<const C: usize>(&self, rhs: &M<K, C>) -> M<R, C> {
        let mut data = [[0.0; C]; R];
        for i in 0..R {
            for k in 0..K {
                let a = self.data[i][k];
                for j in 0..C {
                    data[i][j] += a * rhs.data[k][j];
                }
            }
        }
        M { data }
    }
}

impl<const R: usize, const C: usize> ops::Add<M<R, C>> for M<R, C> {
    type Output = M<R, C>;

    fn add(self, rhs: M<R, C>) -> M<R, C> {
        let mut data = [[0.0; C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        M { data }
    }
}

impl<const R: usize, const C: usize> ops::Add<&M<R, C>> for M<R, C> {
    type Output = M<R, C>;

    fn add(self, rhs: &M<R, C>) -> M<R, C> {
        let mut data = [[0.0; C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        M { data }
    }
}

impl<const R: usize, const C: usize> ops::Add<M<R, C>> for &M<R, C> {
    type Output = M<R, C>;

    fn add(self, rhs: M<R, C>) -> M<R, C> {
        let mut data = [[0.0; C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        M { data }
    }
}

impl<const R: usize, const C: usize> ops::Add<&M<R, C>> for &M<R, C> {
    type Output = M<R, C>;

    fn add(self, rhs: &M<R, C>) -> M<R, C> {
        let mut data = [[0.0; C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        M { data }
    }
}

impl<const R: usize, const C: usize> ops::Sub<M<R, C>> for M<R, C> {
    type Output = M<R, C>;

    fn sub(self, rhs: M<R, C>) -> M<R, C> {
        let mut data = [[0.0; C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        M { data }
    }
}

impl<const R: usize, const C: usize> ops::Sub<&M<R, C>> for M<R, C> {
    type Output = M<R, C>;

    fn sub(self, rhs: &M<R, C>) -> M<R, C> {
        let mut data = [[0.0; C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        M { data }
    }
}

impl<const R: usize, const C: usize> ops::Sub<M<R, C>> for &M<R, C> {
    type Output = M<R, C>;

    fn sub(self, rhs: M<R, C>) -> M<R, C> {
        let mut data = [[0.0; C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        M { data }
    }
}

impl<const R: usize, const C: usize> ops::Sub<&M<R, C>> for &M<R, C> {
    type Output = M<R, C>;

    fn sub(self, rhs: &M<R, C>) -> M<R, C> {
        let mut data = [[0.0; C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        M { data }
    }
}

impl<const R: usize, const C: usize> ops::Mul<FSize> for M<R, C> {
    type Output = M<R, C>;

    fn mul(self, rhs: FSize) -> M<R, C> {
        let mut data = [[0.0; C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = self.data[i][j] * rhs;
            }
        }
        M { data }
    }
}

impl<const R: usize, const C: usize> ops::Mul<FSize> for &M<R, C> {
    type Output = M<R, C>;

    fn mul(self, rhs: FSize) -> M<R, C> {
        let mut data = [[0.0; C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = self.data[i][j] * rhs;
            }
        }
        M { data }
    }
}
impl<const R: usize, const C: usize> ops::Div<FSize> for M<R, C> {
    type Output = M<R, C>;

    fn div(self, rhs: FSize) -> M<R, C> {
        let mut data = [[0.0; C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = self.data[i][j] / rhs;
            }
        }
        M { data }
    }
}

impl<const R: usize, const C: usize> ops::Div<FSize> for &M<R, C> {
    type Output = M<R, C>;

    fn div(self, rhs: FSize) -> M<R, C> {
        let mut data = [[0.0; C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = self.data[i][j] / rhs;
            }
        }
        M { data }
    }
}