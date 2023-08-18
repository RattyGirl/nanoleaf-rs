use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::{json, Value};
use std::fmt::{format, Display, Formatter};

#[derive(Debug)]
pub struct Nanoleaf {
    access_token: Option<String>,
    ip: Option<String>,
    on: Option<bool>,
}

impl Nanoleaf {
    pub fn new() -> Self {
        Self {
            access_token: None,
            ip: None,
            on: None,
        }
    }
    pub fn preload(ip: &str, access_token: &str) -> Self {
        Self {
            access_token: Some(access_token.to_string()),
            ip: Some(ip.to_string()),
            on: None,
        }
    }

    pub fn get_call(&self, target: &str) -> Value {
        let resp = reqwest::blocking::get(format!(
            "http://{}:16021/api/v1/{}{}",
            self.ip.clone().unwrap(),
            self.access_token.clone().unwrap(),
            target
        ))
        .unwrap()
        .text()
        .unwrap();
        serde_json::from_str(resp.as_str()).unwrap()
    }

    pub fn put_call(&self, target: &str, data: &Value) {
        let client = reqwest::blocking::Client::new();
        let requestBuilder = client
            .put(format!(
                "http://{}:16021/api/v1/{}{}",
                self.ip.clone().unwrap(),
                self.access_token.clone().unwrap(),
                target
            ))
            .body(data.to_string());

        println!("{:?}", requestBuilder);

        let response = requestBuilder.send().unwrap();

        println!("{:?}", response);
    }

    pub fn get_info(&mut self) {
        let json = self.get_call("/");
        println!("{:?}", json);
    }

    pub fn get_state(&mut self) {
        let json = self.get_call("/state/on");
        self.on = match json["value"] {
            Value::Bool(x) => Some(x),
            _ => None,
        };
    }

    pub fn get_layout(&mut self) -> Layout {
        let json = self.get_call("/panelLayout/layout");
        let layout = serde_json::from_value(json).unwrap();
        layout
    }

    pub fn set_state(&mut self, state: bool) {
        let json = json!({
            "on": {
                "value": state
            }
        });

        self.put_call("/state/on", &json);
    }

    pub fn open_socket(&mut self) {
        let json = json!({
            "write": {
                "command": "display",
                "animType": "extControl",
                "extControlVersion": "v2"
            }
        });

        self.put_call("/effects", &json);
    }
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct State {
    on: ValueInner,
    brightness: NumericalValue,
    hue: NumericalValue,
    sat: NumericalValue,
    ct: NumericalValue,
    color_mode: String,
}
#[derive(Serialize, Deserialize)]
struct ValueInner {
    value: bool,
}
#[derive(Serialize, Deserialize)]
struct NumericalValue {
    value: i64,
    max: i64,
    min: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Effects {
    select: String,
    effects_list: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PanelLayout {
    layout: Layout,
    global_orientation: NumericalValue,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Layout {
    num_panels: i32,
    side_length: i32,
    pub position_data: Vec<Position>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub panel_id: u16,
    x: i32,
    y: i32,
    o: i32,
    pub shape_type: u32
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LightControllerInfo {
    name: String,
    serial_no: String,
    manufacturer: String,
    firmware_version: String,
    model: String,
    state: State,
    effects: Effects,
    panel_layout: PanelLayout,
    rhythm: Rhythm,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Rhythm {
    rhythm_connected: bool,
    rhythm_active: Option<bool>,
    rhythm_id: Option<String>,
    hardware_version: Option<String>,
    firmware_version: Option<String>,
    aux_available: Option<bool>,
    rhythm_mode: Option<String>,
    rhythm_pos: Option<bool>,
}

pub struct Frame {
    pub states: Vec<PanelState>
}

impl Frame {
    pub fn get_buf(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::with_capacity(72);
        out.push(0);
        out.push(self.states.len() as u8);

        for panel in &self.states {
            let panel_id = panel.panel_id.to_be_bytes();
            out.push(panel_id[0]);
            out.push(panel_id[1]);
            out.push(panel.red);
            out.push(panel.green);
            out.push(panel.blue);
            let white = 255u8;
            out.push(white);
            let transition_time = panel.transition_time.to_be_bytes();
            out.push(transition_time[0]);
            out.push(transition_time[1]);
        }

        out
    }

    pub fn solid_frame(&mut self, panel_id: u16, red: u8, green: u8, blue: u8) {
        self.states.push(PanelState {
            panel_id,
            red,
            green,
            blue,
            transition_time: 1
        });
    }
}

pub struct PanelState {
    panel_id: u16,
    red: u8,
    green: u8,
    blue: u8,
    transition_time: u16
}
