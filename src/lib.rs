mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, galaxy!");
}

#[wasm_bindgen]
pub fn draw() {
    draw_inner();
}

fn draw_inner() {
    const CANVAS_ID: &str = "galaxy_canvas";
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(CANVAS_ID).unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    context.fill_rect(1.0, 1.0, 100.0, 100.0);
}
