use crate::{
    FSize,
    maths::vector::V3,
    scene::{AABB, geometry::Geometry},
};

#[derive(Clone, Debug)]
pub struct Sphere {
    pub radius: FSize,
}

impl Geometry for Sphere {
    fn bounding_box(&self) -> AABB {
        AABB {
            min: V3::new_v3(-self.radius, -self.radius, -self.radius),
            max: V3::new_v3(self.radius, self.radius, self.radius),
        }
    }

    fn clone_box(&self) -> Box<dyn Geometry + Send + Sync> {
        Box::new(self.clone())
    }
}
