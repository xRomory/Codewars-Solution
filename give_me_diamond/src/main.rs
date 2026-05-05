/**
 * Source: Codewars - "Give me a Diamond"
 * Link: https://www.codewars.com/kata/5503013e34137eeeaa001648/train/rust
 * Author: jayeshcp
 * 
 * I got this challenge in Codewars to see and test if I did the challenge correctly.
 * All copyrights to the respective authors of the challenge.
 * 
 * This implementation is my own solution to the challenge.
 * 
 * Jamie is a programmer, and James' girlfriend.
 * She likes diamonds, and wants a diamond string from James.
 * Since James doesn't know how to make this happen, he needs your help.
 * 
 * Task:
 * You need to return a string that looks like a diamond shape 
 * when printed on the screen, using asterisk (*) characters. 
 * Trailing spaces should be removed, and every line must be terminated with a newline character (\n).
 * 
 * Return `null/nil/None/...` if the input is an even number or negative,
 * as it is not possible to print a diamond of even or negative size.
 */

fn diamond (n: i32) -> Option<String> {
    if n <= 0 || n % 2 == 0 { return None; }

    let mut result = String::new();
    let n = n as usize;

    for i in (1..=n).step_by(2) {
        let spaces = (n - i) / 2;

        result.push_str(&" ".repeat(spaces));
        result.push_str(&"*".repeat(i));
        result.push('\n');
    }

    for i in (1..n).step_by(2).rev() {
        let spaces = (n - i) / 2;

        result.push_str(&" ".repeat(spaces));
        result.push_str(&"*".repeat(i));
        result.push('\n');
    }

    Some(result)
}

fn diamond_simpler(n: i32) -> Option<String> {
    if n <= 0 || n % 2 == 0 { return None };

    let n = n as usize;
    let diamond = (1..=n)
        .chain((1..n).rev())
        .step_by(2)
        .map(|i| format!("{}{}\n", " ".repeat((n - i) / 2), "*".repeat(i)))
        .collect();

    Some(diamond)
}

fn main() {
    println!("{}", diamond(5).unwrap());
    println!("{}", diamond_simpler(5).unwrap());
}