pub fn modular_exponential(base: u128, exp: u64, modular: u64) -> u64 {
    let mut e = exp;
    let mut b = base;
    let mut result = 1;
    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % modular as u128;
        }
        println!("b: {} e: {} result: {}", b, e, result);
        b = (b * b) % modular as u128;
        e = e / 2;
    }

    result as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_power_1() {
        let result = modular_exponential(12, 17, 19);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_mod_power_2() {
        let result = modular_exponential(8, 4294967296, 4294967299);
        assert_eq!(result, 4096);
    }

    #[test]
    fn test_mod_powe_3() {
        let result: u64 = modular_exponential(843, 4294967300, 4294967927);
        assert_eq!(result, 1389354282);
    }

    #[test]
    fn test_mod_power_4() {
        let result = modular_exponential(0xFFFF_FFFF_FFFF_FFC5 ,4_340_425_873_327_658_043, 12_669_955_479_143_291_250);
        assert_eq!(result, 3499928332443148193);
    }

}
