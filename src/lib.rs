mod state;
mod vertex;
mod app;
mod texture;

use app::App;

use winit::{
    event_loop::{EventLoop},
};
use wasm_bindgen::prelude::*;

pub fn run() -> anyhow::Result<()> {
    console_log::init_with_level(log::Level::Info).unwrap_throw();

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new(&event_loop);
    event_loop.run_app(&mut app)?;

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    alert("Hello, welcome to WebGPU rendering. \nPlease make sure WebGPU is supported and enabled by the browser");

    console_error_panic_hook::set_once();
    run().unwrap_throw();

    Ok(())
}
