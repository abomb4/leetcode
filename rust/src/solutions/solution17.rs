//
// 17. 电话号码的字母组合
// 难度：中等
// 
// 给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。
// 
// 给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。
// 
// 示例:
// 
// 输入："23"
// 输出：["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
// 
// 说明:
// 尽管上面的答案是按字典序排列的，但是你可以任意选择答案输出的顺序。
// 
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
//

const SOLUTION_NUMBER: usize = 17;

const M2: [char; 4] = ['a', 'b', 'c', '0'];
const M3: [char; 4] = ['d', 'e', 'f', '0'];
const M4: [char; 4] = ['g', 'h', 'i', '0'];
const M5: [char; 4] = ['j', 'k', 'l', '0'];
const M6: [char; 4] = ['m', 'n', 'o', '0'];
const M7: [char; 4] = ['p', 'q', 'r', 's'];
const M8: [char; 4] = ['t', 'u', 'v', '0'];
const M9: [char; 4] = ['w', 'x', 'y', 'z'];

pub struct Solution;
impl Solution {
    fn append(chars: &Vec<char>, i: usize, s: &mut String, results: &mut Vec<String>) {
        let len = chars.len();
        let c = chars[i];
        let (arr, loop_len) = match c {
            '2' => (M2, 3),
            '3' => (M3, 3),
            '4' => (M4, 3),
            '5' => (M5, 3),
            '6' => (M6, 3),
            '7' => (M7, 4),
            '8' => (M8, 3),
            '9' => (M9, 4),
            _ => panic!("Invalid input")
        };
        for j in 0 .. loop_len {
            s.push(arr[j]);
            if i == len - 1 {
                results.push(s.clone());
            } else {
                Self::append(chars, i + 1, s, results);
            }
            s.pop();
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let chars: Vec<char> = digits.chars().collect();
        let len = chars.len();
        if len == 0 {
            return Vec::new();
        }
        let mut results = Vec::with_capacity(len * len);
        Self::append(&chars, 0, &mut String::with_capacity(len), &mut results);
        results
    }
}

pub fn test() {
    let mut all_true = true;
    {
        let case_a = String::from("23");
        let case_a_print = format!("{:?}", case_a);
        let resp = Solution::letter_combinations(case_a);
        let resp_print = format!("{:?}", resp);
        let expe = vec!("ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf");
        let expe_print = format!("{:?}", expe);
        let success = resp == expe;
        if !success { all_true = false; }
        println!("[1] case_a: {0}, resp: {1}, expect: {2}, success: {3}",
            case_a_print, resp_print, expe_print, success);
    }
    match all_true {
        true => println!("solution{0} success", SOLUTION_NUMBER),
        false => println!("solution{0} failed", SOLUTION_NUMBER),
    }
}
