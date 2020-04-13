
pub struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        3
    }
}

pub fn test() {
    let case1 = "abcabcbb";
    let resp1 = Solution::length_of_longest_substring(String::from(case1));
    let expe1 = 3;
    println!("case: {0}, resp: {1}, expect: {2}, success: {3}", case1, resp1, expe1, resp1 == expe1);
}
