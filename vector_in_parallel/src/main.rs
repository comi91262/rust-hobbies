extern crate rand;
extern crate rayon;

use rand::Rng;
use rayon::prelude::*;

fn main() {
    // [1]
    let mut vec = vec![String::new(); 100_000];

    // [2]
    vec.par_iter_mut().for_each(|p| {
        // [3]
        *p = rand::weak_rng().gen_ascii_chars().take(5).collect()
    });

    // [4]
    vec.par_sort_unstable();
}
