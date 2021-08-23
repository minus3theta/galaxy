mod alien;
pub mod interpreter;
pub mod utils;

use std::sync::Mutex;

use itertools::Itertools;
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
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

#[derive(Debug, Clone)]
struct View {
    pictures: Vec<Picture>,
    offset: (i64, i64),
    scale: f64,
}

impl View {
    pub fn new() -> Self {
        Self {
            pictures: Vec::new(),
            offset: (0, 0),
            scale: 20.0,
        }
    }
}

fn get_context() -> CanvasRenderingContext2d {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(CANVAS_ID).unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    context
}

static VIEW: Lazy<Mutex<View>> = Lazy::new(|| Mutex::new(View::new()));
static PROTOCOL: Lazy<Mutex<Protocol>> =
    Lazy::new(|| Mutex::new(Protocol::new::<GalaxyProtocol>().unwrap()));
thread_local! {
    static CONTEXT: CanvasRenderingContext2d = get_context();
}

#[wasm_bindgen(start)]
pub fn initialize() {
    spawn_local(initialize_internal());
}

const CANVAS_ID: &str = "galaxy_canvas";

async fn initialize_internal() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(CANVAS_ID).unwrap();
    let bounding_rect = canvas.get_bounding_client_rect();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
    let canvas_w = canvas.width();
    let canvas_h = canvas.height();

    update(UpdateEvent::Click(0.0, 0.0), canvas_w, canvas_h).await;

    {
        let click_handler = Closure::wrap(Box::new(move |event: MouseEvent| {
            let x = event.client_x() as f64 - bounding_rect.left();
            let y = event.client_y() as f64 - bounding_rect.top();
            spawn_local(update(UpdateEvent::Click(x, y), canvas_w, canvas_h));
        }) as Box<dyn FnMut(_)>);
        let canvas_event_target: EventTarget = canvas.clone().dyn_into::<EventTarget>().unwrap();
        canvas_event_target
            .add_event_listener_with_callback("click", click_handler.as_ref().unchecked_ref())
            .unwrap();
        click_handler.forget();
    }
}

async fn update(event: UpdateEvent, canvas_w: u32, canvas_h: u32) {
    let mut view = VIEW.lock().unwrap();
    let mut protocol = PROTOCOL.lock().unwrap();
    match event {
        UpdateEvent::Click(x, y) => {
            let x = (x / view.scale) as i64 - view.offset.0;
            let y = (y / view.scale) as i64 - view.offset.1;
            view.pictures = protocol.click(x, y).await.unwrap();
            log(&format!("Clicked: ({}, {})", x, y));
            let (new_offset, size) = get_offset(&view.pictures);
            view.scale = (canvas_w as f64 / size.0 as f64).min(canvas_h as f64 / size.1 as f64);
            view.offset = new_offset;
        }
    }
    draw(
        &view.pictures,
        canvas_w as f64,
        canvas_h as f64,
        view.offset,
        view.scale,
    );
}

fn get_offset(pictures: &Vec<Picture>) -> ((i64, i64), (i64, i64)) {
    let (left, right) = pictures
        .iter()
        .flatten()
        .map(|&(x, _)| x)
        .minmax()
        .into_option()
        .unwrap_or_default();
    let (top, bottom) = pictures
        .iter()
        .flatten()
        .map(|&(_, y)| y)
        .minmax()
        .into_option()
        .unwrap_or_default();
    ((-left + 1, -top + 1), (right - left + 3, bottom - top + 3))
}

const COLOR_OF_LAYER: &[&'static str] = &[
    "rgba(255,127,127,0.8)",
    "rgba(127,255,127,0.8)",
    "rgba(127,127,255,0.8)",
];

fn color_of_layer(layer: usize) -> &'static str {
    COLOR_OF_LAYER
        .get(layer)
        .unwrap_or(&"rgba(255,255,255,0.5)")
}

fn draw(pictures: &Vec<Picture>, canvas_w: f64, canvas_h: f64, offset: (i64, i64), scale: f64) {
    CONTEXT.with(|context| {
        context.set_fill_style(&"black".into());
        context.fill_rect(0.0, 0.0, canvas_w, canvas_h);
        context.set_fill_style(&"rgba(255,255,255,0.5)".into());
        for (layer, pic) in pictures.iter().enumerate().rev() {
            context.set_fill_style(&color_of_layer(layer).into());
            for &(x, y) in pic {
                context.fill_rect(
                    (x + offset.0) as f64 * scale,
                    (y + offset.1) as f64 * scale,
                    scale,
                    scale,
                );
            }
        }
    })
}

#[derive(Debug, Clone)]
enum UpdateEvent {
    Click(f64, f64),
}
