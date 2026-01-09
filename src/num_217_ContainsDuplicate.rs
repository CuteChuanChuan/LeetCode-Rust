use super::Solution;

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        nums.len() != nums.into_iter().collect::<HashSet<_>>().len()
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)
