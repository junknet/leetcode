use std::cmp::max;

pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums.len() as i32;
    }
    let mut max_length = 1;
    let mut current_count = 1;
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            current_count += 1;
        } else {
            current_count = 1;
        }
        max_length = max(max_length, current_count);
    }
    max_length
}
