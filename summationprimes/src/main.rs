fn isprime(num: u64, primes: &Vec<u64>) -> bool {
    let sqrt = (num as f64).sqrt() as u64;

    for &prime in primes.iter() {
        if prime > sqrt { break; }

        if num % prime == 0 { return false; }
    }
    true
}

fn main() {
    let mut primes: Vec<u64> = vec![];
    primes.push(2 as u64);

    for i in 3..2_000_000 {
        if isprime(i, &primes) { primes.push(i as u64); }
    }

    let total: u64 = primes.iter().sum();
    println!("{:?}", total);
    println!();
    println!();
    println!("{}", primes.len());

    
}
