fn isprime(num: u32, primes: &Vec<u32>) -> bool {
    if num < 2 {
        return false;
    }

    let sqrt = (num as f64).sqrt() as u32;
    for &prime in primes.iter() {
        if prime > sqrt {
            break;
        }
        if num % prime == 0 {
            return false;
        }
    }
    true
}

fn nth_prime(pos: u32) -> u32 {
    if pos == 1 {
        return 2;
    }

    let mut primes = vec![2];
    let mut count = 1;
    let mut i = 3;

    while count < pos {
        if isprime(i, &primes) {
            primes.push(i);
            count += 1;
        }

        i += 2;
    }

    primes[(pos - 1) as usize]
}

fn main() {
    let nth = nth_prime(10001);
    println!("{}", nth);
}


