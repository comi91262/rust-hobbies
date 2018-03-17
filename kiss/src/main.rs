#![feature(i128_type)]
// interner Zustand
pub static mut X: u128 =  1066149217761810;
pub static mut Y: u128 =  362436362436362436;
pub static mut Z: u128 =  1234567890987654321;
pub static mut C: u128 =  123456123456123456;


fn kiss() -> u128 {
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

fn main() {
    println!("{}", kiss());
}



