use std::env;

fn compute_possible_passwords(lower_bound: u32, upper_bound: u32) -> u32 {
    let mut count: u32 = 0;
    for number in lower_bound..upper_bound {
        let mut has_pair = false;
        let mut all_ascending = true;

        let mut previous_digit: Option<u32> = None;
        let mut streak: u16 = 0;

        let digits: Vec<u32> = number.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
        let digit_count: usize = digits.len();

        for (i, digit) in digits.into_iter().enumerate() {
            if digit < previous_digit.unwrap_or(0) {
                all_ascending = false;
                break;
            }

            if digit == previous_digit.unwrap_or(digit) {
                streak += 1;

                // If this is the last number, and we have a streak of two, it counts!
                if i + 1 == digit_count && streak == 2 {
                    has_pair = true;
                }
            } else if digit != previous_digit.unwrap() && streak == 2 {
                has_pair = true;
            } else {
                streak = 1;
            }

            previous_digit = Some(digit)
        }
        
        if all_ascending && has_pair {
            count += 1;
        }

    }

    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let lower_bound: u32 = args[1].parse().unwrap();
    let upper_bound: u32 = args[2].parse().unwrap();

    let password_count = compute_possible_passwords(lower_bound, upper_bound);

    println!("Possible Passwords: {}", password_count);
}
