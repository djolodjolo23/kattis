use std::io;
use std::mem::forget;

fn main() {
    // let mut input = String::new();
    //
    // io::stdin().read_line(&mut input)
    //     .expect("Failed to read line");
    //
    // let numbers: Vec<i32> = input
    //     .trim()
    //     .split_whitespace()
    //     .map(|num| num.parse().expect("Not a number"))
    //     .collect();
    //
    //
    //®

    let first_num = 345;
    let second_num = 56;
    let product = first_num * second_num;

    let first_digits: Vec<i32> = first_num.to_string().chars().map(|d| d.to_digit(10).unwrap() as i32).collect();
    let second_digits: Vec<i32> = second_num.to_string().chars().map(|d| d.to_digit(10).unwrap() as i32).collect();

    let mut digit_vec = sum_each_digit(&first_digits, &second_digits);
    digit_vec.push(format!("{:02}", product));

    print(first_digits, second_digits, digit_vec);
}


fn sum_each_digit(first:&Vec<i32>, second:&Vec<i32>) -> Vec<String> {
    let mut numbers:Vec<String> = Vec::new();
    for &digit2 in second {
        for &digit1 in first {
            let product = digit1 * digit2;
            numbers.push(format!("{:02}", product));
        }
    }
    numbers
}

fn print(first_digits: Vec<i32>, second_digits: Vec<i32>, mut digit_vec: Vec<String>) {
    let mut num_columns = 0;
    let mut num_rows = 0;
    let mut vertical_slots_available = 0;
    for i in 0..first_digits.len() {
        num_columns += if i == 0 { 9 } else { 4 };
    }
    for j in 0..second_digits.len() {
        num_rows += if j == 0 { 9 } else { 4 };
        vertical_slots_available += 1;
    }

    let sum_num_of_digits:usize = digit_vec.last().map(|s| s.len()).unwrap_or(0);
    let vertical_digits_needed = vertical_numbers_needed(sum_num_of_digits, num_columns);

    for i in 0..num_rows {
        for j in 0..num_columns {
            match (i, j) {

                // Corners: top-left, top-right, bottom-left, bottom-right.
                (i, j) if (i == 0 || i == num_rows - 1) && (j == 0 || j == num_columns - 1) => {
                    print!("+")
                }
                (i, _) if i == 0 || i == num_rows - 1 => print!("-"),

                (_, j) if j == 0 || j == num_columns - 1 => print!("|"),

                (1, j) if j % 4 == 0 => {
                    let idx = j / 4 - 1;
                    if idx < first_digits.len() {
                        print!("{}", first_digits[idx]);
                    } else {
                        print!(" ");
                    }
                }
                (1, _) => print!(" "),
                (2, j) if j < 2 || j > num_columns - 3 => print!(" "),
                (2, j) => match (j - 2) % 4 {
                    0 => print!("+"),
                    _ => print!("-"),
                },

                (3, j) if j < 2 || j > num_columns - 3 || j % 4 == 0 => print!(" "),
                (3, j) if (j - 2) % 4 == 0 => print!("|"),
                (3, j) if j > 3 && (j - 5) % 4 == 0 => print!("/"),
                (3, j) => {
                    let idx = j / 3 - 1;
                    if idx < digit_vec.len() {
                        // Use the first character of the corresponding string.
                        let digit = digit_vec[idx].chars().next().unwrap_or(' ');
                        print!("{}", digit);
                    } else {
                        print!(" ");
                    }
                },
                (4, j) if j == num_columns - 2 => print!("{}", second_digits[i / 4 - 1]),
                (4, j) if (j - 1) % 2 == 0 => print!(" "),
                (4, j) if (j - 2) % 4 == 0 => print!("|"),
                (4, j) if j > 3 && (j - 4) % 4 == 0 => print!("/"),

                // (5, j) if j == 1 && vertical_slots_available == vertical_digits_needed => {
                //      if let Some( sum) = digit_vec.last_mut() {
                //          if !sum.is_empty() {
                //              let first_char = sum.remove(0); // Removes first char
                //              println!("{}", first_char); // Print only the removed character
                //          }
                //      }
                // }
                // (5, j) if j == 1 && vertical_slots_available > vertical_digits_needed => {
                //     vertical_slots_available -= 1;
                //     continue;
                // }

                (5, j) if j > 1 && (j - 2) % 4 == 0 => {
                    print!("|")
                }
                (5, j) if j > 1 && j < num_columns - 2 && (j - 3) % 4 == 0 => {
                    print!("/")
                }
                (5, j ) if j > 3 && (j - 4) % 4 == 0 => {
                    print!(" ")
                }
                (5, j) if j > 1 => {
                    let idx = j / 4 - 1;
                    if idx < digit_vec.len() {
                        let digit = digit_vec[idx].chars().nth(1).unwrap_or(' ');
                        print!("{}", digit);
                    } else {
                        print!(" ");
                    }

                }
                (5, j) if j == 1 && vertical_slots_available == vertical_digits_needed => {
                    if let Some( sum) = digit_vec.last_mut() {
                        if !sum.is_empty() {
                            let first_char = sum.remove(0); 
                            print!("{}", first_char); 
                        }
                    }
                }
                (5, j) if j == 1 && vertical_slots_available > vertical_digits_needed => {
                    vertical_slots_available -= 1;
                    continue;
                }
                // (5, j) if (j - 5) % 4 == 0 => {

                //  }

                _ => print!(" "),
            }
        }
        println!();
    }
}


fn vertical_numbers_needed(total_digits: usize, columns:usize) -> usize {
    total_digits - (columns / total_digits)
}

