fn extendedAlgorithm(m: i64, n: i64) {
    let mut a = m;
    let mut b = n;
    let mut q = a / b;
    let mut r = a % b;
    let mut s1 = 1;
    let mut s2 = 0;
    let mut s3 = 1;
    let mut t1 = 0;
    let mut t2 = 1;
    let mut t3 = t1 - (q * t2);

    while r != 0 {
        a = b;
        b = r;
        q = a / b;
        r = a % b;
        s1 = s2;
        s2 = s3;
        s3 = s1 - (q * s2);
        t1 = t2;
        t2 = t3;
        t3 = t1 - (q * t2);
    }

    println!("GCD of 161 and 28 is {b}, {s2}, {t2}");
}

fn main() {
    extendedAlgorithm(161, 28);
}
