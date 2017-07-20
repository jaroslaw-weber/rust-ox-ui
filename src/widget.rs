use rect::Rectangle;

//widget. trait to build application/widgets
pub trait Widget {
    fn build(&self) -> Rectangle;

    fn on_button_click(&mut self, button_id: i32);

    fn get_window_size(&self)->(f32,f32);

    fn get_window_name(&self)->String;
}
