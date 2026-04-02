use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
#[allow(dead_code)]
fn breaking_records(scores: &[i32]) -> Vec<i32> {
    if scores.is_empty() {
        return vec![0, 0];
    }
    
    let mut max = scores[0];
    let mut min = scores[0];
    let mut max_count = 0;
    let mut min_count = 0;
    
    for &score in &scores[1..] {
        if score > max {
            max = score;
            max_count += 1;
        } else if score < min {
            min = score;
            min_count += 1;
        }
    }
    
    vec![max_count, min_count]
}
#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = breaking_records(&scores);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();
        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }
    writeln!(&mut fptr).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_0() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        assert_eq!(breaking_records(&scores), vec![2, 4]);
    }

    #[test]
    fn test_sample_1() {
        let scores = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
        assert_eq!(breaking_records(&scores), vec![4, 0]);
    }

    #[test]
    fn test_single_game() {
        let scores = vec![42];
        assert_eq!(breaking_records(&scores), vec![0, 0]);
    }

    #[test]
    fn test_empty() {
        let scores: Vec<i32> = vec![];
        assert_eq!(breaking_records(&scores), vec![0, 0]);
    }

    #[test]
    fn test_all_same() {
        let scores = vec![5, 5, 5, 5, 5];
        assert_eq!(breaking_records(&scores), vec![0, 0]);
    }

    #[test]
    fn test_strictly_increasing() {
        let scores = vec![1, 2, 3, 4, 5];
        assert_eq!(breaking_records(&scores), vec![4, 0]);
    }

    #[test]
    fn test_strictly_decreasing() {
        let scores = vec![5, 4, 3, 2, 1];
        assert_eq!(breaking_records(&scores), vec![0, 4]);
    }

    #[test]
    fn test_alternating() {
        let scores = vec![10, 20, 5, 30, 1];
        assert_eq!(breaking_records(&scores), vec![2, 2]);
    }

    #[test]
    fn test_ties_dont_count() {
        let scores = vec![12, 24, 10, 24];
        assert_eq!(breaking_records(&scores), vec![1, 1]);
    }

    #[test]
    fn test_large_values() {
        let scores = vec![0, 100_000_000, 0, 100_000_000];
        assert_eq!(breaking_records(&scores), vec![1, 0]); // min tie at game 2, max tie at game 3
}

    #[test]
    fn test_zero_start() {
        let scores = vec![0, 5, 10, 3];
        assert_eq!(breaking_records(&scores), vec![2, 0]);
    }
}