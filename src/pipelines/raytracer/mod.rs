use std::sync::{Arc, Mutex};

use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use crate::{
    FSize,
    maths::{
        lines::Line,
        matrix::{M4, UnwrapMatrix},
        vector::{V3, V4},
    },
    pipelines::{
        buffer::Buffer,
        render_config::{GlobalRenderConfig, RenderPipeline},
        shared::bvh::Bvh,
    },
    resolutions::Resolution,
    scene::{
        camera::Camera,
        geometry::{Geometry, Sphere, mesh::Mesh},
        scene_node::SceneNode,
    },
};

#[derive(Debug, Clone)]
pub struct RaytracerConfig {}

#[derive(Clone)]
struct RaytracerCache {
    bvh: Arc<Mutex<Option<Bvh>>>,
}

impl RaytracerCache {
    pub fn new() -> RaytracerCache {
        RaytracerCache {
            bvh: Arc::new(Mutex::new(None)),
        }
    }
}

pub fn raytracing_pipeline(config: RaytracerConfig) -> RenderPipeline {
    let cache = RaytracerCache::new();
    Box::new(move |grc, scene, buffer, print_progress| {
        raytrace(grc, config, scene, buffer, &cache, print_progress);
    })
}

fn raytrace(
    grc: GlobalRenderConfig,
    _config: RaytracerConfig,
    scene: SceneNode,
    buffer: Buffer,
    cache: &RaytracerCache,
    print_progress: bool,
) {
    let camera = scene
        .find_first_camera()
        .expect("Scene must contain a camera");
    let (camera_transform, camera) = (camera.transform, camera.camera().unwrap().clone());

    let mut cached = cache.bvh.lock().unwrap();
    if cached.is_none() {
        *cached = Some(Bvh::from_scene(scene))
    }
    let bvh = cached.as_ref().unwrap();

    if grc.multithread {
        // 16 * CPUs groups
        let pixel_group = (((buffer.width() * buffer.height()) / 16) / num_cpus::get()).max(1);
        let group_count =
            (((buffer.width() * buffer.height()) as FSize) / pixel_group as FSize).ceil() as u64;

        let pb = print_progress.then_some(ProgressBar::new(group_count));
        if let Some(pb) = pb.as_ref() {
            pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} chunks ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        )
        }

        (0..(buffer.width() * buffer.height()))
            .into_par_iter()
            .chunks(pixel_group)
            .for_each(|chunk| {
                for index in chunk {
                    let x = index % buffer.width();
                    let y = index / buffer.width();

                    let resolution = buffer.resolution();
                    buffer.set_pixel(
                        x,
                        y,
                        raytrace_pixel(&camera_transform, &camera, bvh, x, y, resolution),
                    );
                }
                if let Some(pb) = pb.as_ref() {
                    pb.inc(1)
                }
            });

        if let Some(pb) = pb.as_ref() {
            pb.finish_with_message("Render complete")
        }
    } else {
        let resolution = buffer.resolution();
        let pb = print_progress.then_some(ProgressBar::new(buffer.height() as u64));
        if let Some(pb) = pb.as_ref() {
            pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} rows ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        )
        }

        for y in 0..buffer.height() {
            for x in 0..buffer.width() {
                buffer.set_pixel(
                    x,
                    y,
                    raytrace_pixel(&camera_transform, &camera, bvh, x, y, resolution),
                );
            }
            if let Some(pb) = pb.as_ref() {
                pb.inc(1)
            }
        }

        if let Some(pb) = pb.as_ref() {
            pb.finish_with_message("Render complete")
        }
    }
}

pub fn raytrace_pixel(
    camera_transform: &M4,
    camera: &Camera,
    bvh: &Bvh,
    x: usize,
    y: usize,
    resolution: Resolution,
) -> [u8; 4] {
    // 1. Generate primary ray from camera
    let aspect = resolution.0 as FSize / resolution.1 as FSize;
    let fov_rad = camera.fov.to_radians();
    let scale = (fov_rad * 0.5).tan();

    let u = (2.0 * (x as FSize + 0.5) / resolution.0 as FSize - 1.0) * scale * aspect;
    let v = (1.0 - 2.0 * (y as FSize + 0.5) / resolution.1 as FSize) * scale;

    let dir_cam = V3::new_v3(-u, v, 1.0).normalised();

    // Transform ray origin and direction by camera transform
    let cam_pos = camera_transform.position();
    let origin = V3::new_v3(cam_pos.0, cam_pos.1, cam_pos.2);
    let transformed_dir_v4 =
        camera_transform.cross(&V4::new_v4(dir_cam.x(), dir_cam.y(), dir_cam.z(), 0.0));
    let direction = V3::new_v3(
        transformed_dir_v4.x(),
        transformed_dir_v4.y(),
        transformed_dir_v4.z(),
    )
    .normalised();

    let ray = Line::new(origin, direction);

    // 2. Traverse BVH and find the first hit
    let mut closest_t = FSize::INFINITY;
    let mut hit_found = false;

    for ((object_transform, scene_object), subobject_index) in
        bvh.intersect(ray, 0.001, FSize::INFINITY)
    {
        if let Some(t) = intersect_scene_object(
            &ray,
            &scene_object.geometry,
            subobject_index,
            object_transform,
        ) && t < closest_t
        {
            closest_t = t;
            hit_found = true;
        }
    }

    if hit_found {
        // Return a simple shaded color based on depth or a default surface color
        let intensity = (255.0 * (1.0 - (closest_t / 10.0).clamp(0.0, 1.0))).max(10.0) as u8;
        [intensity, intensity, intensity, 255]
    } else {
        [0, 20, 40, 255] // Background color
    }
}

pub fn intersect_scene_object(
    ray: &Line,
    geometry: &Geometry,
    subobject_index: u32,
    transform: &M4,
) -> Option<FSize> {
    match geometry {
        Geometry::Sphere(sphere) => intersect_sphere(ray, sphere, transform),
        Geometry::Quad => intersect_quad(ray, transform),
        Geometry::Disk => intersect_disk(ray, transform),
        Geometry::Cube => intersect_cube(ray, transform),
        Geometry::Mesh(mesh) => intersect_mesh(ray, mesh, subobject_index, transform),
    }
}

pub fn intersect_sphere(ray: &Line, sphere: &Sphere, transform: &M4) -> Option<FSize> {
    let inv_transform = transform.inverse().none_panic_or_identity();
    let local_origin = inv_transform.transform_v3_w1(&ray.origin);
    let local_dir = inv_transform.transform_v3_w0(&ray.direction);

    let radius = sphere.radius;
    let a = local_dir.dot(&local_dir);
    let b = 2.0 * local_origin.dot(&local_dir);
    let c = local_origin.dot(&local_origin) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    let sqrt_d = discriminant.sqrt();
    let t0 = (-b - sqrt_d) / (2.0 * a);
    let t1 = (-b + sqrt_d) / (2.0 * a);

    let t = if t0 > 0.001 { t0 } else { t1 };
    if t > 0.001 { Some(t) } else { None }
}

pub fn intersect_quad(ray: &Line, transform: &M4) -> Option<FSize> {
    let inv_transform = transform.inverse().none_panic_or_identity();
    let local_origin = inv_transform.transform_v3_w1(&ray.origin);
    let local_dir = inv_transform.transform_v3_w0(&ray.direction);

    // Quad is unit square in XZ plane (y = 0)
    if local_dir.y().abs() < 1e-6 {
        return None;
    }

    let t = -local_origin.y() / local_dir.y();
    if t < 0.001 {
        return None;
    }

    let hit_pos = local_origin + local_dir * t;
    if hit_pos.x().abs() <= 0.5 && hit_pos.z().abs() <= 0.5 {
        Some(t)
    } else {
        None
    }
}

pub fn intersect_disk(ray: &Line, transform: &M4) -> Option<FSize> {
    let inv_transform = transform.inverse().none_panic_or_identity();
    let local_origin = inv_transform.transform_v3_w1(&ray.origin);
    let local_dir = inv_transform.transform_v3_w0(&ray.direction);

    // Disk is unit-radius in XZ plane (y = 0)
    if local_dir.y().abs() < 1e-6 {
        return None;
    }

    let t = -local_origin.y() / local_dir.y();
    if t < 0.001 {
        return None;
    }

    let hit_pos = local_origin + local_dir * t;
    if hit_pos.x() * hit_pos.x() + hit_pos.z() * hit_pos.z() <= 1.0 {
        Some(t)
    } else {
        None
    }
}

pub fn intersect_cube(ray: &Line, transform: &M4) -> Option<FSize> {
    let inv_transform = transform.inverse().none_panic_or_identity();
    let local_origin = inv_transform.transform_v3_w1(&ray.origin);
    let local_dir = inv_transform.transform_v3_w0(&ray.direction);

    let mut t_min = 0.001;
    let mut t_max = FSize::INFINITY;

    for i in 0..3 {
        let origin = local_origin.data[i][0];
        let dir = local_dir.data[i][0];
        let inv_d = 1.0 / dir;
        let mut t0 = (-0.5 - origin) * inv_d;
        let mut t1 = (0.5 - origin) * inv_d;

        if inv_d < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }

        t_min = t0.max(t_min);
        t_max = t1.min(t_max);

        if t_max <= t_min {
            return None;
        }
    }

    Some(t_min)
}

pub fn intersect_mesh(
    ray: &Line,
    mesh: &Mesh,
    subobject_index: u32,
    transform: &M4,
) -> Option<FSize> {
    let tri = mesh.data.triangles[subobject_index as usize];
    let [i0, i1, i2] = tri.indices.map(|i| i as usize);

    let p0 = transform.transform_v3_w1(&mesh.data.vertices[i0].position);
    let p1 = transform.transform_v3_w1(&mesh.data.vertices[i1].position);
    let p2 = transform.transform_v3_w1(&mesh.data.vertices[i2].position);

    // Möller–Trumbore ray-triangle intersection
    let edge1 = p1 - p0;
    let edge2 = p2 - p0;
    let h = ray.direction.v3_cross(&edge2);
    let a = edge1.dot(&h);

    if a.abs() < 1e-6 {
        return None;
    }

    let f = 1.0 / a;
    let s = ray.origin - p0;
    let u = f * s.dot(&h);

    if u < 0.0 || u > 1.0 {
        return None;
    }

    let q = s.v3_cross(&edge1);
    let v = f * ray.direction.dot(&q);

    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t = f * edge2.dot(&q);
    if t > 0.001 { Some(t) } else { None }
}
