mod sierpinski_generator;

use crate::sierpinski_generator::Point;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate() -> Point {
    sierpinski_generator::generate()
}
