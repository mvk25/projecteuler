pub fn is_prime(a: isize) -> bool {
    if (2..a).all(|x| a % x != 0) {
        true
    } else { false }
}
