
//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::collections::HashMap;
        if s.len() != t.len() { return false;}

        let mut count: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *count.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            let e = count.entry(c).or_insert(0);
            *e -= 1;
            if *e < 0 { return false;}
        }

        true

        // use std::iter::zip;
        // if s.len() != t.len() { return false;}
        // let mut count = [0i32; 26];
        //
        // for (c1, c2) in zip(s.chars(), t.chars()) {
        //     count[(c1 as usize - 'a' as usize)] += 1;
        //     count[(c2 as usize - 'a' as usize)] -= 1;
        // }
        //
        // count.into_iter().all(|x| x == 0)
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)