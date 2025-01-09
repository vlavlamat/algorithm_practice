#![allow(dead_code)]
/*
    Given a string consisting only of numeric characters, there is one digit
    in the string that does not repeat. Write a function to find this digit
    and return it.

    * Write a similar function, but this time the string may contain any characters,
    and the unique digit may be absent. If it exists, there will be at most one.
    Write tests.
*/

fn uniq_digit(s: &str) -> u8 {
    todo!("Unique number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(uniq_digit("3"), 3);
        assert_eq!(uniq_digit("010"), 1);
        assert_eq!(uniq_digit("47343077"), 0);
        assert_eq!(uniq_digit("123454321"), 5);
        assert_eq!(uniq_digit("0987654321234567890"), 1);
        assert_eq!(uniq_digit("4444444444424444444444444"), 2);
    }
}
fn main() {
    uniq_digit("123454321");
}
