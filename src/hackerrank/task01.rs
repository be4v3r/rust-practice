use std::io::{self, BufRead};

fn build_staircase(n: i32) -> String {
    let mut result = String::new();

    for i in 1..=n {
        let spaces = (n - i) as usize;
        let hashes = i as usize;

        result.push_str(&" ".repeat(spaces));
        result.push_str(&"#".repeat(hashes));

        if i != n {
            result.push('\n');
        }
    }

    result
}
#[allow(dead_code)]
fn staircase(n: i32) -> String {
    build_staircase(n)
}
#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    staircase(n);
}


#[test]
fn staircase_test() {
    let n = 6;
    let expected = "     #\n    ##\n   ###\n  ####\n #####\n######";
    assert_eq!(staircase(n), expected);
}
