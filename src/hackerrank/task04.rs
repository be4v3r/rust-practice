use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {
        if x1 == x2 { "YES".to_string() } else { "NO".to_string() }
    } else if (x2 - x1) % (v1 - v2) == 0 && (x2 - x1) / (v1 - v2) > 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}
#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ').map(|s| s.to_string()).collect();
    let x1 = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let v1 = first_multiple_input[1].trim().parse::<i32>().unwrap();
    let x2 = first_multiple_input[2].trim().parse::<i32>().unwrap();
    let v2 = first_multiple_input[3].trim().parse::<i32>().unwrap();

    let result = kangaroo(x1, v1, x2, v2);
    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_0() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_sample_1() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }

    #[test]
    fn test_example_from_problem() {
        assert_eq!(kangaroo(2, 1, 1, 2), "YES");
    }

    #[test]
    fn test_same_start_same_speed() {
        assert_eq!(kangaroo(5, 3, 5, 3), "YES");
    }

    #[test]
    fn test_same_speed_different_start() {
        assert_eq!(kangaroo(0, 3, 5, 3), "NO");
    }

    #[test]
    fn test_faster_kangaroo_ahead() {
        assert_eq!(kangaroo(0, 1, 1, 2), "NO");
    }
}