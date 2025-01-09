#![allow(dead_code)]
/*
    Given a string consisting only of the characters '{', '}', '(', ')', '[', ']',
    the string is considered valid if:
    - Every opening bracket has a corresponding closing bracket of the same type.
    - The order of closing brackets matches the order of opening brackets.
    - For every closing bracket, there is a corresponding opening bracket.

    Write a function to check the validity of the given string.
*/

fn validate_paren(s: &str) -> bool {
    todo!("Validate parentheses")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(validate_paren("()"), true);
        assert_eq!(validate_paren("()[]{}"), true);
        assert_eq!(validate_paren("({[]()})"), true);
        assert_eq!(validate_paren("(}"), false);
        assert_eq!(validate_paren("()]"), false);
        assert_eq!(validate_paren("(){"), false);
    }
}
fn main() {
    validate_paren("({[]()})");
}
