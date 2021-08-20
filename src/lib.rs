mod alien;
pub mod interpreter;
pub mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, EventTarget, HtmlCanvasElement, MouseEvent};

use crate::interpreter::{GalaxyProtocol, Picture, Protocol};

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

#[derive(Debug)]
pub enum GeneralError {
    JsError(JsValue),
    AnyhowError(anyhow::Error),
}

impl From<JsValue> for GeneralError {
    fn from(v: JsValue) -> Self {
        Self::JsError(v)
    }
}

impl From<anyhow::Error> for GeneralError {
    fn from(e: anyhow::Error) -> Self {
        Self::AnyhowError(e)
    }
}

impl From<GeneralError> for JsValue {
    fn from(e: GeneralError) -> Self {
        match e {
            GeneralError::JsError(v) => v,
            GeneralError::AnyhowError(e) => format!("{:?}", e).into(),
        }
    }
}

#[wasm_bindgen(start)]
pub fn initialize() -> Result<(), JsValue> {
    initialize_internal().map_err(|err| format!("{:?}", err).into())
}

fn initialize_internal() -> Result<(), GeneralError> {
    const CANVAS_ID: &str = "galaxy_canvas";
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(CANVAS_ID).unwrap();
    let bounding_rect = canvas.get_bounding_client_rect();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
    let canvas_w = canvas.width();
    let canvas_h = canvas.height();
    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let mut protocol = Protocol::new::<GalaxyProtocol>()?;
    let mut pictures = Vec::new();

    let scale = 20.0;

    let mut update = move |event: UpdateEvent| {
        match event {
            UpdateEvent::Click(x, y) => {
                pictures = protocol.click(x, y).unwrap();
            }
        }
        draw(&pictures, &context, canvas_w as f64, canvas_h as f64, scale);
    };
    update(UpdateEvent::Click(0, 0));

    {
        let click_handler = Closure::wrap(Box::new(move |event: MouseEvent| {
            let x = (event.client_x() as f64 - bounding_rect.left()) / scale;
            let y = (event.client_y() as f64 - bounding_rect.top()) / scale;
            log(&format!("Clicked {}, {}", x, y));
            update(UpdateEvent::Click(x as i64, y as i64));
        }) as Box<dyn FnMut(_)>);
        let canvas_event_target: EventTarget = canvas.clone().dyn_into::<EventTarget>().unwrap();
        canvas_event_target
            .add_event_listener_with_callback("click", click_handler.as_ref().unchecked_ref())?;
        click_handler.forget();
    }

    Ok(())
}

fn draw(
    pictures: &Vec<Picture>,
    context: &CanvasRenderingContext2d,
    canvas_w: f64,
    canvas_h: f64,
    scale: f64,
) {
    context.set_fill_style(&"black".into());
    context.fill_rect(0.0, 0.0, canvas_w, canvas_h);
    context.set_fill_style(&"white".into());
    for pic in pictures {
        for &(x, y) in pic {
            context.fill_rect(x as f64 * scale, y as f64 * scale, scale, scale);
        }
    }
}

enum UpdateEvent {
    Click(i64, i64),
}
