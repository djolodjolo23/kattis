use core::num;
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
    //®868


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

fn print(first_digits: Vec<i32>, mut second_digits: Vec<i32>, mut digit_vec: Vec<String>) {
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

    let mut _range = 2;
    let mut final_product = digit_vec.pop().unwrap();

    let num_of_digits_sum:usize = final_product.len();
    let mut vertical_digits_needed = vertical_numbers_needed(num_of_digits_sum, num_columns);

    let split_vec = split_digit_vec(digit_vec.clone(), vertical_digits_needed);

    for _ in 0..vertical_digits_needed {
        _range += 4;
    }


    for i in 0..num_rows {
        for j in 0..num_columns {
            match (i, j) {

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


                (i, j) if j < num_columns - 2 && (i - 3) % 2 == 0 && i < _range => { // 3, 5, 7, 9, 11...
                    let row_idx = i / 2 - 1;
                    let maybe_idx: Option<usize> = if row_idx % 2 == 0 {
                        // For even rows: valid j are 3, 7, 11, …
                        if j < 3 || ((j - 3) % 4 != 0) {
                            None
                        } else {
                            Some((j - 3) / 4)
                        }
                    } else {
                        // For odd rows: valid j are 5, 9, 13, …
                        if j < 5 || ((j - 5) % 4 != 0) {
                            None
                        } else {
                            Some((j - 5) / 4)
                        }
                    };

                    if let Some(idx) = maybe_idx {
                        if idx < split_vec[row_idx].len() {
                            let digit: String = split_vec[row_idx][idx].clone();
                            print!("{}", digit);
                        } else {
                            print!(" ");
                        }
                    } else {
                        match (i,j) {
                            (i, j) if i >= 7 && (i - 7) % 4 == 0 && i <= _range && j == 1 => print!("/"),

                            (i, j) if (i - 3) % 4 == 0 && i <= _range && (j < 2 || j > num_columns - 3 || j % 4 == 0) => print!(" "),
                            (i, j) if (i - 3) % 4 == 0 && i <= _range && ((j - 2) % 4 == 0) => print!("|"),
                            (i, j) if (i - 3) % 4 == 0 && i <= _range && (j > 3 && (j - 5) % 4 == 0) => print!("/"),

                            (i, j) if (i - 5) % 4 == 0 && i <= _range && (j > num_columns - 3) => print!(" "),
                            (i, j) if (i - 5) % 4 == 0 && i <= _range && j > 3 && (j - 4) % 4 == 0 => print!(" "),
                            (i, j) if (i - 5) % 4 == 0 && i <= _range && j >= 3 && (j - 3) % 4 == 0 => print!("/"),
                            (i, j) if (i - 5) % 4 == 0 && i <= _range && j > 1 && ((j - 2)) % 4 == 0 => print!("|"),


                            (i, j) if (i - 5) % 4 == 0 && i <= _range && j == 1 => { //TODO: here I need to check how to crate a logic for printing the final product
                                if !final_product.is_empty() {
                                    let first_char = final_product.remove(0);
                                    print!("{}", first_char);
                                    vertical_digits_needed -= 1;
                                }
                            }
                            (i, j) if (i - 5) % 4 == 0 && i <= _range && j == 1 => {
                                vertical_slots_available -= 1;
                                continue;
                            }
                            _ => print!(" "),
                        }
                    }
                },

                (i, j) if i >= 4 && (i - 4) % 4 == 0 && i <= _range && (j < 2) => print!(" "),
                (i, j) if i >= 4 && (i - 4) % 4 == 0 && i <= _range && (j > 2 && j < num_columns - 2) && j % 2 == 1 => print!(" "),
                (i, j) if i >= 4 && (i - 4) % 4 == 0 && i <= _range && ((j - 2) % 4 == 0) => print!("|"),
                (i, j) if i >= 4 && (i - 4) % 4 == 0 && i <= _range && (j > 3 && (j - 4) % 4 == 0) => print!("/"),
                (i, j) if i >= 4 && (i - 4) % 4 == 0 && i <= _range && j == num_columns - 2 => print!("{}", second_digits.remove(0)),

                (i, j) if i == _range + 1 => {
                    match (i, j) {
                        (i, j) if (j - 1) % 4 == 0 && final_product.chars().count() < num_of_digits_sum && final_product.chars().count() > 0 => {

                            print!("/");
                        }
                        (i, j) if j >= 3 && (j - 3) % 4 == 0 => {
                            let idx = (j - 3) / 4;
                            if !final_product.is_empty() {
                                let digit = final_product.remove(0);
                                print!("{}", digit);
                            } else {
                                print!(" ");
                            }
                        }

                        _ => print!(" "),
                    }
                }

                _ => print!(" "),
            }
        }
        println!();
    }
}

fn split_digit_vec(digit_vec: Vec<String>, vertical_digits_needed:usize) -> (Vec<Vec<String>>) {
    let mut final_split_vec: Vec<Vec<String>> = Vec::new(); 
    let mut split_vec: Vec<Vec<String>> = split_into_chunks(digit_vec.clone(), vertical_digits_needed);

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
    let chunk_size = digit_vec.len() / vertical_digits_needed;
    let mut split_vec = Vec::new();

    for i in 0..vertical_digits_needed {
        let start = i * chunk_size;
        let end = start + chunk_size;
        split_vec.push(digit_vec[start..end].to_vec());
    }


    split_vec
}


fn vertical_numbers_needed(total_digits: usize, columns:usize) -> usize {
    total_digits - (columns / total_digits)
}

