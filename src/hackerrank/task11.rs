#[allow(dead_code)]
fn divisible_sum_pairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;
    for i in 0..n as usize {
        for j in (i + 1)..n as usize {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_from_problem() {
        // From the problem images: ar = [1,3,2,6,1,2], k = 3 → 5 pairs
        let ar = vec![1, 3, 2, 6, 1, 2];
        assert_eq!(divisible_sum_pairs(6, 3, &ar), 5);
    }

    #[test]
    fn test_example_from_problem() {
        // From the example: ar = [1,2,3,4,5,6], k = 5 → 3 pairs
        let ar = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(divisible_sum_pairs(6, 5, &ar), 3);
    }

    #[test]
    fn test_all_same_divisible() {
        // ar = [3,3,3], k = 3 → all 3 pairs sum to 6, divisible by 3
        let ar = vec![3, 3, 3];
        assert_eq!(divisible_sum_pairs(3, 3, &ar), 3);
    }

    #[test]
    fn test_no_valid_pairs() {
        // ar = [1,1,1], k = 3 → sums are all 2, not divisible by 3
        let ar = vec![1, 1, 1];
        assert_eq!(divisible_sum_pairs(3, 3, &ar), 0);
    }

    #[test]
    fn test_k_equals_1() {
        // k = 1 → every sum is divisible, n=4 → 4*3/2 = 6 pairs
        let ar = vec![1, 2, 3, 4];
        assert_eq!(divisible_sum_pairs(4, 1, &ar), 6);
    }
}