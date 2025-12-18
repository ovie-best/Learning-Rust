/*
Define a `color_to_number` function that accepts a 'color'
parameter (a string). Use if, else if, and else
statements to return a corresponding numeric value based
on the following rules:
1. If the color is "red", return 1.
2. If the color is "green", return 2.
3. If the color is "blue", return 3.
4. If the color is any other string, return 0.

Refactor the function above to use the `match` statement
instead of if, else if, and else.

Define a `factorial` function that calculates the
factorial of a number. The factorial is the product
of multiplying a number by every incremental
number leading up to it, starting from 1.

Examples:
The factorial of 5 is 5 * 4 * 3 * 2 * 1 = 120
factorial(5) should return 120.

The factorial of 4 is 4 * 3 * 2 * 1 = 24
factorial(4) should return 24.

Implement two solutions/functions for the problem.
The first solution should not use recursion.
The second solution should use recursion.
*/

fn color_to_number(color: &str) -> i32 {
    // if color == "red" {
    //     1
    // } else if color == "green" {
    //     2
    // } else if color == "blue" {
    //     3
    // } else {
    //     0
    // }

    // refactored using match
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn non_recursive_factorial(num: u32) -> u32 {
    let mut result = 1;
    for i in 1..=num {
        result *= i;
    }
    result
}

fn recursive_factorial(num: u32) -> u32 {
    if num == 0 || num == 1 {
        1
    } else {
        recursive_factorial(num - 1) * num
    }
}

fn main() {
    println!("Color 'red' corresponds to number: {}", color_to_number("red"));
    println!("Color 'green' corresponds to number: {}", color_to_number("green"));
    println!("Color 'blue' corresponds to number: {}", color_to_number("blue"));
    println!("Color 'yellow' corresponds to number: {}", color_to_number("yellow"));

    println!("Non-recursive factorial of 5: {}", non_recursive_factorial(5));
    println!("Non-recursive factorial of 4: {}", non_recursive_factorial(4));

    println!("Recursive factorial of 5: {}", recursive_factorial(5));
    println!("Recursive factorial of 4: {}", recursive_factorial(4));
}
