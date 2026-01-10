use super::Solution;

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn two_sum_1(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l, mut r) = (0, numbers.len().saturating_sub(1));

        while l < r {
            let sums = numbers[l] + numbers[r];
            if sums == target {
                return vec![l as i32 + 1, r as i32 + 1]
            } else if sums < target {
                l += 1;
            } else {
                r = r.saturating_sub(1);
            }

        }
        vec![]
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)
