use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
#[allow(dead_code)]
fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0i32; 6];
    for &bird in arr {
        counts[bird as usize] += 1;
    }
    let max_count = counts[1..=5].iter().max().unwrap();
    for id in 1..=5 {
        if counts[id] == *max_count {
            return id as i32;
        }
    }
    unreachable!()
}
#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();
    let result = migratory_birds(&arr);
    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_0() {
        assert_eq!(migratory_birds(&[1, 4, 4, 4, 5, 3]), 4);
    }

    #[test]
    fn test_sample_1() {
        assert_eq!(migratory_birds(&[1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4]), 3);
    }

    #[test]
    fn test_example_from_problem() {
        assert_eq!(migratory_birds(&[1, 1, 2, 2, 3]), 1);
    }

    #[test]
    fn test_single_type_dominates() {
        assert_eq!(migratory_birds(&[5, 5, 5, 1, 2, 3, 4]), 5);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(migratory_birds(&[3, 3, 3, 3, 3]), 3);
    }

    #[test]
    fn test_tie_returns_smallest() {
        assert_eq!(migratory_birds(&[5, 4, 3, 2, 1]), 1);
    }
}