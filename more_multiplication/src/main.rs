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

    let lower_limit = 2;
    let mut _range = 2;
    let mut inner_box_index = 3;
    let mut final_product = digit_vec.pop().unwrap();

    let sum_num_of_digits:usize = final_product.len();
    let vertical_digits_needed = vertical_numbers_needed(sum_num_of_digits, num_columns);

    let split_vec = split_digit_vec(digit_vec.clone(), vertical_digits_needed);

    for _ in 0..vertical_digits_needed {
        _range += 4;
    }


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

                // the repetition logic from here

                (i, j) if (i - 2) % 4 == 0 && i <= _range && (j < 2 || j > num_columns - 3) => print!(" "),
                (i, j) if (i - 2) % 4 == 0 && i <= _range => match (j - 2) % 4 {
                    0 => print!("+"),
                    _ => print!("-"),
                },

                (i, j) if (i - 3) % 4 == 0 && i <= _range && (j < 2 || j > num_columns - 3 || j % 4 == 0) => print!(" "),
                (i, j) if (i - 3) % 4 == 0 && i <= _range && ((j - 2) % 4 == 0) => print!("|"), // here I need to stop
                (i, j) if (i - 3) % 4 == 0 && i <= _range && (j > 3 && (j - 5) % 4 == 0) => print!("/"),
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

                (5, j) if j > 1 && (j - 2) % 4 == 0 => {
                    print!("|")
                }
                (5, j) if j > 1 && j < num_columns - 2 && (j - 3) % 4 == 0 => {
                    print!("/")
                }
                (5, j ) if j > 3 && (j - 4) % 4 == 0 => {
                    print!(" ")
                }
                (5, j) if j > 1 && j < num_columns - 2 => {
                    let idx = j / 4 - 1;
                    if idx < digit_vec.len() {
                        let digit = digit_vec[idx].chars().nth(1).unwrap_or(' ');
                        print!("{}", digit);
                    } else {
                        print!(" ");
                    }

                }
                (5, j) if j == 1 && vertical_slots_available == vertical_digits_needed => {
                    if !final_product.is_empty() {
                        let first_char = final_product.remove(0);
                        print!("{}", first_char);
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

fn split_digit_vec(digit_vec: Vec<String>, vertical_digits_needed:usize) -> (Vec<Vec<String>>) {
    let mut final_split_vec: Vec<Vec<String>> = Vec::new(); 
    let mut split_vec: Vec<Vec<String>> = split_into_chunks(digit_vec.clone(), vertical_digits_needed);

    let rows_needed = vertical_digits_needed + vertical_digits_needed;

    // split digit_vec into rows with vertical_digits_needed number of digits


    for i in 0..split_vec.len() {
        let mut row_first = Vec::new();
        let mut row_second = Vec::new();
        for j in 0..split_vec[i].len() {
            row_first.push(split_vec[i][j].clone().chars().nth(0).unwrap().to_string());
            row_second.push(split_vec[i][j].clone().chars().nth(1).unwrap().to_string());
        }
        final_split_vec.push(row_first);
        final_split_vec.push(row_second);
    }
    final_split_vec
}

fn split_into_chunks(digit_vec: Vec<String>, vertical_digits_needed: usize) -> Vec<Vec<String>> {
    let chunk_size = digit_vec.len() / vertical_digits_needed; // Compute chunk size
    let mut split_vec = Vec::new();

    for i in 0..vertical_digits_needed {
        let start = i * chunk_size;
        let end = start + chunk_size;
        split_vec.push(digit_vec[start..end].to_vec()); // Push each chunk
    }

    split_vec
}


fn vertical_numbers_needed(total_digits: usize, columns:usize) -> usize {
    total_digits - (columns / total_digits)
}

