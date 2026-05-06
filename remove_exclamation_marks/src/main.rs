/**
 * Source: Codewars - "Remove Exclamation Marks"
 * Link: https://www.codewars.com/kata/57a0885cbb9944e24c00008e/train/rust
 * Author: wichu
 * 
 * This implementation is my own solution to the challenge.
 * 
 * Challenge:
 * Write function RemoveExclamationMarks 
 * which removes all exclamation marks from a given string.
 */

fn remove_exclamation_marks(input: &str) -> String {
    input.to_string()
        .chars()
        .filter(|&c| c != '!')
        .collect()
}

fn main() {
    println!("{}", remove_exclamation_marks("Hello World!"));
    println!("{}", remove_exclamation_marks("Hello World!!!"));
}

#[cfg(test)]
mod tests {
    use crate::remove_exclamation_marks;

    fn do_test(input: &str, expected: &str) {
        let actual = remove_exclamation_marks(input);
        assert_eq!(actual, expected, "\nYour result (left) did not match the expected output (right) for the input: {input:?}");
    }

    #[test]
    fn sample_test() {
        do_test("Hello World!", "Hello World");
        do_test("Hello World!!!", "Hello World");
        do_test("Hi! Hello!", "Hi Hello");
        do_test("", "");
        do_test("Oh, no!!!", "Oh, no");
    }
}