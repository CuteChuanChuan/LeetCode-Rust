

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new();

        for (idx, &num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&j) = map.get(&complement) {
                return vec![j, idx as i32];
            }

            map.insert(num, idx as i32);
        }

        vec![]
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)