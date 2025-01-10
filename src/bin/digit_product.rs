#![allow(dead_code)]
/*
    Write a function that calculates the product of the digits of a number,
    ignoring the digit 0. Then repeat the operation with the result of the product
    until a single-digit number is obtained.
*/

fn digit_product(n: u32) -> u8 {
    // Convert the number to a string for easier manipulation of digits
    let mut number_as_string = n.to_string();
    let mut product;

    // If the input number is 0, the result is immediately 0
    if n == 0 {
        return 0;
    }
    loop {
        // Reset the product to the neutral element for multiplication
        product = 1;

        // Iterate over each character in the string representation of the number
        for char in number_as_string.chars() {
            // Convert the character to a digit (safe since it's a numeric string)
            let number_from_char = char.to_digit(10).unwrap();

            // Ignore the digit 0 as it does not affect the product calculation
            if number_from_char == 0 {
                continue;
            }

            // Multiply the current product by the digit
            product *= number_from_char;
        }
        if product < 10 {
            break;
        } else {
            // Convert the product back to a string for the next iteration
            number_as_string = product.to_string();
        }
    }

    // Return the single-digit product as the final result
    product as u8
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
    println!("Hello, digit_product!");
}
