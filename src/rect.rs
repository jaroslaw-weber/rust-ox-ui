use text::Text;
use vector::Vector2;
use color::Color;
use image::Image;

//to json
#[derive(Debug)]
pub struct Rectangle {
    children: Vec<Rectangle>,
    text: Option<Text>,
    image: Option<Image>,
    color: Option<Color>,
    global_position: Vector2,
    size: Vector2,
}

impl Rectangle {
    pub fn new() -> Rectangle {
        Rectangle {
            children: Vec::new(),
            text: None,
            color: None,
            image: None,
            global_position: Vector2::zero(),
            size: Vector2::new(100, 100),
        }
    }
    pub fn to_json(&self) -> String {
        String::new()
    } //todo Json object
    pub fn add_child(&mut self, child: Rectangle) {
        self.children.push(child);
    }
    pub fn set_size(&mut self, w: i32, h: i32) {
        self.size = Vector2::new(w, h)
    }
    pub fn set_global_position(&mut self, x: i32, y: i32) {
        self.global_position = Vector2::new(x, y);
    }
    pub fn set_text(&mut self, text_cmp: Text) {
        self.text = Some(text_cmp);
    }
    pub fn get_text(&self) -> &Option<Text> {
        &self.text
    }
    pub fn get_size(&self) -> Vector2 {
        self.size
    }
    pub fn get_name(&self) -> String {
        "app name".to_string()
    }
    pub fn get_global_position(&self) -> Vector2 {
        self.global_position
    }
    pub fn get_color_fill(&self) -> Option<Color> {
        self.color
    }
    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }
    pub fn get_children(&self) -> &Vec<Rectangle> {
        &self.children
    }
    pub fn set_image(&mut self, image: Image) {
        self.image = Some(image);
    }
    pub fn get_image(&self) -> &Option<Image> {
        &self.image
    }
}
