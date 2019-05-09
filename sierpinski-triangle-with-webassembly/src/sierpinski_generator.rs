use rand_os::rand_core::RngCore;
use rand_os::OsRng;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point {
    #[wasm_bindgen(readonly)]
    pub x: u32,
    #[wasm_bindgen(readonly)]
    pub y: u32,
}

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const A: Point = Point { x: WIDTH / 2, y: 0 };
const B: Point = Point { x: 0, y: HEIGHT };
const D: Point = Point {
    x: WIDTH,
    y: HEIGHT,
};

pub static mut PRE_X: u32 = 400;
pub static mut PRE_Y: u32 = 400;

pub fn generate() -> Point {
    // let mut os_rng = OsRng::new().unwrap();

    // let mut key = [0u8; 16];
    // os_rng.fill_bytes(&mut key);
    // let random_number = os_rng.next_u64() % 3;

    let p = match kiss() % 3 {
        0 => A,
        1 => B,
        2 => D,
        _ => unreachable!(),
    };

    unsafe {
        let x = (p.x + PRE_X) / 2;
        let y = (p.y + PRE_Y) / 2;
        PRE_X = x;
        PRE_Y = y;
        Point { x: x, y: y }
    }
}

// interner Zustand
pub static mut X: u64 = 1066149217761810;
pub static mut Y: u64 = 362436362436362436;
pub static mut Z: u64 = 1234567890987654321;
pub static mut C: u64 = 123456123456123456;

fn kiss() -> u64 {
    unsafe {
        // Linearer Kongruenzgenerator
        X = 6906969069 * X + 1234567;

        // Xorshift
        Y ^= Y << 13;
        Y ^= Y >> 17;
        Y ^= Y << 43;

        // Multiply-with-carry
        let t = (Z << 58) + C;
        C = C + Z >> 6;
        Z = Z + t;
        C = C + Z << t;

        X + Y + Z
    }
}
