# Algorithm Practice

This project is a collection of algorithmic challenges implemented in Rust. The goal is to practice solving problems and deepen understanding of fundamental programming concepts through real-world examples. Each challenge includes detailed requirements and test cases.

## Challenges Included

### [1. **Digit Multiplication Reduction**](src/bin/digit_product.rs)

Write a function that calculates the product of the digits of a number, ignoring zeros. Repeat this operation on the result until a single-digit number is obtained.

### [2. **Fibonacci Sequence**](src/bin/fibonacci.rs)

- Implement a function to calculate the Fibonacci sequence element at index `n`.
- Write a second function that returns the Fibonacci sequence from the first element up to the `n`th element.
- Include tests to validate both functions.

### [3. **FizzBuzz**](src/bin/fizzbuzz.rs)
- Write a function that converts a number into a string based on the following rules:
    1. If the number is divisible by 3, return "Fizz".
    2. If the number is divisible by 5, return "Buzz".
    3. If the number is divisible by both 3 and 5, return "FizzBuzz".
    4. Otherwise, return the string representation of the number.
- Write a function `fizzbuzz_list` that takes an input `n: u32` and returns a list of strings containing the FizzBuzz representations for numbers from 1 to `n`.
- Include tests to validate the results.

### [4. **Missing Number**](src/bin/missing_number.rs)
Write a function that finds the single missing number in an array containing `n` unique numbers from the range `0` to `n`. The array will not have duplicates, and all numbers belong to the specified range.

### [5. **Unique Digit Finder**](src/bin/unique_digit.rs)
- Write a function to find the unique digit in a string containing only numeric characters.
- Write a similar function for strings that may contain any characters. In this case, the unique digit may not exist, but if it does, there is at most one. Include tests for both functions.

### [6. **Valid Parentheses**](src/bin/validate_parentheses.rs)
Write a function to determine whether a string of parentheses is valid. A string is valid if:
- Every opening bracket has a corresponding closing bracket of the same type.
- Brackets are closed in the correct order.
- For every closing bracket, there is a corresponding opening bracket.

The string will only contain the characters `{`, `}`, `(`, `)`, `[`, and `]`.

## Project Structure
- **src/bin**: Contains all Rust implementation files for the challenges.
- **README.md**: This file, describing the project and its goals.

## Requirements
- Rust (latest stable version)

## How to Run
1. Clone the repository.
2. Navigate to the project directory.
3. Build and run the project using Cargo:
   ```bash
   cargo build
   cargo run
   ```
4. Run tests to ensure correctness:
   ```bash
   cargo test
   ```

## Contribution
Feel free to contribute by adding more challenges, improving solutions, or enhancing test coverage. Submit a pull request with your proposed changes.
