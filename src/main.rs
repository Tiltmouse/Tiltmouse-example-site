extern crate kiss3d;
extern crate nalgebra;
extern crate rand;

use rand::Rng;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use nalgebra::geometry::Quaternion;
use nalgebra::{Point3, UnitQuaternion, Vector3};
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::vec;

// --

extern crate stdweb;

use std::sync::mpsc::Sender;
use stdweb::traits::{IEventTarget, IMessageEvent};
use stdweb::web::event::SocketMessageEvent;
use stdweb::web::WebSocket;

pub struct WebSocketWeb {}

impl WebSocketWeb {
    pub fn start(ip_port: &str, tx: Sender<String>) -> WebSocket {
        let mut socket = WebSocket::new(&("ws://".to_owned() + &ip_port)).unwrap();

        socket.add_event_listener(move |e: SocketMessageEvent| {
            let data = e.data().into_text().unwrap();
            stdweb::console!(log, data.clone());
            tx.send(data).unwrap();
        });

        socket
    }
}

// --

struct NodeSc {
    sn: SceneNode,
    id: f32,
}

impl NodeSc {
    pub fn new(sn: SceneNode, id: f32) -> NodeSc {
        NodeSc{sn: sn, id: id}
    }
}

struct AppState {
    nodes: Vec<NodeSc>,
    time: f32,
    vec: Vec<Vec<[f32; 3]>>,
    rx: Receiver<String>,
}

impl State for AppState {
    fn step(&mut self, window: &mut Window) {
        while let Ok(data) = self.rx.try_recv() {
            let data_arr = data.split(" ").collect::<Vec<_>>();
            
            if f32::from_str(data_arr[0]).unwrap() == 0.0 {
                stdweb::web::alert(data_arr[1]);
            }
            
            if f32::from_str(data_arr[0]).unwrap() == 1.0 {
                let mut id = f32::from_str(data_arr[7]).unwrap();

                let mut flag: bool = true;
                for mut noda in &mut self.nodes {
                    if noda.id == id {
                        
                        let new_pos = &Point3::new(
                            f32::from_str(data_arr[1]).unwrap(),
                            f32::from_str(data_arr[2]).unwrap(),
                            f32::from_str(data_arr[3]).unwrap(),
                        );

                        noda.sn.reorient(new_pos, new_pos, &Vector3::y_axis());

                        noda.sn.set_local_rotation(UnitQuaternion::new(Vector3::new(
                            f32::from_str(data_arr[4]).unwrap(),
                            f32::from_str(data_arr[5]).unwrap(),
                            f32::from_str(data_arr[6]).unwrap(),
                        )));

                        flag = false;
                        break;
                    }
                }

                if flag {
                    let cxyz = [8.0, 70.0, 41.5];
                    let size_cube = 0.01;

                    let mut kek = window.add_cube(
                        cxyz[0] * size_cube,
                        cxyz[1] * size_cube,
                        cxyz[2] * size_cube,
                    );
                    let mut cube = NodeSc::new(kek, id);
                    
                    let x: f32 = rand::random::<f32>() % 255.0;
                    let y: f32 = rand::random::<f32>() % 255.0;
                    let z: f32 = rand::random::<f32>() % 255.0;
                    
                    cube.sn.set_color(x, y, z);
                    self.nodes.push(cube);
                }
            }
        }

        // let new_pos = &Point3::new(self.time.sin(), 0.5, self.time.cos());
        // self.cube.reorient(new_pos, new_pos, &Vector3::y_axis());

        // self.cube
        //     .set_local_rotation(UnitQuaternion::new(Vector3::new(0.0, 0.0, self.time)));

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
    let (tx, rx) = channel();

    let _socket = WebSocketWeb::start("", tx);

    let mut window = Window::new("kiss3d");
    window.set_background_color(0.6, 0.4, 1.0);
    window.set_light(Light::StickToCamera);
    window.set_title("lalalalal");

    let size_step = 1.0;
    let count_step = 11;
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

    let mut nodes: Vec<NodeSc> = Vec::new();

    let state = AppState {
        nodes,
        time,
        vec,
        rx,
    };

    window.render_loop(state)
}
