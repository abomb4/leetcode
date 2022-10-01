//
// 4. 寻找两个正序数组的中位数
//
// 难度：困难
//
// 给定两个大小为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。
//
// 请你找出这两个正序数组的中位数，并且要求算法的时间复杂度为 O(log(m + n))。
//
// 你可以假设 nums1 和 nums2 不会同时为空。
//
//
//
// 示例 1:
//
// ums1 = [1, 3]
// ums2 = [2]
//
// 则中位数是 2.0
//
// 示例 2:
//
// ums1 = [1, 2]
// ums2 = [3, 4]
//
// 则中位数是 (2 + 3)/2 = 2.5
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/median-of-two-sorted-arrays
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
//

use std::cmp::min;

const SOLUTION_NUMBER: usize = 4;

pub struct Solution;
impl Solution {

    /*
     * 直接看答案的解法：
     *
     * 问题化为‘寻找两个数组中第 k 小的数字’，k = (len1 + len2) / 2；
     * 两数组中寻找，每次寻找第 k / 2 小的数字，就是在两个数组中找第 k / 2 个元素，对他俩进行比较；
     * 若某个数字比较小，说明它及它前面的所有数字都不可能是第 k 小的数字，都排除。
     */

    pub fn find_kth_num(nums1: &Vec<i32>, left_1: usize, nums2: &Vec<i32>, left_2: usize, k: usize) -> i32 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let len_left_1 = len1 - left_1;
        let len_left_2 = len2 - left_2;
        if len_left_1 > len_left_2 {
            return Self::find_kth_num(nums2, left_2, nums1, left_1, k);
        }

        if len_left_1 == 0 {
            return nums2[k - 1 + left_2];
        }
        if k == 1 {
            return min(nums1[left_1], nums2[left_2]);
        }

        let cut = k / 2;
        let p1 = min(len1, left_1 + cut) - 1;
        let p2 = min(len2, left_2 + cut) - 1;
        let num1 = nums1[p1];
        let num2 = nums2[p2];
        if num1 < num2 {
            Self::find_kth_num(nums1, p1 + 1, nums2, left_2, k - (p1 - left_1 + 1))
        } else {
            Self::find_kth_num(nums1, left_1, nums2, p2 + 1, k - (p2 - left_2 + 1))
        }
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {

        let len1 = nums1.len();
        let len2 = nums2.len();
        let is_odd = (len1 + len2) % 2 > 0;
        if is_odd {
            Self::find_kth_num(&nums1, 0, &nums2, 0, (len1 + len2) / 2 + 1) as f64
        } else {
            (
            Self::find_kth_num(&nums1, 0, &nums2, 0, (len1 + len2) / 2)
            +
            Self::find_kth_num(&nums1, 0, &nums2, 0, (len1 + len2) / 2 + 1)
            ) as f64
            /
            2.0
        }
    }
}
///
/// 1 3 5
/// 2 4 6
/// -> 3.5

pub fn test() {
    let mut all_true = true;
    {
        let case_a = vec![-2, -1];
        let case_a_print = format!("{:?}", case_a);
        let case_b = vec![3];
        let case_b_print = format!("{:?}", case_b);
        let resp = Solution::find_median_sorted_arrays(case_a, case_b);
        let expe = -1.0;
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
        let case_a = vec![1, 2];
        let case_a_print = format!("{:?}", case_a);
        let case_b = vec![3, 4];
        let case_b_print = format!("{:?}", case_b);
        let resp = Solution::find_median_sorted_arrays(case_a, case_b);
        let expe = 2.5;
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
        let case_a = vec![];
        let case_a_print = format!("{:?}", case_a);
        let case_b = vec![1];
        let case_b_print = format!("{:?}", case_b);
        let resp = Solution::find_median_sorted_arrays(case_a, case_b);
        let expe = 1.0;
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
        let case_a = vec![];
        let case_a_print = format!("{:?}", case_a);
        let case_b = vec![2, 4];
        let case_b_print = format!("{:?}", case_b);
        let resp = Solution::find_median_sorted_arrays(case_a, case_b);
        let expe = 3.0;
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
        let case_a = vec![1, 3];
        let case_a_print = format!("{:?}", case_a);
        let case_b = vec![2, 4];
        let case_b_print = format!("{:?}", case_b);
        let resp = Solution::find_median_sorted_arrays(case_a, case_b);
        let expe = 2.5;
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
        let case_a = vec![1, 3];
        let case_a_print = format!("{:?}", case_a);
        let case_b = vec![2];
        let case_b_print = format!("{:?}", case_b);
        let resp = Solution::find_median_sorted_arrays(case_a, case_b);
        let expe = 2.0;
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
        let case_a = vec![1, 3, 5, 7, 9, 11, 13];
        let case_a_print = format!("{:?}", case_a);
        let case_b = vec![2, 4];
        let case_b_print = format!("{:?}", case_b);
        let resp = Solution::find_median_sorted_arrays(case_a, case_b);
        let expe = 5.0;
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
        let case_a = vec![1];
        let case_a_print = format!("{:?}", case_a);
        let case_b = vec![2,3,4,5,6];
        let case_b_print = format!("{:?}", case_b);
        let resp = Solution::find_median_sorted_arrays(case_a, case_b);
        let expe = 3.5;
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
