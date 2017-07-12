use text::Text;
use vector::Vector2;
use color::Color;
use image::Image;
use button::Button;

//to json
//rectangle containing different components. holds position and size info
#[derive(Debug)]
pub struct Rectangle {
    children: Vec<Rectangle>,
    text: Option<Text>,
    image: Option<Image>,
    color: Option<Color>,
    button: Option<Button>,
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
            button: None,
            global_position: Vector2::zero(),
            size: Vector2::new(100, 100),
        }
    }
    //todo to json

    //set child rectangle. need to manage hierarchy
    pub fn add_child(&mut self, child: Rectangle) {
        self.children.push(child);
    }

    //set rectangle size
    pub fn set_size(&mut self, w: i32, h: i32) {
        self.size = Vector2::new(w, h)
    }

    //global position. todo: change to local position
    pub fn set_global_position(&mut self, x: i32, y: i32) {
        self.global_position = Vector2::new(x, y);
    }

    //set text component
    pub fn set_text(&mut self, text_cmp: Text) {
        self.text = Some(text_cmp);
    }

    //get text component
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

    //get all rectangle children
    pub fn get_children(&self) -> &Vec<Rectangle> {
        &self.children
    }
    pub fn set_image(&mut self, image: Image) {
        self.image = Some(image);
    }
    pub fn get_image(&self) -> &Option<Image> {
        &self.image
    }

    //is point inside the rectangle? using for button raycasting check
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        let gp = self.global_position;
        let size = self.size;
        let contains: bool = x >= gp.get_x() && x <= gp.get_x() + size.get_x() && y >= gp.get_y() &&
                             y <= gp.get_y() + size.get_y();
        contains

    }

    pub fn get_button(&self) -> &Option<Button> {
        &self.button
    }
    pub fn set_button(&mut self, btn: Button) {
        self.button = Some(btn);
    }
}
