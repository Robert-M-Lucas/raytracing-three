use crate::{
    FSize,
    maths::{
        lines::Line,
        matrix::M4,
        vector::{V3, V4},
    },
};

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub min: V3,
    pub max: V3,
}

impl Aabb {
    pub fn new(min: V3, max: V3) -> Aabb {
        Aabb { min, max }
    }

    pub fn centre(&self) -> V3 {
        (self.min + self.max) / 2 as FSize
    }

    pub fn surface_area(&self) -> FSize {
        let d = self.max - self.min;
        2.0 * (d.x() * d.y() + d.y() * d.z() + d.z() * d.x())
    }

    pub fn union(&self, other: &Aabb) -> Aabb {
        Aabb {
            min: V3::new_v3(
                self.min.x().min(other.min.x()),
                self.min.y().min(other.min.y()),
                self.min.z().min(other.min.z()),
            ),
            max: V3::new_v3(
                self.max.x().max(other.max.x()),
                self.max.y().max(other.max.y()),
                self.max.z().max(other.max.z()),
            ),
        }
    }

    pub fn transform(&self, m: &M4) -> Aabb {
        let corners = [
            V3::new_v3(self.min.x(), self.min.y(), self.min.z()),
            V3::new_v3(self.max.x(), self.min.y(), self.min.z()),
            V3::new_v3(self.min.x(), self.max.y(), self.min.z()),
            V3::new_v3(self.max.x(), self.max.y(), self.min.z()),
            V3::new_v3(self.min.x(), self.min.y(), self.max.z()),
            V3::new_v3(self.max.x(), self.min.y(), self.max.z()),
            V3::new_v3(self.min.x(), self.max.y(), self.max.z()),
            V3::new_v3(self.max.x(), self.max.y(), self.max.z()),
        ];
        let mut min = V3::new_v3(FSize::INFINITY, FSize::INFINITY, FSize::INFINITY);
        let mut max = V3::new_v3(
            FSize::NEG_INFINITY,
            FSize::NEG_INFINITY,
            FSize::NEG_INFINITY,
        );

        for c in &corners {
            let v4 = m.cross(&V4::new_v4(c.x(), c.y(), c.z(), 1.0));
            let w = v4.w();
            let pt = if w != 0.0 {
                V3::new_v3(v4.x() / w, v4.y() / w, v4.z() / w)
            } else {
                V3::new_v3(v4.x(), v4.y(), v4.z())
            };
            min = V3::new_v3(
                min.x().min(pt.x()),
                min.y().min(pt.y()),
                min.z().min(pt.z()),
            );
            max = V3::new_v3(
                max.x().max(pt.x()),
                max.y().max(pt.y()),
                max.z().max(pt.z()),
            );
        }
        Aabb::new(min, max)
    }

    pub fn intersect(&self, ray: &Line, mut t_min: FSize, mut t_max: FSize) -> bool {
        for i in 0..3 {
            let origin = ray.origin.data[i][0];
            let dir = ray.direction.data[i][0];
            let min = self.min.data[i][0];
            let max = self.max.data[i][0];

            // Parallel to this slab - must already be inside it
            if dir.abs() < 1e-8 {
                if origin < min || origin > max {
                    return false;
                }
                continue;
            }

            let inv_d = 1.0 / dir;
            let mut t0 = (min - origin) * inv_d;
            let mut t1 = (max - origin) * inv_d;

            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max < t_min {
                return false;
            }
        }

        true
    }
}
