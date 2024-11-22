use num_traits::Num;
use num_complex::Complex;

// Function to remove duplicates from a sorted array, allowing each element to appear at most twice
pub fn remove_duplicates<T>(nums: &mut Vec<T>) -> Vec<T>
where
    T: std::cmp::PartialEq + Clone + Num,
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
            nums[k] = nums[i].clone();
            // Move the pointer to the next position
            k += 1;
        }
    }

    // Return the deduplicated array
    nums[0..k].to_vec()
}