//
// 10. 正则表达式匹配
//
// 难度：困难
//
// 给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则表达式匹配。
//
// '.' 匹配任意单个字符
// '*' 匹配零个或多个前面的那一个元素
//
// 所谓匹配，是要涵盖 整个 字符串 s的，而不是部分字符串。
//
// 说明:
//
//     s 可能为空，且只包含从 a-z 的小写字母。
//     p 可能为空，且只包含从 a-z 的小写字母，以及字符 . 和 *。
//
// 示例 1:
//
// 输入:
// s = "aa"
// p = "a"
// 输出: false
// 解释: "a" 无法匹配 "aa" 整个字符串。
//
// 示例 2:
//
// 输入:
// s = "aa"
// p = "a*"
// 输出: true
// 解释: 因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。
//
// 示例 3:
//
// 输入:
// s = "ab"
// p = ".*"
// 输出: true
// 解释: ".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。
//
// 示例 4:
//
// 输入:
// s = "aab"
// p = "c*a*b"
// 输出: true
// 解释: 因为 '*' 表示零个或多个，这里 'c' 为 0 个, 'a' 被重复一次。因此可以匹配字符串 "aab"。
//
// 示例 5:
//
// 输入:
// s = "mississippi"
// p = "mis*is*p*."
// 输出: false
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/regular-expression-matching
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
//

use std::cmp::{max,min};

const SOLUTION_NUMBER: usize = 10;

pub struct Solution;
impl Solution {

    /*
     * 据说可以用动态规划，那我就试试，横轴为字符串位置，纵轴为规律位置；
     *
     *
     */
    pub fn is_match(s: String, p: String) -> bool {
        todo!("What is that");
    }
}
///
/// 1 3 5
/// 2 4 6
/// -> 3.5

pub fn test() {
    let mut all_true = true;
    {
        let case_a = String::from("aa");
        let case_a_print = format!("{:?}", case_a);
        let case_b = String::from("a");
        let case_b_print = format!("{:?}", case_b);
        let resp = Solution::is_match(case_a, case_b);
        let expe = false;
        let success = resp == expe;
        if !success {
            all_true = false;
        }
        println!(
            "[1] case_a: {0}, case_b: {1}, resp: {2}, expect: {3}, success: {4}",
            case_a_print, case_b_print, resp, expe, success
        );
    }
    {
        let case_a = String::from("aa");
        let case_a_print = format!("{:?}", case_a);
        let case_b = String::from("a*");
        let case_b_print = format!("{:?}", case_b);
        let resp = Solution::is_match(case_a, case_b);
        let expe = true;
        let success = resp == expe;
        if !success {
            all_true = false;
        }
        println!(
            "[1] case_a: {0}, case_b: {1}, resp: {2}, expect: {3}, success: {4}",
            case_a_print, case_b_print, resp, expe, success
        );
    }
    {
        let case_a = String::from("aba");
        let case_a_print = format!("{:?}", case_a);
        let case_b = String::from(".*");
        let case_b_print = format!("{:?}", case_b);
        let resp = Solution::is_match(case_a, case_b);
        let expe = true;
        let success = resp == expe;
        if !success {
            all_true = false;
        }
        println!(
            "[1] case_a: {0}, case_b: {1}, resp: {2}, expect: {3}, success: {4}",
            case_a_print, case_b_print, resp, expe, success
        );
    }
    {
        let case_a = String::from("aab");
        let case_a_print = format!("{:?}", case_a);
        let case_b = String::from("c*a*b");
        let case_b_print = format!("{:?}", case_b);
        let resp = Solution::is_match(case_a, case_b);
        let expe = true;
        let success = resp == expe;
        if !success {
            all_true = false;
        }
        println!(
            "[1] case_a: {0}, case_b: {1}, resp: {2}, expect: {3}, success: {4}",
            case_a_print, case_b_print, resp, expe, success
        );
    }
    {
        let case_a = String::from("mississippi");
        let case_a_print = format!("{:?}", case_a);
        let case_b = String::from("mis*is*p*.");
        let case_b_print = format!("{:?}", case_b);
        let resp = Solution::is_match(case_a, case_b);
        let expe = false;
        let success = resp == expe;
        if !success {
            all_true = false;
        }
        println!(
            "[1] case_a: {0}, case_b: {1}, resp: {2}, expect: {3}, success: {4}",
            case_a_print, case_b_print, resp, expe, success
        );
    }
    match all_true {
        true => println!("solution{0} success", SOLUTION_NUMBER),
        false => println!("solution{0} failed", SOLUTION_NUMBER),
    }
}
