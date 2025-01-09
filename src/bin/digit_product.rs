#![allow(dead_code)]
/*
    Write a function that calculates the product of the digits of a number,
    ignoring the digit 0. Then repeat the operation with the result of the product
    until a single-digit number is obtained.
*/

fn digit_product(n: u32) -> u8 {
    todo!("Product of digits")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(digit_product(0), 0);
        assert_eq!(digit_product(9), 9);
        assert_eq!(digit_product(10), 1);
        assert_eq!(digit_product(987), 2); // 9*8*7=504, 5*4=20, 2
        assert_eq!(digit_product(123456), 4); // 1*2*3*4*5*6=720, 7*2=14, 1*4=4
        assert_eq!(digit_product(123454321), 6); // 1*2*3*4*5*4*3*2*1=2880, 2*8*8=128, 1*2*8=16, 1*6=6
    }
}

fn main() {
    digit_product(123456);
}
