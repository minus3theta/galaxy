mod interpreter;
pub mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn initialize() -> Result<(), JsValue> {
    const CANVAS_ID: &str = "galaxy_canvas";
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(CANVAS_ID).unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let draw = || {
        let canvas_w = canvas.width() as usize;
        let canvas_h = canvas.height() as usize;

        let mut data = Vec::with_capacity(canvas_w * canvas_h);
        for r in 0..canvas_h {
            for c in 0..canvas_w {
                data.push((255 * r / canvas_h) as u8);
                data.push((255 * c / canvas_w) as u8);
                data.push(0);
                data.push(255);
            }
        }
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&data),
            canvas.width(),
            canvas.height(),
        )
        .unwrap();
        context.put_image_data(&data, 0.0, 0.0).ok();
    };

    draw();

    Ok(())
}

#[wasm_bindgen]
pub fn on_click(x: f64, y: f64) {
    let msg = format!("Clicked {}, {}", x, y);
    log(&msg);
}
