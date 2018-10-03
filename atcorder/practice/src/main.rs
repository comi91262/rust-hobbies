
fn main() {
    let mut a:u16 = read();
    let b:Vec<u16> = read_vec();
    let c:String = read();

    for value in b.iter() {
        a = a + value;
    }
    
    println!("{} {}", a, c);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}
