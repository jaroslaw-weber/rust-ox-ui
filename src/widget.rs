use rect::Rectangle;

//widget. trait to build application/widgets
pub trait Widget {
    fn build(&self) -> Rectangle;

    fn on_button_click(&mut self, button_id: i32);
}
