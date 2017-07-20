
//pair of numbers
#[derive(Copy,Clone, Debug, Serialize, Deserialize)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x: x, y: y }
    }

    pub fn zero() -> Vector2 {
        Vector2::new(0.0, 0.0)
    }

    pub fn one()->Vector2
    {
        Vector2::new(1.,1.)
    }

    pub fn half()->Vector2
    {
        Vector2::new(0.5,0.5)
    }
}
