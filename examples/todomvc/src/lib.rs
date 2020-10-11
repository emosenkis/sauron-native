//#![deny(warnings)]
#[cfg(feature = "with-web")]
use sauron_native::backend::HtmlApp;
use sauron_native::Backend;
#[cfg(feature = "with-web")]
use wasm_bindgen::prelude::*;

pub mod app;

#[cfg(feature = "with-web")]
#[wasm_bindgen]
pub fn initialize(initial_state: &str) {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("must init");
    log::trace!("Initial state: {}", initial_state);
    HtmlApp::init(app::Model::new());
}
