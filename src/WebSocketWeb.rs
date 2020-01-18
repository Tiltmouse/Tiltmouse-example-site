extern crate stdweb;

use std::sync::mpsc::Sender;
use stdweb::traits::{IEventTarget, IMessageEvent};
use stdweb::web::event::SocketMessageEvent;
use stdweb::web::WebSocket;

pub struct WebSocketWeb {}

impl WebSocketWeb {
    pub fn init(ip_port: &str, tx: &'static Sender<String>) {
        let mut socket = WebSocket::new(&("ws://".to_owned() + &ip_port)).unwrap();

        socket.add_event_listener(move |e: SocketMessageEvent| {
            let data = e.data().into_text().unwrap();
            stdweb::console!(log, data.clone());
            tx.send(data).unwrap();
        });
    }
}
