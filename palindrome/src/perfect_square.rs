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

pub fn get_largest_multiples(number: isize) -> (isize, isize) {
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
    return (a, b);
}
