mod websocket;

pub struct Data {
    arr: Vec<u8>, //quaternion: [f32;4];
                  //position_in_space: [f32;3];
                  //delta_position_in_space: [f32;3];
}

impl Data {
    pub fn new(arr_: Vec<u8>) -> Self {
        // ... arr
        Data { arr: arr_ }
    }
    // ...
}

pub struct Connect {
    type_connect: &'static str,
    ip_port: &'static str,
    pub get_bytes: fn() -> Vec<u8>,
}

impl Connect {
    pub fn new(type_connect: &'static str, ip_port: &'static str) -> Self {
        let get: fn() -> Vec<u8> = init_connect(&type_connect, &ip_port);
        Connect {
            type_connect: type_connect,
            ip_port: ip_port,
            get_bytes: get,
        }
    }
    pub fn get_data(&self) -> Data {
        return Data::new((self.get_bytes)());
    }
}

fn init_connect(type_connect: &str, ip_port: &str) -> fn() -> Vec<u8> {
    return match type_connect {
        "WebSocket_Web" => websocket::init(ip_port),
        _ => panic!("Error type connect"),
    };
}
