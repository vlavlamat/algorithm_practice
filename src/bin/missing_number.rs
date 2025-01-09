#![allow(dead_code)]
/*
    Given an array containing n unique numbers in the range from 0 to n inclusive.

    Write a function that returns the single number missing
    from the array.

    It is guaranteed that the numbers in the array are unique and all belong
    to the specified range.
*/

fn missing_num(nums: &[i32]) -> i32 {
    todo!("Missing Number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(missing_num(&[1, 2]), 0);
        assert_eq!(missing_num(&[1, 0, 4, 2]), 3);
        assert_eq!(missing_num(&[0, 4, 2, 5, 3, 6]), 1);
    }
}

fn main() {
    missing_num(&[1, 2]);
}
