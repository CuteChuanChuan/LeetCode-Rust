use super::Solution;

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
use std::collections::HashMap;

impl Solution {
    // Top-down; Time Complexity: O(n); Space Complexity: O(n)
    pub fn climb_stairs(n: i32) -> i32 {
        fn dp(i: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            if i <= 1 {
                return 1;
            }
            if let Some(&v) = memo.get(&i) {
                return v;
            }
            let result = dp(i - 1, memo) + dp(i - 2, memo);
            memo.insert(i, result);
            result
        }

        let mut memo: HashMap<i32, i32> = HashMap::new();
        dp(n, &mut memo)
    }

    // // Bottom-up; Time Complexity: O(n); Space Complexity: O(1)
    // if n < 2 { return 1}
    // else {
    //     let mut prev_1 = 1;
    //     let mut prev_2 = 1;
    //     for i in 2..=n {
    //         let current = prev_1 + prev_2;
    //         prev_2 = prev_1;
    //         prev_1 = current;
    //     }
    //     prev_1
    // }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)
