use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
#[allow(dead_code)]
fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let mut count = 0;
    let max_b = *b.iter().max().unwrap();
    
    for x in 1..=max_b {
        if a.iter().all(|&ai| x % ai == 0) && b.iter().all(|&bi| bi % x == 0) {
            count += 1;
        }
    }
    
    count
}
#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();
    let _n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let _m = first_multiple_input[1].trim().parse::<i32>().unwrap();
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();
    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();
    let total = get_total_x(&arr, &brr);
    writeln!(&mut fptr, "{}", total).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        assert_eq!(get_total_x(&[2, 4], &[16, 32, 96]), 3);
    }

    #[test]
    fn test_example() {
        assert_eq!(get_total_x(&[2, 6], &[24, 36]), 2);
    }
}