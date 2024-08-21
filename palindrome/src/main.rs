mod perfect_square;
use perfect_square::get_largest_multiples;

/** palindrome_checker - receives an input as a String
 * and checks if the number is a palindrome
 *
 * Returns a boolean -true if conditions are met otherwise false
 */
fn palindrome_checker(numstr: String) -> bool {
    if numstr.chars().rev().collect::<String>() == numstr { return true; }
    false
}


/** is_palindrome - receives an input of number as u64 and checks
 * if it is a palindrome
 */
fn is_palindrome(sus_pal: u64) -> bool {
    let num_to_string: String = sus_pal.to_string();
    
    palindrome_checker(num_to_string)
}


/** find_palindrome- palindrome checker between let say
 * 998001 and 948001
 */
fn find_palindrome() -> Vec<u64> {
    let upper_limit = 948001;
    let lower_limit = 900000;
    let mut vec_pals: Vec<u64> = Vec::new();
    for i in lower_limit..upper_limit {
        if is_palindrome(i) { vec_pals.push(i) }
    }

    vec_pals
}

fn main() {
    for i in find_palindrome().into_iter().rev() {
        let (a, b ) = get_largest_multiples(i as isize);
        println!("{a} {b}");
        if a.to_string().len() == 3 && b.to_string().len() == 3 {
            println!("{} {} {i}", a, b);
            break;
        }
    }
}

