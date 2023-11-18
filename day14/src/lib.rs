use cave::{Cave, Cell};
use nom::{
    bytes::complete::tag, character::complete::u32, combinator::map, sequence::separated_pair,
    IResult,
};
use std::{cell::RefCell, fmt::Debug, rc::Rc};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use wasm_bindgen::{prelude::Closure, JsValue};
use web_sys::console;
use webgl::{IntVec2, Rectangle};
mod cave;
mod webgl;

struct Application {
    cave: Cave,
    webgl: webgl::Webgl,
}

fn cave_to_webgl_adapter(cave: Vec<Vec<Cell>>) -> Vec<Rectangle> {
    cave.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().fold(vec![], |mut a, (x, cell)| {
                let pos = IntVec2 {
                    x: x as i32,
                    y: y as i32,
                };
                let width = 1;
                let height = 1;
                match cell {
                    Cell::Rock => a.push(Rectangle {
                        pos,
                        width,
                        height,
                        color: [0.290, 0.188, 0.0551, 1.0],
                    }),
                    Cell::Sand => a.push(Rectangle {
                        pos,
                        width,
                        height,
                        color: [1.00, 0.887, 0.250, 1.0],
                    }),
                    _ => {}
                }
                return a;
            })
        })
        .collect()
}
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let cave = Cave::new();
    let cells = cave.get_printable_cells();
    console::log_1(&format!("{:?}", cave).into());
    let webgl = webgl::Webgl::new(canvas, cells[0].len() as i32, cells.len() as i32)?;

    let app = Rc::new(RefCell::new(Application {
        cave: cave,
        webgl: webgl,
    }));

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        if app.borrow().cave.is_done() {
            let _ = f.borrow_mut().take();
            return;
        }
        app.borrow_mut().cave.step();
        let cells = app.borrow_mut().cave.get_printable_cells();
        let rectangles = cave_to_webgl_adapter(cells);
        app.borrow_mut().webgl.clear();
        rectangles.iter().for_each(|r| {
            app.borrow_mut().webgl.draw_rectangle(r).unwrap();
        });
        let sand = app.borrow().cave.sand.clone();
        let min_x = app.borrow().cave.min_x;
        sand.iter().for_each(|s| {
            let r = Rectangle {
                pos: IntVec2 {
                    x: s.0 as i32 - min_x as i32,
                    y: s.1 as i32,
                },
                width: 1,
                height: 1,
                color: [1.0, 0.0, 0.0, 1.0],
            };
            app.borrow_mut().webgl.draw_rectangle(&r).unwrap();
        });
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));
    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
