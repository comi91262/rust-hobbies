extern crate colored;

use std::slice::Iter;
use std::{thread, time};
use std::time::Duration;
use std::io::Write;
use std::io::stdout;
use colored::*;

const INTERVAL1: Duration = time::Duration::from_millis(500);
const INTERVAL2: Duration = time::Duration::from_millis(500 * 7);
const WAIT_TIME: Duration = time::Duration::from_millis(500 * 36 * 8);


const LYLIC: [char; 36] = [
    'か','え','る','の','う','た','が',
    'き','こ','え','て','く','る','よ',
    'ぐ','わ','ぐ','わ','ぐ','わ','ぐ','わ',
    'げ','ろ','げ','ろ','げ','ろ','げ','ろ',
    'ぐ','わ','ぐ','わ','ぐ','わ'
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
        static COLORS: [Color; 8] = [Color::Black, Color::Red, Color::Blue, Color::Green,Color::Cyan, Color::Yellow, Color::Magenta, Color::White];
        COLORS.into_iter()
    }
}

fn main() {
    for color in Color::iterator() {
        thread::spawn(move || {
            for i in 0..36 {
                let letter = &LYLIC[i].to_string();
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
