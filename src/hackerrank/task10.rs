fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let max = candles.iter().max().unwrap();
    candles.iter().filter(|&c| c == max).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_0() {
        let candles = vec![3, 2, 1, 3];
        assert_eq!(birthday_cake_candles(&candles), 2);
    }
}