use std::collections::HashMap;
use piston_window::*;
use widget::Widget;
use vector::Vector2;
use rect::Rectangle;
use std::path::Path;

pub fn initialize(widget: &mut Widget) {

    //unroll widget
    let mut builded: Rectangle = widget.build();

    let (size_x, size_y): (f32, f32) = widget.get_window_size();
    let name: String = widget.get_window_name();
    let mut window: PistonWindow =
        WindowSettings::new(name, (size_x as u32, size_y as u32))
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

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
        //cache font glyphs in hashmap
        //todo access by name not by layer id? benchmark to test which is better
        let mut glyphs_hashmap = HashMap::new();

        for (i, layer_global_info) in unrolled.get_layers().iter().enumerate() {
            let &(layer, _) = layer_global_info;
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
            let txt_option = layer.get_text();
            match txt_option {
                &Some(ref txt) => {
                    let font_path = txt.get_font_path();
                    let mut font = load_font(&font_path, &window);
                    glyphs_hashmap.insert(i, font);

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
                    for layer_info in unrolled.get_layers().iter().rev() {
                        let &(layer, global_position) = layer_info;
                        let button_option = layer.get_button();
                        match button_option {
                            &Some(ref btn) => {
                                let btn_id = btn.get_id();
                                //todo fix for local position
                                //todo
                                let mpx = mpx as f32;
                                let mpy = mpy as f32;
                                let gpx = global_position.get_x();
                                let gpy = global_position.get_y();
                                let size = layer.get_size();
                                let w = size.get_x();
                                let h = size.get_y();
                                let clicked_this = mpx > gpx && mpx < gpx + w && mpy > gpy &&
                                                   gpy < gpy + h;
                                if clicked_this {
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
                    //drawing
                    window.draw_2d(&e, |_c, g| for (i, layer_info) in unrolled
                            .get_layers()
                            .iter()
                            .enumerate() {
                        //todo performance
                        let &(layer, global_position) = layer_info;
                        //parameters
                        let p = global_position;
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
                                let layer_key = i as usize;
                                let mut glyphs_now: &mut Glyphs =
                                    glyphs_hashmap.get_mut(&layer_key).unwrap();
                                let txt_content: String = txt.get_content();
                                //todo text color
                                let (_r, _g, _b, _a) = txt.get_color().to_tuple();
                                //println!("{:?}", tr);
                                text::Text::new_color([_r, _g, _b, _a], txt.get_font_size())
                                    .draw(&txt_content, glyphs_now, &_c.draw_state, tr, g);

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

fn load_font(path: &str, window: &PistonWindow) -> Glyphs {
    let factory = window.factory.clone();
    let glyphs = Glyphs::new(path, factory).unwrap();
    glyphs
}

//flat list of rectangles in app. used for drawing order
//also info about global position
struct UnrolledWidget<'a> {
    content: Vec<(&'a Rectangle, Vector2)>, //rectangle and its global position
}

impl<'a> UnrolledWidget<'a> {
    //unroll widget into a list of rectangles
    fn unroll_widget(rt: &Rectangle) -> UnrolledWidget {
        let mut result = UnrolledWidget { content: Vec::new() };
        let unrolled = result.unroll(rt, Vector2::zero(), rt.get_size());
        result.content = unrolled;
        result
    }

    //recursive unrolling
    fn unroll(&self,
              rt: &'a Rectangle,
              parent_global_position: Vector2,
              parent_size: Vector2)
              -> Vec<(&'a Rectangle, Vector2)> {
        let mut v = Vec::new();
        let pivot = rt.get_pivot();
        let size = rt.get_size();
        let local_position = rt.get_local_position();
        //unity3d-like anchor calculation
        let mut global_x = parent_global_position.get_x() + local_position.get_x();
        global_x += pivot.get_x() * (parent_size.get_x() - size.get_x());
        let reverse_y_pivot = 1. - pivot.get_y();
        let mut global_y = parent_global_position.get_y() + local_position.get_y();
        global_y += reverse_y_pivot * (parent_size.get_y() - size.get_y());

        let global_position = Vector2::new(global_x, global_y);

        v.push((rt, global_position));
        for ch in rt.get_children() {
            let unrolled: Vec<(&Rectangle, Vector2)> = self.unroll(ch, global_position, size);
            for ur in unrolled {
                v.push(ur);
            }
        }
        v
    }

    //get unrolled rectangles
    fn get_layers(&self) -> &Vec<(&'a Rectangle, Vector2)> {
        &self.content
    }
}
