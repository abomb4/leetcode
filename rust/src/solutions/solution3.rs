use std::cmp::max;

pub struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let len: usize = s.len();
        let mut map_like: [i32; 256] = [-1; 256];

        let char_array = s.into_bytes();

        let mut max_length = 0;
        let mut index_start: usize = 0;
        for (i, char) in char_array.iter().enumerate() {
            let previous: &i32 = map_like.get(*char as usize).expect("DIE");
            if *previous > -1 && *previous >= index_start as i32 {
                max_length = max(max_length, i - index_start);
                index_start = (*previous + 1) as usize;
            }
            map_like[*char as usize] = i as i32;
        }

        max(max_length, len - index_start) as i32
    }
}

pub fn test() {
    let mut all_true = true;
    {
        let case = "abcabcbb";
        let resp = Solution::length_of_longest_substring(String::from(case));
        let expe = 3;
        let success = resp == expe;
        if !success { all_true = false; }
        println!("[1] case: {0}, resp: {1}, expect: {2}, success: {3}", case, resp, expe, success);
    }
    {
        let case = "bbbbbbbb";
        let resp = Solution::length_of_longest_substring(String::from(case));
        let expe = 1;
        let success = resp == expe;
        if !success { all_true = false; }
        println!("[2] case: {0}, resp: {1}, expect: {2}, success: {3}", case, resp, expe, success);
    }
    {
        let case = "pwwkew";
        let resp = Solution::length_of_longest_substring(String::from(case));
        let expe = 3;
        let success = resp == expe;
        if !success { all_true = false; }
        println!("[3] case: {0}, resp: {1}, expect: {2}, success: {3}", case, resp, expe, success);
    }
    match all_true {
        true => println!("solution3 success"),
        false => println!("solution3 failed"),
    }
}
