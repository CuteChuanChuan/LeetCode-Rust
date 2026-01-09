use super::Solution;

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = vec![];
        let mut nums = nums;
        nums.sort();

        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = nums.len() - 1;

            while l < r {
                let total = nums[i] + nums[l] + nums[r];
                if total < 0 {
                    l += 1;
                } else if total > 0 {
                    r -= 1;
                } else {
                    results.push(vec![nums[i], nums[l], nums[r]]);
                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }
                    while l < r && nums[r] == nums[r - 1] {
                        r -= 1;
                    }
                    l += 1;
                    r -= 1;
                }
            }
        }

        results
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)
