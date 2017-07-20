extern crate oxui;
extern crate serde_json;
use oxui::engine;
use oxui::rect::{Rectangle, AlignType};
use oxui::text::Text;
use oxui::widget::Widget;
use oxui::color::Color;
use oxui::image::Image;
use oxui::button::Button;
use oxui::vector::Vector2;

//todo macros

pub fn main() {
    run();
}

//run example app
pub fn run() {
    let mut example_app = ExampleApp {
        page_now: AppPage::Main,
        size: Vector2::new(500., 800.),
    };
    //oxui::serialization::save_rectangle_to_json_file(&example_app.build(), "assets/serialized_main_page.txt");
    engine::initialize(&mut example_app);
}

//which page is currently opened
enum AppPage {
    Main,
    AfterClick,
}

//example app with "state" in field
struct ExampleApp {
    page_now: AppPage,
    size: Vector2,
}

//implementation of widget
impl Widget for ExampleApp {
    //building app with new state
    fn build(&self) -> Rectangle {
        match self.page_now {
            AppPage::Main => self.get_main_page(),
            AppPage::AfterClick => self.get_second_page(),
        }
    }

    //on button click listeners
    fn on_button_click(&mut self, button_id: i32) {
        match button_id {
            1 => {
                self.page_now = AppPage::AfterClick;
                println!("button clicked!");
            }
            _ => println!("Button listener not registered!"),
        }
    }

    fn get_window_size(&self) -> (f32, f32) {
        (500., 800.)
    }

    fn get_window_name(&self) -> String {
        "Rust GUI APP".to_string()
    }
}

fn get_font_path()->String{
    "assets/JosefinSans-Regular.ttf".to_string()
}

//application layout definition
impl ExampleApp {
    //page no2
    fn get_second_page(&self) -> Rectangle {
        let size = self.size;
        let (w, h) = (size.get_x(), size.get_y());
        let rusty_color = Color::rusty();
        let mut root = Rectangle::new();
        root.set_size(w, h);
        root.set_color(rusty_color);
        let mut txt = Text::new("Clicked!", &get_font_path());
        txt.set_color(Color::white());
        let mut txt_rt = Rectangle::new();
        txt_rt.set_local_position(-70., 50.);
        txt_rt.set_text(txt);
        root.add_child(txt_rt);
        root

    }

    //page no1
    fn get_main_page(&self) -> Rectangle {
        let (w, h) = (500., 800.);
        let mut logo_rt = self.get_logo();
        logo_rt.add_child(self.get_text_under_logo());

        let btn_rt = self.get_button();
        logo_rt.add_child(btn_rt);
        //root
        let mut root = Rectangle::new();
        root.set_size(w, h);
        root.add_child(self.get_header());
        root.add_child(logo_rt);
        root.add_child(self.get_footer());
        let bg = Color::white();
        root.set_color(bg);
        root
    }

    fn get_logo(&self) -> Rectangle {

        let logo_img = Image::new("assets/logo.png");
        let mut rust_logo_rt = Rectangle::new();
        let (logo_w, logo_h) = (300., 280.);
        rust_logo_rt.set_local_position(0., -60.);
        rust_logo_rt.set_align_type(AlignType::Center);
        rust_logo_rt.set_size(logo_w, logo_h);
        rust_logo_rt.set_image(logo_img);
        rust_logo_rt
    }

    fn get_footer(&self) -> Rectangle {

        let mut footer_rt = Rectangle::new();
        footer_rt.set_color(Color::rusty());
        footer_rt.set_local_position(0., 0.);
        footer_rt.set_size(self.size.get_x(), 100.);
        footer_rt.set_align_type(AlignType::Bottom);

        let mut footer_txt = Text::new("Footer!", &get_font_path());
        footer_txt.set_color(Color::white());

        let mut footer_text_rt = Rectangle::new();
        footer_text_rt.set_local_position(180., 60.);
        footer_text_rt.set_text(footer_txt);
        footer_text_rt.set_align_type(AlignType::TopLeft);
        footer_rt.add_child(footer_text_rt);
        footer_rt
    }

    fn get_text_under_logo(&self) -> Rectangle {
        let mut some_text = Text::new("Lets rewrite GUI apps in rust!", &get_font_path());
        some_text.set_font_size(20);
        let mut some_text_rt = Rectangle::new();
        some_text_rt.set_local_position(-20., 350.);
        some_text_rt.set_align_type(AlignType::TopLeft);
        some_text_rt.set_text(some_text);
        some_text_rt
    }

    //header
    fn get_header(&self) -> Rectangle {

        let w = self.size.get_x();
        let mut top_bar = Rectangle::new();
        let bgc = Color::rusty();
        top_bar.set_local_position(0., 0.);
        top_bar.set_align_type(AlignType::Top);
        top_bar.set_size(w, 150.);
        top_bar.set_color(bgc);
        //txt
        let mut title_text = Text::new("Simple Rust GUI!", &get_font_path());
        title_text.set_color(Color::white());
        let mut txt_rt = Rectangle::new();
        txt_rt.set_size(w, 150.);
        txt_rt.set_local_position(100., 100.);
        txt_rt.set_align_type(AlignType::TopLeft);
        txt_rt.set_text(title_text);
        top_bar.add_child(txt_rt);
        top_bar
    }

    //main page btn
    fn get_button(&self) -> Rectangle {
        let btn_cmp = Button::new(1);
        let mut btn_text = Text::new("Click here!", &get_font_path());
        btn_text.set_color(Color::white());
        let mut btn_text_rt = Rectangle::new();
        btn_text_rt.set_text(btn_text);
        let mut btn_rt = Rectangle::new();
        btn_rt.set_size(300., 100.);
        btn_rt.set_align_type(AlignType::Top);
        btn_rt.set_local_position(0., 380.);
        btn_text_rt.set_local_position(60., 70.);
        btn_text_rt.set_align_type(AlignType::TopLeft);
        btn_rt.add_child(btn_text_rt);
        btn_rt.set_color(Color::rusty());
        btn_rt.set_button(btn_cmp);
        btn_rt
    }
}
