fn is_valid_code(mut code: u32, part_2: bool) -> bool {
    let mut previous_digit = 10;
    let mut current_digit = code % 10;
    let mut digit_counts = [0; 10];

    loop {
        if code == 0 {
            break;
        }

        if current_digit > previous_digit {
            return false;
        }

        digit_counts[current_digit as usize] += 1;

        previous_digit = current_digit;
        code /= 10;
        current_digit = code % 10;
    }

    for &count in digit_counts.iter() {
        if part_2 {
            if count == 2 {
                return true;
            }
        } else {
            if count >= 2 {
                return true;
            }
        }
    }

    false
}

fn main() {
    let min = 152085;
    let max = 670283;
    let mut part_1_count = 0;
    let mut part_2_count = 0;

    for number in min..(max + 1) {
        if is_valid_code(number, false) {
            part_1_count += 1;
        }
        if is_valid_code(number, true) {
            part_2_count += 1;
        }
    }

    println!("Number of valid passwords (part 1): {}", part_1_count);
    println!("Number of valid passwords (part 2): {}", part_2_count);
}
