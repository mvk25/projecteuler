fn divisors(num: u32) -> Vec<u64> {
    let mut divs: Vec<u64> = Vec::new();
    for i in 2..=num {
        if num % i == 0 {
            divs.push(i as u64);
        }
    }
    divs
}


fn main() {
    let mut additive: u64 = 2;
    let mut num: u64 = 1;
    loop {
        num += additive;
        additive += 1;
        println!("{num} {additive}");
        if num == 28 {

            break;
        }
    }

    let div = divisors(4294901760);
    println!("{:?} {}", div, div.len());
}
