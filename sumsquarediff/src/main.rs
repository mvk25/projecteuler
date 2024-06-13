/** sum of squares
 */

fn sum_of_squares(a: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=a {
        sum += i.pow(2);
    }
    sum
}

/** square of sums
 */
fn squares_of_sums(a: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=a {
        sum += i;
    }
    sum.pow(2)

}

fn difference() {
    let lower_limit = 1;
    let upper_limit = 100;
    println!("{}", squares_of_sums(upper_limit) - sum_of_squares(upper_limit));
}

fn main() {
    difference();
}
