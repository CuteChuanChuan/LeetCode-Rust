use super::Solution;

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums_set: HashSet<i32> = nums.into_iter().collect();
        let mut max_length = 0;

        for &num in &nums_set {
            if !nums_set.contains(&(num - 1)) {
                let mut current_length = 1;
                while nums_set.contains(&(num + current_length)) {
                    current_length += 1;
                }

                max_length = max_length.max(current_length)
            }
        }

        max_length
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)