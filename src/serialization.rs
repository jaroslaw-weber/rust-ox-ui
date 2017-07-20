use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use serde_json;
use rect::Rectangle;

pub fn load_rectangle_from_file(path: &str) -> Rectangle {

    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let as_rectangle: Rectangle = serde_json::from_str(&contents).unwrap();
    as_rectangle
}

pub fn save_rectangle_to_json_file(rt: &Rectangle, path: &str) {
    let as_json = serde_json::to_string(rt).unwrap();
    let mut buffer = File::create(path).unwrap();
    buffer.write_all(&as_json.as_bytes()).unwrap();
}