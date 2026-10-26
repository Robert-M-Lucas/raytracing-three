use crate::{CUSTOM_UNWRAP_BEHAVIOUR, CustomUnwrapBehaviour, FSize, maths::vector::V3};
use std::{fmt, ops};

#[macro_export]
macro_rules! m {
    (r[$($x:expr),+ $(,)?]; $n:expr) => {
        $crate::maths::matrix::M::new_m([[$($x as $crate::FSize),+]; $n])
    };
    (c[$($x:expr),+ $(,)?]; $n:expr) => {
        $crate::maths::matrix::M::new_m([$([$x as $crate::FSize; $n]),+])
    };
    ($elem:expr; $r:expr, $c:expr) => {
        $crate::maths::matrix::M::new_m([[$elem as $crate::FSize; $c]; $r])
    };
    ($([$($x:expr),+ $(,)?]),+ $(,)?) => {
        $crate::maths::matrix::M::new_m([
            $([$($x as $crate::FSize),+]),+
        ])
    };
}

#[derive(Clone, Copy, PartialEq)]
pub struct M<const R: usize, const C: usize> {
    pub data: [[FSize; C]; R],
}

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

impl<const R: usize, const C: usize> Default for M<R, C> {
    fn default() -> Self {
        Self::new_m([[0 as FSize; C]; R])
    }
}

impl<const N: usize> M<N, N> {
    pub fn inverse(&self) -> Option<Self> {
        let mut a = self.data;
        let mut inv = [[0.0; N]; N];
        for i in 0..N {
            inv[i][i] = 1.0;
        }

        for i in 0..N {
            let mut max_row = i;
            let mut max_val = a[i][i].abs();
            for r in (i + 1)..N {
                let val = a[r][i].abs();
                if val > max_val {
                    max_val = val;
                    max_row = r;
                }
            }

            if max_val == 0.0 {
                return None;
            }

            if max_row != i {
                a.swap(i, max_row);
                inv.swap(i, max_row);
            }

            let pivot = a[i][i];
            for j in 0..N {
                a[i][j] /= pivot;
                inv[i][j] /= pivot;
            }

            for r in 0..N {
                if r != i {
                    let factor = a[r][i];
                    for j in 0..N {
                        a[r][j] -= factor * a[i][j];
                        inv[r][j] -= factor * inv[i][j];
                    }
                }
            }
        }

        Some(Self { data: inv })
    }
}

pub trait UnwrapMatrix<const N: usize> {
    fn none_panic_or_identity(self) -> M<N, N>;
}

const DEFAULT_DISPLAY_PRECISION: usize = 3;

impl<const R: usize, const C: usize> fmt::Debug for M<R, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if C > 1 {
            write!(f, "M<{R},{C}>")?;
        } else {
            write!(f, "V<{R}>")?;
        }

        f.write_str("[")?;
        for (i, row) in self.data.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            if C > 1 {
                f.write_str("[")?;
            }
            for (j, x) in row.iter().enumerate() {
                if j > 0 {
                    f.write_str(", ")?;
                }
                match f.precision() {
                    Some(p) => write!(f, "{:.*}", p, x)?,
                    None => write!(f, "{}", x)?,
                }
            }
            if C > 1 {
                f.write_str("]")?;
            }
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

        let cells: [[String; C]; R] =
            std::array::from_fn(|i| std::array::from_fn(|j| fmt_cell(self.data[i][j], precision)));

        let mut widths = [0usize; C];
        for row in &cells {
            for (w, s) in widths.iter_mut().zip(row) {
                *w = (*w).max(s.len());
            }
        }

        for (i, row) in cells.iter().enumerate() {
            if i == 0 {
                f.write_str("[")?;
            } else {
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
            } else {
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

pub type M4 = M<4, 4>;

const CAMERA_PITCH_LIMIT: FSize = std::f64::consts::FRAC_PI_2 as FSize - 0.001;
impl M4 {
    pub fn translation(x: FSize, y: FSize, z: FSize) -> Self {
        let mut m = Self::IDENTITY;
        m.data[0][3] = x;
        m.data[1][3] = y;
        m.data[2][3] = z;
        m
    }

    pub fn scaling(x: FSize, y: FSize, z: FSize) -> Self {
        let mut m = Self::IDENTITY;
        m.data[0][0] = x;
        m.data[1][1] = y;
        m.data[2][2] = z;
        m
    }

    pub fn rotation_x(angle: FSize) -> Self {
        let (sin, cos) = angle.sin_cos();
        let mut m = Self::IDENTITY;
        m.data[1][1] = cos;
        m.data[1][2] = -sin;
        m.data[2][1] = sin;
        m.data[2][2] = cos;
        m
    }

    pub fn rotation_y(angle: FSize) -> Self {
        let (sin, cos) = angle.sin_cos();
        let mut m = Self::IDENTITY;
        m.data[0][0] = cos;
        m.data[0][2] = sin;
        m.data[2][0] = -sin;
        m.data[2][2] = cos;
        m
    }

    pub fn rotation_z(angle: FSize) -> Self {
        let (sin, cos) = angle.sin_cos();
        let mut m = Self::IDENTITY;
        m.data[0][0] = cos;
        m.data[0][1] = -sin;
        m.data[1][0] = sin;
        m.data[1][1] = cos;
        m
    }

    pub fn translated(&self, x: FSize, y: FSize, z: FSize) -> Self {
        self.cross(&Self::translation(x, y, z))
    }

    pub fn position(&self) -> (FSize, FSize, FSize) {
        (self.data[0][3], self.data[1][3], self.data[2][3])
    }

    pub fn set_position(&mut self, x: FSize, y: FSize, z: FSize) {
        self.data[0][3] = x;
        self.data[1][3] = y;
        self.data[2][3] = z;
    }

    pub fn translate_world(&mut self, x: FSize, y: FSize, z: FSize) {
        self.data[0][3] += x;
        self.data[1][3] += y;
        self.data[2][3] += z;
    }

    pub fn yaw(&self) -> FSize {
        (-self.data[2][0]).atan2(self.data[0][0])
    }

    pub fn pitch(&self) -> FSize {
        (-self.data[1][2]).atan2(self.data[1][1])
    }

    pub fn fps_camera(x: FSize, y: FSize, z: FSize, yaw: FSize, pitch: FSize) -> Self {
        Self::translation(x, y, z)
            .rotated_y(yaw)
            .rotated_x(pitch.clamp(-CAMERA_PITCH_LIMIT, CAMERA_PITCH_LIMIT))
    }

    pub fn fps_look(&mut self, d_yaw: FSize, d_pitch: FSize) {
        let (x, y, z) = self.position();
        *self = Self::fps_camera(x, y, z, self.yaw() + d_yaw, self.pitch() + d_pitch);
    }

    pub fn fps_translate(&mut self, right: FSize, up: FSize, forward: FSize) {
        let (s, c) = self.yaw().sin_cos();
        self.translate_world(c * right - s * forward, up, -s * right - c * forward);
    }

    pub fn scaled(&self, x: FSize, y: FSize, z: FSize) -> Self {
        self.cross(&Self::scaling(x, y, z))
    }
    pub fn rotated_x(&self, angle: FSize) -> Self {
        self.cross(&Self::rotation_x(angle))
    }
    pub fn rotated_y(&self, angle: FSize) -> Self {
        self.cross(&Self::rotation_y(angle))
    }

    pub fn rotated_z(&self, angle: FSize) -> Self {
        self.cross(&Self::rotation_z(angle))
    }

    pub fn transform_v3_w0(&self, v3: &V3) -> V3 {
        self.cross(&v3.v4_w0()).v3()
    }

    pub fn transform_v3_w1(&self, v3: &V3) -> V3 {
        self.cross(&v3.v4_w1()).div_w_v3()
    }

    pub const IDENTITY: M4 = M4 {
        data: [
            [1 as FSize, 0 as FSize, 0 as FSize, 0 as FSize],
            [0 as FSize, 1 as FSize, 0 as FSize, 0 as FSize],
            [0 as FSize, 0 as FSize, 1 as FSize, 0 as FSize],
            [0 as FSize, 0 as FSize, 0 as FSize, 1 as FSize],
        ],
    };
}

impl UnwrapMatrix<4> for Option<M4> {
    fn none_panic_or_identity(self) -> M4 {
        if let Some(m) = self {
            m
        } else {
            if CUSTOM_UNWRAP_BEHAVIOUR == CustomUnwrapBehaviour::Panic {
                panic!("Unwrapping None Matrix");
            } else {
                M4::IDENTITY
            }
        }
    }
}
