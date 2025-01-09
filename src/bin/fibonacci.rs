#![allow(dead_code)]
/*
    The Fibonacci sequence is a series of numbers that satisfies the following conditions:
    - The element at index 0 is the number 0.
    - The element at index 1 is the number 1.
    - Each subsequent element is the sum of the two previous elements.

    0, 1, 1, 2, 3, 5, 8, 13, 21 ...

    Write a function that calculates the element of the sequence at index n.

    * Write a second function that returns the Fibonacci sequence
      from the first element up to the nth element. Write tests.
*/

fn fib(n: u32) -> u32 {
    todo!("Fibonacci")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(7), 13);
    }
}
fn main() {
    fib(5);
}