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
    //

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
    for &digit2 in first {
        for &digit1 in second {
            let product = digit1 * digit2;
            numbers.push(format!("{:02}", product));
        }
    }
    numbers
}

fn print(first_digits:Vec<i32>, second_digits:Vec<i32>, digit_vec:Vec<String>) {
    let mut num_columns = 0;
    let mut num_rows = 0;
    for i in 0..first_digits.len() {
        if i == 0 {
            num_columns +=9
        } else {
            num_columns += 4
        }
    }
    for j in 0..second_digits.len() {
        if j == 0 {
            num_rows += 9
        } else {
            num_rows += 4
        }
    }

    for i in 0..num_rows {  // cols
        for j in 0..num_columns { // rows
            if (i == 0 || i == num_rows - 1) && (j == 0 || j == num_columns - 1) {
                print!("+");
            } else if i == 0 || i == num_rows - 1 {
                print!("-");
            } else if j == 0 || j == num_columns - 1 {
                print!("|");
            } else if i == 1 {
                if j % 4 == 0 {
                    print!("{}", first_digits[j / 4 - 1]);
                } else {
                    print!(" ");
                }
            }

            else {
                print!(" ");
            }
        }
        println!();
    }

}