
use color::Color;
//todo font
//todo font alignment

//text component
#[derive(Debug, Serialize,Deserialize)]
pub struct Text {
    content: String,
    font_path: String,
    color: Color,
    font_size: u32,

}
impl Text {
    pub fn new(text: &str, font_path: &str) -> Text {
        Text {
            content: text.to_string(),
            font_path: font_path.to_string(),
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
    pub fn get_font_path(&self)->String
    {
        self.font_path.to_string()
    }
}
