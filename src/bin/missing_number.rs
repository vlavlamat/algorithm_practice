#![allow(dead_code)]
/*
    Given an array containing n unique numbers in the range from 0 to n inclusive.

    Write a function that returns the single number missing
    from the array.

    It is guaranteed that the numbers in the array are unique and all belong
    to the specified range.
*/

/// Though the code was initially written independently, its current optimized form was guided by suggestions from ChatGPT.
fn missing_num(nums: &[i32]) -> i32 {
    // Calculate the sum of all numbers in the input array.
    let sum: i32 = nums.iter().sum();

    // Calculate the expected sum of all numbers from 0 to n (inclusive).
    // This range is one number longer than the input array to account for the missing number.
    let expected_sum = (0..=nums.len() as i32).sum::<i32>();

    // The difference between the expected sum and the actual sum gives the missing number.
    expected_sum - sum
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(missing_num(&[1, 2]), 0);
        assert_eq!(missing_num(&[0, 1, 3]), 2);
        assert_eq!(missing_num(&[1, 0, 4, 2]), 3);
        assert_eq!(missing_num(&[0, 1, 2, 4, 5]), 3);
        assert_eq!(missing_num(&[1, 2, 3, 4, 5]), 0);
        assert_eq!(missing_num(&[0, 4, 2, 5, 3, 6]), 1);
    }
}

fn main() {
    println!("Hello, missing_number!");
}
