//image loading
#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    path: String,
}
impl Image {
    pub fn new(path: &str) -> Image {
        Image { path: path.to_string() }
    }
    pub fn get_path(&self) -> String {
        (&self.path).to_string()
    }
}