use engine;
use rect::Rectangle;
use text::Text;
use widget::Widget;
use color::Color;
use image::Image;
use button::Button;

//run example app
pub fn run() {
    let mut example_app = ExampleApp { page_now: AppPage::Main };
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
}

//implementation of widget
impl Widget for ExampleApp {
    
    //building app with new state
    fn build(&self) -> Rectangle {
        match self.page_now {
            AppPage::Main => self.build_main_page(),
            AppPage::AfterClick => self.build_after_click_page(),
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
}

//application layout definition
impl ExampleApp {

    //page no2
    fn build_after_click_page(&self) -> Rectangle {

        let (w, h) = (500, 800);
        let rusty_color = Color::new(0.64, 0.32, 0.18, 1.0);
        let mut root = Rectangle::new();
        root.set_size(w, h);
        root.set_color(rusty_color);
        let mut txt = Text::new("Clicked!");
        txt.set_color(Color::white());
        let mut txt_rt = Rectangle::new();
        txt_rt.set_global_position(150, 350);
        txt_rt.set_text(txt);

        root.add_child(txt_rt);
        root

    }

    //page no1
    fn build_main_page(&self) -> Rectangle {
        //rusty color #d8592b
        let rusty_color = Color::new(0.64, 0.32, 0.18, 1.0);
        let (w, h) = (500, 800);

        let top_bar = self.create_top_bar(w, rusty_color);

        //second rectangle
        let logo_img = Image::new("assets/logo.png");
        let mut rust_logo_rt = Rectangle::new();
        let (logo_w, logo_h) = (300, 280);
        rust_logo_rt.set_global_position((w - logo_w) / 2, 250);
        rust_logo_rt.set_size(logo_w, logo_h);
        rust_logo_rt.set_image(logo_img);

        //Some text
        let mut some_text = Text::new("Lets rewrite GUI apps in rust!");
        //some_text.set_color(white_c);
        some_text.set_font_size(20);
        let mut some_text_rt = Rectangle::new();
        some_text_rt.set_global_position(95, 620);
        //some_text_rt.set_size(w, h);
        some_text_rt.set_text(some_text);

        let btn_rt = self.create_button_on_main_page(rusty_color);
        //root
        let mut root = Rectangle::new();
        root.set_size(w, h);
        root.add_child(some_text_rt);
        root.add_child(top_bar);
        root.add_child(rust_logo_rt);
        root.add_child(btn_rt);
        let bg = Color::white();
        root.set_color(bg);
        root
    }

    //header
    fn create_top_bar(&self, width: i32, rusty_color: Color) -> Rectangle {

        //one rectangle
        let mut top_bar = Rectangle::new();
        let bgc = rusty_color;
        top_bar.set_global_position(0, 0);
        top_bar.set_size(width, 150);
        top_bar.set_color(bgc);
        //txt
        let mut title_text = Text::new("Simple Rust GUI!");
        let white_c = Color::white();
        title_text.set_color(white_c);
        let mut txt_rt = Rectangle::new();
        txt_rt.set_global_position(100, 100);
        txt_rt.set_text(title_text);
        top_bar.add_child(txt_rt);
        top_bar
    }

    //main page btn
    fn create_button_on_main_page(&self, rusty_color: Color) -> Rectangle {
        let btn_cmp = Button::new(1);
        let mut btn_text = Text::new("Button");
        btn_text.set_color(Color::white());
        let mut btn_text_rt = Rectangle::new();
        btn_text_rt.set_text(btn_text);
        let mut btn_rt = Rectangle::new();
        btn_rt.set_size(400, 70);
        btn_rt.set_global_position(60, 650);
        btn_text_rt.set_global_position(205, 700);
        btn_rt.add_child(btn_text_rt);
        btn_rt.set_color(rusty_color);
        btn_rt.set_button(btn_cmp);
        btn_rt
    }
}
