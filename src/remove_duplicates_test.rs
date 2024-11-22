#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;
    use num_complex::Complex;

    #[test]
    fn test_remove_duplicates() {
        // Test case 1: Array with no duplicates
        let mut nums1 = vec![1, 2, 3, 4, 5];
        assert_eq!(remove_duplicates(&mut nums1), vec![1, 2, 3, 4, 5]);

        // Test case 2: Array with some duplicates, but not exceeding twice
        let mut nums2 = vec![1, 1, 2, 2, 3];
        assert_eq!(remove_duplicates(&mut nums2), vec![1, 1, 2, 2, 3]);

        // Test case 3: Array with duplicates exceeding twice
        let mut nums3 = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(remove_duplicates(&mut nums3), vec![1, 1, 2, 2, 3]);

        // Test case 4: Array with all elements the same
        let mut nums4 = vec![1, 1, 1, 1, 1];
        assert_eq!(remove_duplicates(&mut nums4), vec![1, 1]);

        // Test case 5: Empty array
        let mut nums5: Vec<i32> = vec![];
        assert_eq!(remove_duplicates(&mut nums5), vec![]);

        // Test case 6: Array with one element
        let mut nums6 = vec![1];
        assert_eq!(remove_duplicates(&mut nums6), vec![1]);

        // Test case 7: Array with two elements
        let mut nums7 = vec![1, 1];
        assert_eq!(remove_duplicates(&mut nums7), vec![1, 1]);

        // Test case 8: Array with multiple duplicates exceeding twice
        let mut nums8 = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(remove_duplicates(&mut nums8), vec![0, 0, 1, 1, 2, 3, 3]);

    }

    #[test]
    fn test_remove_duplicates_float() {
        // 测试用例1：没有重复的浮点数数组
        let mut nums1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(remove_duplicates(&mut nums1), vec![1.0, 2.0, 3.0, 4.0, 5.0]);

        // 测用例2：有一些重复，但不超过两次的浮点数数组
        let mut nums2 = vec![1.1, 1.1, 2.2, 2.2, 3.3];
        assert_eq!(remove_duplicates(&mut nums2), vec![1.1, 1.1, 2.2, 2.2, 3.3]);

        // 测试用例3：有超过两次重复的浮点数数组
        let mut nums3 = vec![1.5, 1.5, 1.5, 2.5, 2.5, 3.5];
        assert_eq!(remove_duplicates(&mut nums3), vec![1.5, 1.5, 2.5, 2.5, 3.5]);

        // 测试用例4：所有素都相同的浮点数数组
        let mut nums4 = vec![1.0, 1.0, 1.0, 1.0, 1.0];
        assert_eq!(remove_duplicates(&mut nums4), vec![1.0, 1.0]);

        // 测试用例5：空的浮点数数组
        let mut nums5: Vec<f64> = vec![];
        assert_eq!(remove_duplicates(&mut nums5), vec![]);

        // 测试用例6：只有一个元素的浮点数数组
        let mut nums6 = vec![1.5];
        assert_eq!(remove_duplicates(&mut nums6), vec![1.5]);

        // 测试用例7：有两个元素的浮点数数组
        let mut nums7 = vec![1.5, 1.5];
        assert_eq!(remove_duplicates(&mut nums7), vec![1.5, 1.5]);

        // 测试用例8：有多个超过两次重复的浮点数数组
        let mut nums8 = vec![0.0, 0.0, 1.1, 1.1, 1.1, 1.1, 2.2, 3.3, 3.3];
        assert_eq!(remove_duplicates(&mut nums8), vec![0.0, 0.0, 1.1, 1.1, 2.2, 3.3, 3.3]);
    }

    #[test]
    fn test_remove_duplicates_complex() {
        // 测试用例1：没有重复的复数数组
        let mut nums1 = vec![
            Complex::new(1.0, 2.0),
            Complex::new(1.0, 2.0),
            Complex::new(1.0, 2.0),
            Complex::new(2.0, 3.0),
            Complex::new(3.0, 4.0),
            Complex::new(3.0, 4.0),
        ];
        assert_eq!(
            remove_duplicates(&mut nums1),
            vec![
                Complex::new(1.0, 2.0),
                Complex::new(1.0, 2.0),
                Complex::new(2.0, 3.0),
                Complex::new(3.0, 4.0),
                Complex::new(3.0, 4.0),
            ]
        );
    }

    #[test]
    fn test_remove_duplicates_bigint() {
        // 测试用例1：没有重复的大整数数组
        let mut nums1 = vec![
            BigInt::from(1),
            BigInt::from(1),
            BigInt::from(1),
            BigInt::from(2),
            BigInt::from(3),
            BigInt::from(3),
        ];
        assert_eq!(
            remove_duplicates(&mut nums1),
            vec![
                BigInt::from(1),
                BigInt::from(1),
                BigInt::from(2),
                BigInt::from(3),
                BigInt::from(3),
            ]
        );
    }


}