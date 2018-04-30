extern crate rand;

use rand::{thread_rng, Rng};

fn main() {
    let rand_string: String = thread_rng().gen_ascii_chars().take(30).collect();
    println!("{}", rand_string);
}
