use super::Solution;

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut freq_map = HashMap::new();

        for &num in &nums {
            *freq_map.entry(num).or_insert(0) += 1;
        }

        let n = nums.len();
        let mut buckets: Vec<Vec<i32>> = vec![vec![]; n + 1];
        for (num, freq) in freq_map {
            buckets[freq].push(num);
        }

        for bucket in (0..=n).rev() {
            result.extend(buckets[bucket].iter());

            if result.len() == k as usize {
                break;
            }
        }
        result

        // let mut freq_map = HashMap::new();
        // for num in nums {
        //     *freq_map.entry(num).or_insert(0) += 1;
        // }
        //
        // let k = k as usize;
        // let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        //
        // for (num, freq) in freq_map {
        //     heap.push(Reverse((freq, num)));
        //     if heap.len() > k {
        //         heap.pop();
        //     }
        // }
        //
        // heap.into_iter().map(|Reverse((_, num))| num).collect()
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)
