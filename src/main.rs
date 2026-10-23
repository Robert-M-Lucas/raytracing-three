use crate::interface::Interface;
use crate::maths::vectors::V3;
use crate::rendering::camera::Camera;
use crate::rendering::render_config::RenderConfig;
use crate::utils::{_1080P, _240P, _720P};

mod interface;
mod utils;
mod rendering;
mod maths;
mod scene;

pub type FSize = f32;

fn main() {
    let m1 = m![c[1, 2, 3]; 2];
    let m2 = m![r[1, 2, 3]; 2];

    println!("{m1:?} {m2:?}");
    println!("{m1}\n{m2}");

    let render_config: RenderConfig = RenderConfig {
        resolution: _240P,
        screenshot_resolution: _1080P
    };

    let camera = Camera::new(&V3::new_v3(-5.0, 1.0, 0.0), (0.0, 0.0), 0.866);

    Interface::start(render_config, camera, _720P);
}
