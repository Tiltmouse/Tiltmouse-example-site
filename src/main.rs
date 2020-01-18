extern crate kiss3d;
extern crate nalgebra;

use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use nalgebra::geometry::Quaternion;
use nalgebra::{Point3, UnitQuaternion, Vector3};
use std::vec;

struct AppState {
    cube: SceneNode,
    sphere: SceneNode,
    time: f32,
    vec: Vec<Vec<[f32; 3]>>,
    rot: UnitQuaternion<f32>,
}

impl State for AppState {
    fn step(&mut self, window: &mut Window) {
        self.cube
            .set_local_rotation(UnitQuaternion::new(Vector3::new(0.0, 0.0, self.time)));

        // todo: add reorient

        for x in &self.vec {
            window.draw_line(
                &Point3::new(x[0][0], x[0][1], x[0][2]),
                &Point3::new(x[1][0], x[1][1], x[1][2]),
                &Point3::new(x[2][0], x[2][1], x[2][2]),
            );
        }

        self.time += 0.01;
    }
}

fn main() {
    let mut window = Window::new("");
    window.set_background_color(0.2, 0.8, 1.0);
    window.set_light(Light::StickToCamera);

    let cxyz = [8.0, 70.0, 41.5];
    let size_cube = 0.01;

    let mut sphere = window.add_sphere(0.1);
    let mut cube = window.add_cube(
        cxyz[0] * size_cube,
        cxyz[1] * size_cube,
        cxyz[2] * size_cube,
    );

    cube.set_color(1.0, 0.0, 0.0);

    let size_step = 2.0;
    let count_step = 4;
    let startp = size_step / 2.0 * (count_step as f32 - 1.0);

    let mut vec: Vec<Vec<[f32; 3]>> = Vec::new();
    // тута линии
    for i in 0..count_step {
        let x = i as f32;
        vec.push(vec![
            [startp - size_step * x, 0.0, -startp],
            [startp - size_step * x, 0.0, startp],
            [0.25, 0.25, 0.25],
        ]);

        vec.push(vec![
            [-startp, 0.0, startp - size_step * x],
            [startp, 0.0, startp - size_step * x],
            [0.25, 0.25, 0.25],
        ]);
    }

    let time: f32 = 0.0;

    let rot = UnitQuaternion::new_normalize(Quaternion::new(1.0, 0.0, 0.0, 0.0));

    let state = AppState {
        cube,
        sphere,
        time,
        vec,
        rot,
    };

    window.render_loop(state)
}
