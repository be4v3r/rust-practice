use std::io::{self, BufRead};
#[allow(dead_code)]
fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apple_count = apples.iter().filter(|&&d| a + d >= s && a + d <= t).count();
    let orange_count = oranges.iter().filter(|&&d| b + d >= s && b + d <= t).count();
    println!("{}", apple_count);
    println!("{}", orange_count);
}
#[allow(dead_code)]
fn count(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) -> (usize, usize) {
    let apple_count = apples.iter().filter(|&&d| a + d >= s && a + d <= t).count();
    let orange_count = oranges.iter().filter(|&&d| b + d >= s && b + d <= t).count();
    (apple_count, orange_count)
}
#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ').map(|s| s.to_string()).collect();
    let s = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let t = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let second_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ').map(|s| s.to_string()).collect();
    let a = second_multiple_input[0].trim().parse::<i32>().unwrap();
    let b = second_multiple_input[1].trim().parse::<i32>().unwrap();

    let _third_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ').map(|s| s.to_string()).collect();

    let apples: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end().split(' ').map(|s| s.parse::<i32>().unwrap()).collect();
    let oranges: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end().split(' ').map(|s| s.parse::<i32>().unwrap()).collect();

    count_apples_and_oranges(s, t, a, b, &apples, &oranges);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_0() {
        assert_eq!(count(7, 11, 5, 15, &[-2, 2, 1], &[5, -6]), (1, 1));
    }

    #[test]
    fn test_example_from_problem() {
        assert_eq!(count(7, 10, 4, 12, &[2, 3, -4], &[3, -2, -4]), (1, 2));
    }

    #[test]
    fn test_none_land_on_house() {
        assert_eq!(count(5, 10, 1, 20, &[-3, -2], &[5, 6]), (0, 0));
    }

    #[test]
    fn test_all_land_on_house() {
        assert_eq!(count(5, 10, 5, 10, &[0, 1, 2], &[-1, 0, -2]), (3, 3));
    }

    #[test]
    fn test_boundary_values() {
        assert_eq!(count(5, 10, 5, 10, &[0, 5], &[0, -5]), (2, 2));
    }
}