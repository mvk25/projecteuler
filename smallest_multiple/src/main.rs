/** gcd - finds the gcd of two numbers
 *
 * params a and b
 *
 * Returns a u64
 */
fn gcd(a: u64, b: u64) -> u64 {
    if a % b == 0 { return b; }
    gcd(b, a % b)
}

/** - performs the lcm of two integers
 */
fn lcm(a: u64, b: u64) -> u64 {
    let i_gcd = gcd(a, b);
    let product = a * b;
    product / i_gcd
}

fn main() {
    let a = 2520;
    let mut multiple = lcm(1, 2);
    for i in 3..=20 {
        multiple = lcm(multiple, i);
    }
    println!("{}", multiple);
}
