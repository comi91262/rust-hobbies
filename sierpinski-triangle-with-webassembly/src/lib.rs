mod sierpinski_generator;

use crate::sierpinski_generator::Point;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn generate() -> Point {
    sierpinski_generator::generate()
}

#[wasm_bindgen]
pub fn debug(x: u32, y: u32) {
    alert(&format!("x: {}, y: {}", x, y));
}
