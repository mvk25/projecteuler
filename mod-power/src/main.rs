pub fn modular_exponential(base: u128, exp: u64, modular: u64) -> u64 {
    let mut e = exp;
    let mut b = base;
    let mut result = 1;
    let mut bin: u8;
    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % modular as u128;
            bin = 1;
        } else {
            bin = 0;
        }
        println!("b: {} e: {} bin: {} result: {}", b, e, bin, result);
        b = (b * b) % modular as u128;
        e = e / 2;
    }

    result as u64
}

fn main() {
    println!("\nResult {}", modular_exponential(12, 17, 19));
}
