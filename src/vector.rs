//pair of numbers
#[derive(Copy,Clone, Debug)]
pub struct Vector2 {
    x: i32,
    y: i32,
}

impl Vector2 {
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn new(x: i32, y: i32) -> Vector2 {
        Vector2 { x: x, y: y }
    }

    pub fn zero() -> Vector2 {
        Vector2::new(0, 0)
    }
}
