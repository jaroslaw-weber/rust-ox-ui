
use piston_window::*;
use find_folder;
use widget::Widget;
use vector::Vector2;
use rect::Rectangle;
use text::Text;
use std::path::Path;

pub fn draw(widget: &Widget) {

    //unroll widget
    let builded: Rectangle = widget.build();

    let size: Vector2 = builded.get_size();
    let name: String = builded.get_name();
    let mut window: PistonWindow =
        WindowSettings::new(name, (size.get_x() as u32, size.get_y() as u32))
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    //font load
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let ref font = assets.join("JosefinSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();
    //font load end
    let mut image_factory = window.factory.clone();

    //todo pass builded
    let unrolled = UnrolledWidget::unroll_widget(&builded);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g| for layer in unrolled.get_layers() {
            //parameters
            let p = layer.get_global_position();
            let size = layer.get_size();
            let x = p.get_x() as f64;
            let y = p.get_y() as f64;
            let w = size.get_x() as f64;
            let h = size.get_y() as f64;
            let tr = _c.transform.trans(w, h);
            //render color first
            match layer.get_color_fill() {
                Some(color) => {
                    let slice = color.to_arr();
                    rectangle(slice, [x, y, w, h], _c.transform, g);
                }
                None => {}
            }
            //render text
            match layer.get_text() {
                &Some(ref txt) => {
                    let txtContent: String = txt.get_content();
                    //todo text color
                    let (_r, _g, _b, _a) = txt.get_color().to_tuple();
                    //println!("{:?}", tr);
                    text::Text::new_color([_r, _g, _b, _a], txt.get_font_size())
                        .draw(&txtContent, &mut glyphs, &_c.draw_state, tr, g);
                }
                &None => {}
            }

            //render image
            match layer.get_image() {
                &Some(ref img) => {

                    //let sq = rectangle::square(0.0, 0.0, 300.0);
                    //let image=Image::new();
                    let image = Image::new().rect([x, y, w, h]);
                    let path_str: String = img.get_path();
                    let path: &Path = Path::new(&path_str);
                    let txt_set: TextureSettings = TextureSettings::new();
                    // let state=draw_state::state::
                    let texture =
                        Texture::from_path(&mut image_factory, path, Flip::None, &txt_set).unwrap();
                    //println!("texture loaded: {:?}", texture);
                    image.draw(&texture, &_c.draw_state, _c.transform, g);
                }
                &None => {}
            }

        });
    }
}
fn from_slice(bytes: &[u8]) -> [u8; 4] {
    let mut a = [0; 4];
    for i in 0..a.len() {
        // Panics if not enough input
        a[i] = bytes[i];
    }
    a
}

struct UnrolledWidget<'a> {
    content: Vec<&'a Rectangle>,
}

impl<'a> UnrolledWidget<'a> {
    fn unroll_widget(rt: &Rectangle) -> UnrolledWidget {
        let mut rects = Vec::new();
        let mut result = UnrolledWidget { content: rects };
        let unrolled = result.unroll(rt);
        result.content = unrolled;
        result


    }

    fn unroll(&self, rt: &'a Rectangle) -> Vec<&'a Rectangle> {
        let mut v = Vec::new();
        //println!("found rt!: {:?}", rt);
        v.push(rt);
        for ch in rt.get_children() {
            let unrolled: Vec<&Rectangle> = self.unroll(ch);
            for ur in unrolled {
                v.push(ur);
            }
        }
        v
    }



    fn get_size(&self) -> Vector2 {
        self.content.get(0).unwrap().get_size()
    }
    fn get_layers(&self) -> &Vec<&'a Rectangle> {
        &self.content
    }
}