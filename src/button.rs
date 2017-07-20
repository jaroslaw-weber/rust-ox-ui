
#[derive(Debug, Serialize, Deserialize)]
pub struct Button {
    id: i32,
}
//button component, have id (usually unique) for listeners
impl Button {
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn new(id: i32) -> Button {
        Button { id: id }
    }
}
