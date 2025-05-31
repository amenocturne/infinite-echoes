use wasm_bindgen::prelude::*;

pub type GameTime = f64;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    pub fn random() -> f32;
}
