use rect::Rectangle;

pub trait Widget {
    fn build(&self) -> Rectangle;
}
