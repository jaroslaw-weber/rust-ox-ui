use render;
use rect::Rectangle;
use text::Text;
use widget::Widget;
use color::Color;
use image::Image;

pub fn run() {
    let exampleApp = ExampleApp {};
    render::draw(&exampleApp);
}


struct ExampleApp {}

impl Widget for ExampleApp {
    //build app in reverse order (from small components to bigger)
    fn build(&self) -> Rectangle {
        //rusty color #d8592b
        let rusty_color = Color::new(0.64, 0.32, 0.18, 1.0);
        let (w, h) = (500, 800);

        //one rectangle
        let mut top_bar = Rectangle::new();
        let bgc = rusty_color;
        top_bar.set_global_position(0, 0);
        top_bar.set_size(w, 150);
        top_bar.set_color(bgc);
        //txt
        let mut title_text = Text::new("Simple Rust GUI!");
        let white_c = Color::white();
        title_text.set_color(white_c);
        let mut txt_rt = Rectangle::new();
        txt_rt.set_global_position(40, 100);
        txt_rt.set_text(title_text);
        top_bar.add_child(txt_rt);
        //second rectangle
        let logo_img = Image::new("assets/logo.png");
        let mut rust_logo_rt = Rectangle::new();
        let (logo_w, logo_h) = (300, 300);
        rust_logo_rt.set_global_position((w - logo_w) / 2, 250);
        rust_logo_rt.set_size(logo_w, logo_h);
        rust_logo_rt.set_image(logo_img);
        /*
        //Some text
        let mut some_text = Text::new("Lets rewrite GUI apps in rust!");
        some_text.set_color(white_c);
        some_text.set_font_size(20);
        let mut some_text_rt = Rectangle::new();
        some_text_rt.set_global_position(70, 600);
        //some_text_rt.set_size(w, h);
        some_text_rt.set_text(some_text);
        top_bar.add_child(some_text_rt);
        */
        //root
        let mut root = Rectangle::new();
        root.set_size(w, h);
        root.add_child(top_bar);
        root.add_child(rust_logo_rt);
        let bg = white_c;
        root.set_color(bg);
        root
    }
}

pub fn run2() {

    let exampleApp = ExampleApp2 {};
    render::draw(&exampleApp);
}

struct ExampleApp2 {}

impl Widget for ExampleApp2 {
    //build app in reverse order (from small components to bigger)
    fn build(&self) -> Rectangle {
        //one rectangle
        let mut root = Rectangle::new();
        root.set_size(500, 500);
        let c = Color::black();
        root.set_color(c);
        root
    }
}
