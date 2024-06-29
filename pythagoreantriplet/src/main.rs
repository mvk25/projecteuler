fn main() {
    for a in 100..300 {
        for b in 250..450 {
            let mut c = (a as f64).powf(2.0) + (b as f64).powf(2.0);
            c = c.sqrt();

            if (a as f64) + (b as f64) + c == 1000.0 {
                println!("{a} {b} {c}");
                println!("{}", (a as f64) * (b as f64) * c);
                break;
            }
        }
    }
}
