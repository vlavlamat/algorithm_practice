#![allow(dead_code)]
/*
    Write a function that converts a number into a string based on the following rules:
    1. If the number is divisible by 3, return the string "Fizz".
    2. If the number is divisible by 5, return the string "Buzz".
    3. If the number is divisible by both 3 and 5, return the string "FizzBuzz".
    4. Otherwise, return a string containing the given number.

    Write a function `fizzbuzz_list` that takes a number `n: u32` and returns
    a list of strings representing the FizzBuzz values
    for numbers in the range from 1 to n. Write tests.
*/

fn fizzbuzz(num: u32) -> String {
    match (num % 3 == 0, num % 5 == 0) {
        (true, true) => "FizzBuzz".into(),
        (true, false) => "Fizz".into(),
        (false, true) => "Buzz".into(),
        _ => num.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(&fizzbuzz(1), "1");
        assert_eq!(&fizzbuzz(3), "Fizz");
        assert_eq!(&fizzbuzz(5), "Buzz");
        assert_eq!(&fizzbuzz(7), "7");
        assert_eq!(&fizzbuzz(9), "Fizz");
        assert_eq!(&fizzbuzz(15), "FizzBuzz");
        assert_eq!(&fizzbuzz(30), "FizzBuzz");
        assert_eq!(&fizzbuzz(49), "49");
    }
}
fn main() {
    println!("Hello, FizzBuzz!");
}
