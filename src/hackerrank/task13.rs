fn page_count(n: i32, p: i32) -> i32 {
    let from_front = p / 2;
    let from_back = n / 2 - p / 2;
    from_front.min(from_back)
}

#[cfg(test)]
#[test]
fn test_sample_0() {
    // n=6, p=2 → from front=1, from back=2 → min=1
    assert_eq!(page_count(6, 2), 1);
}

#[test]
fn test_sample_1() {
    // n=5, p=4 → from front=2, from back=0 → min=0
    assert_eq!(page_count(5, 4), 0);
}

#[test]
fn test_first_page() {
    // p=1 is always on the right at open, 0 turns needed
    assert_eq!(page_count(10, 1), 0);
}

#[test]
fn test_last_page_even() {
    // n=6, p=6 → from front=3, from back=0 → min=0
    assert_eq!(page_count(6, 6), 0);
}

#[test]
fn test_last_page_odd() {
    // n=5, p=5 → from front=2, from back=0 → min=0
    assert_eq!(page_count(5, 5), 0);
}

#[test]
fn test_middle_page() {
    // n=5, p=3 → from front=1, from back=1 → min=1
    assert_eq!(page_count(5, 3), 1);
}