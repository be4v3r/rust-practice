use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
#[allow(dead_code)]
fn sock_merchant(_n: i32, ar: &[i32]) -> i32 {
    use std::collections::HashMap;
    
    let mut counts: HashMap<i32, i32> = HashMap::new();
    
    for &sock in ar {
        *counts.entry(sock).or_insert(0) += 1;
    }
    
    counts.values().map(|&count| count / 2).sum()
}
#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = sock_merchant(n, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(9, &ar), 3);
    }
}