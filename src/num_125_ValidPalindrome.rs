use super::Solution;

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let (mut l, mut r) = (0, chars.len().saturating_sub(1));

        while l < r {
            while l < r && !chars[l].is_alphanumeric() {
                l += 1;
            }
            while l < r && !chars[r].is_alphanumeric() {
                r -= 1;
            }

            if chars[l].to_ascii_lowercase() != chars[r].to_ascii_lowercase() {
                return false;
            }
            l += 1;
            r = r.saturating_sub(1);
        }

        true
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)
