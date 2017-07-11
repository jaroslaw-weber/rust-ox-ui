
use piston_window::*;
use find_folder;

pub fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", (640, 480))
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let ref font = assets.join("JosefinSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();



    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g| {
            let transform = _c.transform.trans(10.0, 100.0);
            clear([0.5, 1.0, 0.5, 1.0], g);
            let center = _c.transform.trans(300.0, 300.0);
            let square = rectangle::square(0.0, 0.0, 100.0);
            let red = [1.0, 0.0, 0.0, 1.0];
            rectangle(red, square, center.trans(-50.0, -50.0), g);
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                .draw("some text", &mut glyphs, &_c.draw_state, transform, g);
        });
    }
}