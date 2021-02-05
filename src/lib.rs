mod mapper;
mod note_viewer;
mod app;

pub use app::MidiMapper;

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    let app = MidiMapper::default();
    eframe::start_web(canvas_id, Box::new(app))
}
