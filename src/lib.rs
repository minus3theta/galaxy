mod interpreter;
pub mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, EventTarget, HtmlCanvasElement, ImageData, MouseEvent};

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
    let bounding_rect = canvas.get_bounding_client_rect();
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

    {
        let click_handler = Closure::wrap(Box::new(move |event: MouseEvent| {
            let x = (event.client_x() as f64 - bounding_rect.left()) / bounding_rect.width();
            let y = (event.client_y() as f64 - bounding_rect.top()) / bounding_rect.height();
            log(&format!("Clicked {}, {}", x, y));
        }) as Box<dyn FnMut(_)>);
        let canvas_event_target: EventTarget = canvas.clone().dyn_into::<EventTarget>().unwrap();
        canvas_event_target
            .add_event_listener_with_callback("click", click_handler.as_ref().unchecked_ref())?;
        click_handler.forget();
    }
    draw();

    Ok(())
}
