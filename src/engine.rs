use std::collections::HashMap;
use piston_window::*;
use find_folder;
use widget::Widget;
use vector::Vector2;
use rect::Rectangle;
use std::path::Path;

pub fn initialize(widget: &mut Widget) {

    //unroll widget
    let mut builded: Rectangle = widget.build();

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

    //redrawing loop
    loop {
        //rebuild on mouse click
        builded = widget.build();
        println!("rebuilding widget");
        //todo pass builded
        let unrolled = UnrolledWidget::unroll_widget(&builded);

        /////PERFORMANCE CACHING
        //cached textures in hashmap, loading only once
        let mut textures_hashmap = HashMap::new();

        for (i, layer) in unrolled.get_layers().iter().enumerate() {
            let img_option = layer.get_image();
            match img_option {
                &Some(ref img) => {
                    let path_str: String = img.get_path();
                    let path: &Path = Path::new(&path_str);
                    let txt_set: TextureSettings = TextureSettings::new();
                    let texture =
                        Texture::from_path(&mut image_factory, path, Flip::None, &txt_set).unwrap();
                    textures_hashmap.insert(i, texture);
                    println!("found texture in layer: {}", i);
                }
                &None => (),
            }
        }
        ////
        let mut mouse_pos: (f64, f64) = (0., 0.);
        while let Some(e) = window.next() {
            // println!("event: {:?}", e);
            match e {
                Input::Move(motion) => {
                    match motion {
                        //on cursor move
                        Motion::MouseCursor(mouse_x, mouse_y) => mouse_pos = (mouse_x, mouse_y),
                        _ => (),
                    }
                }
                Input::Press(_) => {
                    //on mouse click
                    let (mpx, mpy) = mouse_pos;
                    //todo reverse
                    let mut should_rebuild = false;
                    for layer in unrolled.get_layers().iter().rev() {
                        let button_option = layer.get_button();
                        match button_option {
                            &Some(ref btn) => {
                                let btn_id = btn.get_id();
                                let clicked_this = layer.contains_point(mpx as i32, mpy as i32);
                                if clicked_this {
                                    println!("clicked on rect: {:?}", layer.get_global_position());
                                    widget.on_button_click(btn_id);
                                    //break for reload widget from new state
                                    should_rebuild = true;
                                }
                            }
                            &None => {}
                        }
                    }
                    //should rebuild? if clicked mouse then yes
                    if should_rebuild {
                        println!("should rebuild!");
                        break;
                    }

                }
                Input::Render(_) => {
                    //todo performance

                    //drawing
                    window.draw_2d(&e, |_c, g| for (i, layer) in unrolled
                            .get_layers()
                            .iter()
                            .enumerate() {
                        //todo performance

                        //parameters
                        let p = layer.get_global_position();
                        let size = layer.get_size();
                        let x = p.get_x() as f64;
                        let y = p.get_y() as f64;
                        let w = size.get_x() as f64;
                        let h = size.get_y() as f64;
                        let tr = _c.transform.trans(x, y);

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

                                let txt_content: String = txt.get_content();
                                //todo text color
                                let (_r, _g, _b, _a) = txt.get_color().to_tuple();
                                //println!("{:?}", tr);
                                text::Text::new_color([_r, _g, _b, _a], txt.get_font_size())
                                    .draw(&txt_content, &mut glyphs, &_c.draw_state, tr, g);

                            }
                            &None => {}
                        }
                        //show texture if layer has image
                        let layer_key = i as usize;
                        match textures_hashmap.get(&layer_key) {
                            Some(texture) => {
                                let image = Image::new().rect([x, y, w, h]);
                                image.draw(texture, &_c.draw_state, _c.transform, g);
                            }
                            None => (),
                        }
                    });
                }
                _ => (),
            }
        }
    }
}

//flat list of rectangles in app. used for drawing order
struct UnrolledWidget<'a> {
    content: Vec<&'a Rectangle>,
}

impl<'a> UnrolledWidget<'a> {
    //unroll widget into a list of rectangles
    fn unroll_widget(rt: &Rectangle) -> UnrolledWidget {
        let mut result = UnrolledWidget { content: Vec::new() };
        let unrolled = result.unroll(rt);
        result.content = unrolled;
        result
    }

    //recursive unrolling
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

    //get unrolled rectangles
    fn get_layers(&self) -> &Vec<&'a Rectangle> {
        &self.content
    }
}