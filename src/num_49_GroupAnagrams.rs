use super::Solution;

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for str in strs {
            let mut char_freq = [0u8; 26];
            for c in str.bytes() {
                char_freq[(c as u8 - b'a') as usize] += 1;
            }
            map.entry(char_freq).or_default().push(str);
        }

        map.into_values().collect()

        // use std::collections::HashMap;
        // use itertools::Itertools;
        //
        // let mut map: HashMap<String, Vec<String>> = HashMap::new();
        // strs.into_iter().for_each(|s| {
        //     let k = s.chars().sorted().collect::<String>();
        //     map.entry(k).or_default().push(s);
        // });
        //
        // map.into_values().collect()
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)
