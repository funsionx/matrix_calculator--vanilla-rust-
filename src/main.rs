use std::io;

mod basic;
mod operations;

use crate::basic::print::print_matrix;
use crate::basic::read::read_matrix;
use crate::operations::{add::add_matrix, multiply::multiply_matrix, subtract::subtract_matrix};

fn main() {
    println!("Enter the dimensions of the first matrix:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let dimensions: Vec<_> = input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    let rows_first = dimensions[0];
    let cols_first = dimensions[1];

    println!("Enter the first matrix:");
    let mut first = Vec::with_capacity(rows_first * cols_first);
    for _ in 0..rows_first {
        let row = read_matrix();
        first.extend_from_slice(&row[0..cols_first]);
    }

    println!("Enter the dimensions of the second matrix:");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let dimensions: Vec<usize> = input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    let rows_second = dimensions[0];
    let cols_second = dimensions[1];

    println!("Enter the second matrix:");
    let mut second = Vec::with_capacity(rows_second * cols_second);
    for _ in 0..rows_second {
        let row = read_matrix();
        second.extend_from_slice(&row[0..cols_second]);
    }

    if cols_first != rows_second {
        println!("Error: Cannot multiply matrices with these dimensions.");
        return;
    }

    println!("Enter the operation (+, -, *): ");
    input.clear();
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read input.");
    let operation = operation.trim().chars().next().expect("Invalid input.");

    println!("The answer is: ");

    match operation {
        '+' => print_matrix(
            &add_matrix(&first, &second, rows_first, cols_first),
            rows_first,
            cols_first,
        ),
        '*' => print_matrix(
            &multiply_matrix(&first, &second, rows_first, cols_first, cols_second),
            rows_first,
            cols_first,
        ),
        '-' => print_matrix(
            &subtract_matrix(&first, &second, rows_first, cols_first),
            rows_first,
            cols_first,
        ),
        _ => println!("You printed wrong symbol, try whole prompt another time"),
    };
}
