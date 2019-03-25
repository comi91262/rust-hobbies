fn main() {
    let s = String::from("gmbh}i@");
    let mut s_bytes = s.into_bytes();

    for _ in 0..26 {
        for b in s_bytes.iter_mut() {
            *b += 1
        }
        println!("{}", String::from_utf8(s_bytes.clone()).unwrap());
    }
}
