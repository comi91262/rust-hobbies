extern crate colored;

use std::slice::Iter;
use std::{thread, time};
use std::time::Duration;
use std::io::Write;
use std::io::stdout;
use colored::*;
use std::str;

const INTERVAL1: Duration = time::Duration::from_millis(500);
const INTERVAL2: Duration = time::Duration::from_millis(500 * 7);
const WAIT_TIME: Duration = time::Duration::from_millis(500 * 36 * 8);


const LYLIC:[[u8; 3]; 36] = [
    [227, 129, 139],//か
    [227, 129, 136],//え
    [227, 130, 139],//る
    [227, 129, 174],//の
    [227, 129, 134],//う
    [227, 129, 159],//た
    [227, 129, 140],//が
    [227, 129, 141],//き
    [227, 129, 147],//こ
    [227, 129, 136],//え
    [227, 129, 166],//て
    [227, 129, 143],//く
    [227, 130, 139],//る
    [227, 130, 136],//よ
    [227, 129, 144],//ぐ
    [227, 130, 143],//わ
    [227, 129, 144],//ぐ
    [227, 130, 143],//わ
    [227, 129, 144],//ぐ
    [227, 130, 143],//わ
    [227, 129, 144],//ぐ
    [227, 130, 143],//わ
    [227, 129, 146],//げ
    [227, 130, 141],//ろ
    [227, 129, 146],//げ
    [227, 130, 141],//ろ
    [227, 129, 146],//げ
    [227, 130, 141],//ろ
    [227, 129, 146],//げ
    [227, 130, 141],//ろ
    [227, 129, 144],//ぐ
    [227, 130, 143],//わ
    [227, 129, 144],//ぐ
    [227, 130, 143],//わ
    [227, 129, 144],//ぐ
    [227, 130, 143],//わ
    ];


enum Color {
    Red,
    Blue,
    Green,
    Cyan,
    Yellow,
    Magenta,
    Black,
    White
}

impl Color {
    pub fn iterator() -> Iter<'static, Color> {
        static COLORS: [Color; 8] = [Color::Black, Color::Red, Color::Blue, Color::Green, Color::Cyan, Color::Yellow, Color::Magenta, Color::White];
        COLORS.into_iter()
    }
}

fn main() {

    for color in Color::iterator() {
        thread::spawn(move || {
            for i in 0..36 {
                let letter = str::from_utf8(&LYLIC[i]).unwrap();
                match color {
                    Color::Black => print!("{}", letter.black()),
                    Color::Red => print!("{}", letter.red().on_cyan()),
                    Color::Blue => print!("{}", letter.blue().on_yellow()),
                    Color::Green => print!("{}", letter.green().on_magenta()),
                    Color::Magenta => print!("{}", letter.magenta().on_green()),
                    Color::Yellow => print!("{}", letter.yellow().on_blue()),
                    Color::Cyan => print!("{}", letter.cyan().on_red()),
                    Color::White => print!("{}", letter.white().on_black()),
                }
                stdout().flush().unwrap();
                thread::sleep(INTERVAL1);

                if i % 18 == 17 {
                    println!("");
                }
            }
        });
        thread::sleep(INTERVAL2);
    }

    thread::sleep(WAIT_TIME);

}
