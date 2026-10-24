use crate::FSize;
use crate::maths::matrix::M;

#[macro_export]
macro_rules! v {
    ($elem:expr; $n:expr) => {
        $crate::maths::vector::V::new_v([$elem as $crate::FSize; $n])
    };
    ($($x:expr),+ $(,)?) => {
        $crate::maths::vector::V::new_v([$($x as $crate::FSize),+])
    };
}

pub type V<const N: usize> = M<N, 1>;

impl<const N: usize> V<N> {
    pub fn new_v(data: [FSize; N]) -> V<N> {
        Self::new_m(data.map(|x| [x]))
    }

    pub fn dot(&self, rhs: &V<N>) -> FSize {
        self.data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a[0] * b[0])
            .sum()
    }

    pub fn magnitude(&self) -> FSize {
        self.dot(self).sqrt()
    }

    pub fn normalised(&self) -> V<N> {
        self * (1.0 / self.magnitude())
    }

    pub fn cos_angle_to(&self, rhs: &V<N>) -> FSize {
        self.dot(rhs) / (self.magnitude() * rhs.magnitude())
    }

    pub fn angle_to(&self, rhs: &V<N>) -> FSize {
        self.cos_angle_to(rhs).acos()
    }

    pub fn reflected(&self, normal: &V<N>) -> V<N> {
        let n = normal.normalised();
        *self - (n * (2.0 * (self.dot(&n))))
    }
}

pub type V2 = V<2>;
impl V2 {
    pub const fn new_v2(x: FSize, y: FSize) -> Self {
        Self { data: [[x], [y]] }
    }

    pub const fn x(&self) -> FSize {
        self.data[0][0]
    }

    pub const fn y(&self) -> FSize {
        self.data[1][0]
    }

    pub const fn x_mut(&mut self) -> &mut FSize {
        &mut self.data[0][0]
    }

    pub const fn y_mut(&mut self) -> &mut FSize {
        &mut self.data[1][0]
    }

    pub const UP: V2 = V2 {
        data: [[0.0], [1.0]],
    };

    pub const DOWN: V2 = V2 {
        data: [[0.0], [-1.0]],
    };

    pub const LEFT: V2 = V2 {
        data: [[-1.0], [0.0]],
    };

    pub const RIGHT: V2 = V2 {
        data: [[1.0], [0.0]],
    };
}

pub type V3 = V<3>;

impl V3 {
    pub const fn new_v3(x: FSize, y: FSize, z: FSize) -> Self {
        Self {
            data: [[x], [y], [z]],
        }
    }

    pub const fn x(&self) -> FSize {
        self.data[0][0]
    }

    pub const fn y(&self) -> FSize {
        self.data[1][0]
    }

    pub const fn z(&self) -> FSize {
        self.data[2][0]
    }

    pub const fn x_mut(&mut self) -> &mut FSize {
        &mut self.data[0][0]
    }

    pub const fn y_mut(&mut self) -> &mut FSize {
        &mut self.data[1][0]
    }

    pub const fn z_mut(&mut self) -> &mut FSize {
        &mut self.data[2][0]
    }

    pub fn rotate_y(self, about: &V3, radians: FSize) -> V3 {
        let centered = self - about;
        V3::new_v3(
            centered.x() * radians.cos() + centered.z() * radians.sin(),
            centered.y(),
            -centered.x() * radians.sin() + centered.z() * radians.cos(),
        ) + about
    }

    pub fn rotate_z(self, about: &V3, radians: FSize) -> V3 {
        let centered = self - about;
        V3::new_v3(
            centered.x() * radians.cos() - centered.y() * radians.sin(),
            centered.x() * radians.sin() + centered.y() * radians.cos(),
            centered.z(),
        ) + about
    }

    pub const FORWARD: V3 = V3 {
        data: [[1.0], [0.0], [0.0]],
    };

    pub const BACK: V3 = V3 {
        data: [[-1.0], [0.0], [0.0]],
    };

    pub const UP: V3 = V3 {
        data: [[0.0], [1.0], [0.0]],
    };

    pub const DOWN: V3 = V3 {
        data: [[0.0], [-1.0], [0.0]],
    };

    pub const LEFT: V3 = V3 {
        data: [[0.0], [0.0], [-1.0]],
    };

    pub const RIGHT: V3 = V3 {
        data: [[0.0], [0.0], [1.0]],
    };
}

pub type V4 = V<4>;

impl V4 {
    pub fn new_v4(x: FSize, y: FSize, z: FSize, w: FSize) -> Self {
        Self {
            data: [[x], [y], [z], [w]],
        }
    }

    pub fn x(&self) -> FSize {
        self.data[0][0]
    }

    pub fn y(&self) -> FSize {
        self.data[1][0]
    }

    pub fn z(&self) -> FSize {
        self.data[2][0]
    }

    pub fn w(&self) -> FSize {
        self.data[3][0]
    }

    pub fn x_mut(&mut self) -> &mut FSize {
        &mut self.data[0][0]
    }

    pub fn y_mut(&mut self) -> &mut FSize {
        &mut self.data[1][0]
    }

    pub fn z_mut(&mut self) -> &mut FSize {
        &mut self.data[2][0]
    }

    pub fn w_mut(&mut self) -> &mut FSize {
        &mut self.data[3][0]
    }
}
