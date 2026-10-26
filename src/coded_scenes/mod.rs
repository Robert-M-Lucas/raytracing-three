use crate::{
    FSize,
    maths::matrix::M4,
    scene::{
        camera::Camera,
        geometry::{Geometry, Sphere},
        material::Material,
        scene_node::SceneNode,
        scene_object::SceneObject,
        texture::Texture,
    },
    v,
};

pub fn default_scene() -> SceneNode {
    SceneNode::new_empty()
        .with_child(Camera::new(90 as FSize).scene_node())
        // .with_child(Camera::default().scene_node())
        .with_child(
            SceneObject::new(
                Geometry::Sphere(Sphere { radius: 3 as FSize }),
                Material::lambert(Texture::Solid(v![1.0, 0.0, 0.0])),
            )
            .scene_node()
            .with_transform(M4::translation(5.0, 0.0, 0.0)),
        )
        .with_child(
            SceneObject::new(
                Geometry::Sphere(Sphere { radius: 3 as FSize }),
                Material::lambert(Texture::Solid(v![0.0, 1.0, 0.0])),
            )
            .scene_node()
            .with_transform(M4::translation(0.0, 5.0, 0.0)),
        )
        .with_child(
            SceneObject::new(
                Geometry::Sphere(Sphere { radius: 3 as FSize }),
                Material::lambert(Texture::Solid(v![0.0, 0.0, 1.0])),
            )
            .scene_node()
            .with_transform(M4::translation(0.0, 0.0, 5.0)),
        )
        .with_child(
            SceneObject::new(
                Geometry::Sphere(Sphere { radius: 3 as FSize }),
                Material::lambert(Texture::Solid(v![1.0, 1.0, 0.0])),
            )
            .scene_node()
            .with_transform(M4::translation(-5.0, 0.0, 0.0)),
        )
        .with_child(
            SceneObject::new(
                Geometry::Sphere(Sphere { radius: 3 as FSize }),
                Material::lambert(Texture::Solid(v![0.0, 1.0, 1.0])),
            )
            .scene_node()
            .with_transform(M4::translation(0.0, -5.0, 0.0)),
        )
        .with_child(
            SceneObject::new(
                Geometry::Sphere(Sphere { radius: 3 as FSize }),
                Material::lambert(Texture::Solid(v![1.0, 0.0, 1.0])),
            )
            .scene_node()
            .with_transform(M4::translation(0.0, 0.0, -5.0)),
        )
}
