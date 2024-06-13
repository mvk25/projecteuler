mod isprime;
use isprime::is_prime;

fn isqrt(n: usize) -> usize {
    if n == 0 { return n; }
    let mut s = (n as f64).sqrt() as usize;
    s = (s + n / s) >> 1;
    if s * s > n { s - 1 } else { s }
}

fn perfect_square(n: usize) -> isize {
    match n & 0xf {
        0 | 1 | 4 | 9 => {
            let t = isqrt(n);
            if t*t == n { t as isize } else { -1 }
        },
        _ => -1,
    }
}

fn get_largest_multiples(number: isize) -> (isize, isize) {
    let mut i = 1isize;
    let a: isize;
    let b: isize;

    loop {
        let pos_perfect_square = number + (i * i);
        let sqrt: isize = perfect_square(pos_perfect_square as usize);
        if sqrt != -1 {
            a = sqrt - i;
            b = sqrt + i;
            break;
        }
        i += 1;
    }
    // println!("{} {}", a, b);
    return (a, b);
}


fn get_largest_number(number: isize) -> isize {
    let a: isize;
    let b: isize;

    (a, b) = get_largest_multiples(number);
    if is_prime(b) || is_prime(a) {
        if a > b { a }
        else { b }
    } else {
        let m = get_largest_number(a);
        let n = get_largest_number(b);
        if m > n {
            get_largest_number(m)
        } else {
            get_largest_number(n)
        }
    }
}


fn main() {
    let number = 600851475143isize;
    // let number = 1234169isize;
    let b = get_largest_number(number);
    println!("{}", b);
    
}



