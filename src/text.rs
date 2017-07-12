use color::Color;

//todo font

//text component
#[derive(Debug)]
pub struct Text {
    content: String,
    color: Color,
    font_size: u32,
}
impl Text {
    pub fn new(text: &str) -> Text {
        Text {
            content: text.to_string(),
            color: Color::black(),
            font_size: 32,
        }
    }
    pub fn get_content(&self) -> String {
        self.content.to_string()
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    pub fn get_color(&self) -> Color {
        self.color
    }
    pub fn get_font_size(&self) -> u32 {
        self.font_size
    }
    pub fn set_font_size(&mut self, font_size: u32) {
        self.font_size = font_size;
    }
}