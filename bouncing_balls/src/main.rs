/**
 * Source: Codewars - "Bouncing Balls"
 * Link: https://www.codewars.com/kata/5544c7a5cb454edb3c000047/train/rust
 * Author: g694
 * 
 * This implementation is my own solution to the challenge.
 * 
 * Challenge:
 * A child is playing with a ball on the nth floor of a tall building. 
 * The height of this floor above ground level, h, is known.
 * 
 * He drops the ball out of the window. 
 * The ball bounces (for example), 
 * to two-thirds of its height (a bounce of 0.66).
 * 
 * His mother looks out of a window 1.5 meters from the ground.
 * 
 * How many times will the mother see the ball 
 * pass in front of her window 
 * (including when it's falling and bouncing)?
 * 
 * Three conditions must be met for a valid experiment:
 * - Float parameter "h" in meters must be greater than 0
 * - Float parameter "bounce" must be greater than 0 and less than 1
 * - Float parameter "window" must be less than h.
 * 
 * If all three conditions above are fulfilled, 
 * return a positive integer, otherwise return -1.
 * 
 * Note:
 * The ball can only be seen if the height of the 
 * rebounding ball is strictly greater than the window parameter.
 */

/**
 * Math explain
 * 
 * The condition:
 * h * b^n > w
 * 
 * Total = 1 + 2N
 * - Where initially we set the number of bounce count to 1 since the ball bounces
 * - N is the number of sightings
 * - 2N valid bounce == 2 sightings (one going up, one going down).
 */

fn bouncing_ball (h: f64, bounce: f64, window: f64) -> i32 {
    if h <= 0.0 || bounce <= 0.0 || bounce >= 1.0 || window >= h {
        return -1;
    }

    let mut bounce_count = 1;
    let mut height = h * bounce;

    while height > window {
        bounce_count += 2;
        height *= bounce;
    }

    bounce_count
}

fn main() {
    println!("{}", bouncing_ball(3.0, 0.66, 1.5));
    println!("{}", bouncing_ball(30.0, 0.66, 1.5));
    println!("{}", bouncing_ball(40.0, 0.4, 10.0));
}

#[cfg(test)]
mod tests {
    use crate::bouncing_ball;
    
    fn test_equal(h: f64, bounce: f64, window: f64, expected: i32) -> () {
        assert_eq!(bouncing_ball(h, bounce, window), expected);
    }

    #[test]
    fn test_bouncing_ball() {
        test_equal(3.0, 0.66, 1.5, 3);
        test_equal(30.0, 0.66, 1.5, 15);
        test_equal(40.0, 0.4, 10.0, 3);
        test_equal(10.0, 0.6, 10.0, -1);
    }
}