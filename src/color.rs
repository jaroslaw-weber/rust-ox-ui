//color as struct, also contains helper methods
#[derive(Copy,Clone, Debug)]
pub struct Color {
    color: (f32, f32, f32, f32),
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { color: (r, g, b, a) }
    }

    pub fn to_arr(&self) -> [f32; 4] {
        let (r, g, b, a) = self.color;
        [r, g, b, a]
    }

    pub fn to_tuple(&self) -> (f32, f32, f32, f32) {
        self.color
    }

    pub fn black() -> Color {
        Color::new(0., 0., 0., 1.)
    }
    pub fn white() -> Color {
        Color::new(1., 1., 1., 1.)
    }
}