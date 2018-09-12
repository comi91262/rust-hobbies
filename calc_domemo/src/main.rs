extern crate rand;

use rand::prelude::*;
use rand::prng::XorShiftRng;

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn main() {
    let mut rng = XorShiftRng::from_entropy();
    let input1 = read_vec::<u8>();
    let input2 = read_vec::<u8>();

    for i in input1 {
        println!("{}", i);
    }
    for i in input2 {
        println!("{}", i);
    }

    let mut v = vec![];
    for _ in 0..7 {
        v.push(rng.gen_range(1, 7))
    }
    println!("{:?}", v);
}
