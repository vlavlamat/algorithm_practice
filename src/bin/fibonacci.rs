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

fn fib_recurse(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_recurse(n - 1) + fib_recurse(n - 2),
    }
}

fn fib_iter(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_recurse() {
        assert_eq!(fib_recurse(0), 0);
        assert_eq!(fib_recurse(1), 1);
        assert_eq!(fib_recurse(2), 1);
        assert_eq!(fib_recurse(7), 13);
    }

    #[test]
    fn it_works_iter() {
        assert_eq!(fib_iter(0), 0);
        assert_eq!(fib_iter(1), 1);
        assert_eq!(fib_iter(2), 1);
        assert_eq!(fib_iter(7), 13);
    }
}
fn main() {
    fib_recurse(6);
    fib_iter(8);
}
