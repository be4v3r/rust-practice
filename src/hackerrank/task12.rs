fn bon_appetit(bill: &[i32], k: i32, b: i32) -> String {
    let actual: i32 = bill.iter()
        .enumerate()
        .filter(|(i, _)| *i != k as usize)
        .map(|(_, &cost)| cost)
        .sum::<i32>() / 2;

    if actual == b {
        "Bon Appetit".to_string()
    } else {
        (b - actual).to_string()
    }
}

#[cfg(test)]
    #[test]
    fn test_sample_0_overcharged() {
        // Anna didn't eat bill[1]=10, shared = 3+2+9=14, actual=7, charged=12 → refund 5
        let bill = vec![3, 10, 2, 9];
        assert_eq!(bon_appetit(&bill, 1, 12), "5");
    }

    #[test]
    fn test_sample_1_fair() {
        // Anna didn't eat bill[1]=10, shared = 3+2+9=14, actual=7, charged=7 → Bon Appetit
        let bill = vec![3, 10, 2, 9];
        assert_eq!(bon_appetit(&bill, 1, 7), "Bon Appetit");
    }

    #[test]
    fn test_skip_first_item() {
        // Skip bill[0]=6, shared = 4+8=12, actual=6, charged=6 → Bon Appetit
        let bill = vec![6, 4, 8];
        assert_eq!(bon_appetit(&bill, 0, 6), "Bon Appetit");
    }

    #[test]
    fn test_skip_last_item() {
        // Skip bill[2]=10, shared = 4+6=10, actual=5, charged=8 → refund 3
        let bill = vec![4, 6, 10];
        assert_eq!(bon_appetit(&bill, 2, 8), "3");
    }

    #[test]
    fn test_two_items_skip_one() {
        // Skip bill[0]=100, shared = 20, actual=10, charged=10 → Bon Appetit
        let bill = vec![100, 20];
        assert_eq!(bon_appetit(&bill, 0, 10), "Bon Appetit");
    }

    #[test]
    fn test_zero_cost_items() {
        // Skip bill[1]=0, shared = 0+0=0, actual=0, charged=0 → Bon Appetit
        let bill = vec![0, 0, 0];
        assert_eq!(bon_appetit(&bill, 1, 0), "Bon Appetit");
    }