/**
 * Source: Codewars - "So Many Permutations!"
 * Link: https://www.codewars.com/kata/5254ca2719453dcc0b00027d/train/rust
 * Author: BattleRattle
 * 
 * Challenge:
 * Your task is to create all permutations of a non-empty input string and remove duplicates, if present.
 * Create as many "shufflings" as you can!
 * 
 * Examples:
 * With input 'a':
 * Your function should return: ['a']
 * 
 * With input 'ab':
 * Your function should return ['ab', 'ba']
 * 
 * With input 'abc':
 * Your function should return ['abc','acb','bac','bca','cab','cba']
 * 
 * With input 'aabb':
 * Your function should return ['aabb', 'abab', 'abba', 'baab', 'baba', 'bbaa']
 * 
 * Note: The order of the permutations doesn't matter.
 */

use itertools::Itertools;

fn permutations(s: &str) -> Vec<String> {
    s.chars()
        .permutations(s.len())
        .unique()
        .map(String::from_iter)
        .collect()
}

fn main() {
    println!("{:?}", permutations("a"));
    println!("{:?}", permutations("ab"));
    println!("{:?}", permutations("abc"));
    println!("{:?}", permutations("aabb"));
}

#[cfg(test)]
mod tests {
    use crate::permutations;

    fn assert_ordered_equals(actual: &[String], expected: &[String]) {
        let mut actual: Vec<_> = actual.to_vec();
        let mut expected: Vec<_> = expected.to_vec();
        actual.sort_unstable();
        expected.sort_unstable();

        assert_eq!(
            actual, expected,
            "\nYour result (left) did not match the expected output (right)"
        );
    }

    #[test]
    fn sample_tests() {
        let expected = vec!["a".to_string()];
        let actual = permutations("a");
        assert_ordered_equals(&actual, &expected);

        let expected = vec!["ab".to_string(), "ba".to_string()];
        let actual = permutations("ab");
        assert_ordered_equals(&actual, &expected);

        let expected = vec![
            "aabb".to_string(),
            "abab".to_string(),
            "abba".to_string(),
            "baab".to_string(),
            "baba".to_string(),
            "bbaa".to_string(),
        ];
        let actual = permutations("aabb");
        assert_ordered_equals(&actual, &expected);
    }
}