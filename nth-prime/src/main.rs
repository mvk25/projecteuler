fn isprime(num: u32) -> bool {
    (2..=num).all(|x| num % x != 0)
}

fn nth_prime(pos: u32) {
    let mut count = 0;
    let mut i = 3;
    loop {
        if isprime(i) { count += 1; }
        if count == pos { break; }
        i += 2;
    }

    println!("{}", i);
}

fn main() {
    nth_prime(10001);
}


