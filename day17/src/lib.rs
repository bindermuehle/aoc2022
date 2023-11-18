use std::{cell::RefCell, rc::Rc};

use cave::TileType;
use wasm_bindgen::{prelude::wasm_bindgen, prelude::Closure, JsCast, JsValue};
use web_sys::console;

mod cave;

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
    let mut cave = cave::Cave::new();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        if cave.is_done() {
            let _ = f.borrow_mut().take();
            return;
        }
        cave.step();
        canvas.set_height(cave.height * 10);
        canvas.set_width(cave.width * 10);
        let cells = cave.get_cells();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        context.set_fill_style(&JsValue::from_str("orange"));
        context.fill_rect(((0) * 10) as _, ((0) * 10) as _, 10.0, 10.0);
        (0..cave.height).for_each(|y| {
            (0..cave.width).for_each(|x| {
                let tile = &cells[(x + y * cave.width) as usize];
                let color = match tile {
                    TileType::Block => "#33302d",
                    TileType::Empty => "#4db4e3",
                };
                context.set_fill_style(&JsValue::from_str(color));
                context.fill_rect((x * 10) as _, ((cave.height - y - 1) * 10) as _, 10.0, 10.0);
            })
        });

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));
    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
