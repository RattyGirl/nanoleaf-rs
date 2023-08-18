use nanoleaf_rs::{Frame, Nanoleaf};
use std::fmt::Display;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use rand::{random, Rng};

const ACCESS_TOKEN: &str = env!("ACCESS_TOKEN");
const TEST_IP: &str = "10.0.0.3";

fn main() {
    let mut nanoleaf = Nanoleaf::preload(TEST_IP, ACCESS_TOKEN);
    let layout = nanoleaf.get_layout();

    nanoleaf.open_socket();
    let socket_addr = format!("{}:{}", TEST_IP, 60222);
    println!("{}", socket_addr);
    let socket = UdpSocket::bind("0.0.0.0:60222").unwrap();

    loop {
        let mut frame = Frame {
            states: Vec::new(),
        };
        for position_datum in &layout.position_data {
            if position_datum.shape_type == 7 {
                frame.solid_frame(position_datum.panel_id, 255, 0, 0);
            }
        }
        let binding = frame.get_buf();
        let buf = binding.as_slice();

        socket.send_to(&buf, &socket_addr).expect("TODO: panic message");
        thread::sleep(Duration::from_secs(1));
    }
}
