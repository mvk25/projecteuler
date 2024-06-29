
// let fibonacci(i) {
//     if i == 1 {
//         1
//     } else if i == 1 {
//         1
//     } else {
//         fibonacci(i - 1) + fibonacci(i - 2)
//     }

// }

fn main() {
    let mut vec: Vec<i32> = vec![];

    vec.push(0);
    vec.push(1);
    let mut i = 2;
    println!("{:?}", vec);
    loop {
        let a = vec[i - 1] + vec[i - 2];
        if a > 4_000_000 {
            break;
        }
        vec.push(a);
        i += 1;
    }

    println!("{:?}", vec);
    let mut sum = 0;
    for i in vec.into_iter() {
        if i % 2 == 0 {
            sum += i;
        }
    }
    println!("{}", sum);
}
