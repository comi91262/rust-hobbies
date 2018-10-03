fn main() {
    let a:Vec<u32> = read_vec();
    let mut b = 1;
    
    for v in a.iter() {
        b = b * v;
    }

    if b % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }


}

//fn read<T: std::str::FromStr>() -> T {
//    let mut s = String::new();
//    std::io::stdin().read_line(&mut s).ok();
//    s.trim().parse().ok().unwrap()
//}
//
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}
