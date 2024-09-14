use num_traits::Num;

// Function to remove duplicates from a sorted array, allowing each element to appear at most twice
pub fn remove_duplicates<T>(nums: &mut Vec<T>) -> Vec<T>
where
    T: std::cmp::PartialEq + std::cmp::PartialOrd + Copy + Num,
{
    // If the array length is less than or equal to 2, return the original array
    if nums.len() <= 2 {
        return nums.to_vec();
    }

    // Initialize a pointer for placing the next valid element
    let mut k = 2;

    // Iterate through the array starting from the third element
    for i in 2..nums.len() {
        // If the current element is different from the element at k-2 position, it's a valid element
        if nums[i] != nums[k - 2] {
            // Place the current element at position k
            nums[k] = nums[i];
            // Move the pointer to the next position
            k += 1;
        }
    }

    // Return the deduplicated array
    nums[0..k].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}