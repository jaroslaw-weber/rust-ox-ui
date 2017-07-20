extern crate oxui;
use oxui::rect::Rectangle;
use oxui::widget::Widget;
use oxui::engine;
use oxui::serialization;

struct JsonWidget {}

//implementation of widget
impl Widget for JsonWidget {
    fn build(&self) -> Rectangle {
        serialization::load_rectangle_from_file("assets/serialized_main_page.txt")
    }

    //on button click listeners
    fn on_button_click(&mut self, button_id: i32) {}

    fn get_window_size(&self) -> (f32, f32) {
        (500., 800.)
    }

    fn get_window_name(&self) -> String {
        "Loaded from json".to_string()
    }
}

fn main() {
    let mut w = JsonWidget {};
    engine::initialize(&mut w);
}