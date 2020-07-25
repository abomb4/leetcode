//
// 215. 数组中的第K个最大元素
//
// 在未排序的数组中找到第 k 个最大的元素。请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。
//
// 示例 1:
//
// 输入: [3,2,1,5,6,4] 和 k = 2
// 输出: 5
//
// 示例 2:
//
// 输入: [3,2,3,1,2,4,5,5,6] 和 k = 4
// 输出: 4
//
// 说明:
//
// 你可以假设 k 总是有效的，且 1 ≤ k ≤ 数组的长度。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/kth-largest-element-in-an-array
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
//

use std::time::{SystemTime, UNIX_EPOCH};

const SOLUTION_NUMBER: usize = 215;
pub struct Solution;
impl Solution {

    fn fake_rand(left_bound: usize, right_bound: usize) -> usize {
        let time = SystemTime::now();
        let ep = time.duration_since(UNIX_EPOCH).expect("Time failed").as_millis() as u64;
        let rnd: usize = (ep % ((right_bound - left_bound) as u64)) as usize;

        left_bound + rnd
    }

    fn swap<T: Copy>(nums: &mut Vec<T>, i: usize, j: usize) {
        if i != j {
            let v = nums[i];
            nums[i] = nums[j];
            nums[j] = v;
        }
    }

    fn qsort<T: PartialOrd + Copy>(nums: &mut Vec<T>, left_bound: usize, right_bound: usize) {
        if left_bound >= right_bound {
            return;
        }
        let rand = Self::fake_rand(left_bound, right_bound);
        if rand != right_bound {
            Self::swap(nums, rand, right_bound);
        }

        let base = right_bound;
        let base_value = nums[base];
        let mut lo = left_bound;
        let mut hi = right_bound;

        loop {
            while lo < hi && nums[lo] <= base_value {
                lo += 1;
            }
            while lo < hi && nums[hi] >= base_value {
                hi -= 1;
            }
            if lo == hi {
                // finished
                Self::swap(nums, base, hi);
                break;
            } else {
                Self::swap(nums, lo, hi);
            }
        }

        if lo > 0 {
            Self::qsort(nums, left_bound, lo - 1);
        }
        Self::qsort(nums, lo + 1, right_bound);
    }

    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        // 直接排序然后就找呗哈哈哈
        if k < 0 {
            panic!("Negative k is not allowd");
        }

        let len = nums.len();
        if (k as usize) > len {
            panic!("k is too large!");
        }

        Self::qsort(&mut nums, 0, len - 1);

        nums[len - (k as usize)]
    }
}

pub fn test() {
    let mut all_true = true;
    {
        let case_a = vec!(3,2,1,5,6,4);
        let case_a_print = format!("{:?}", case_a);
        let case_b = 2;
        let case_b_print = format!("{:?}", case_b);
        let resp = Solution::find_kth_largest(case_a, case_b);
        let expe = 5;
        let success = resp == expe;
        if !success { all_true = false; }
        println!("[1] case_a: {0}, case_b: {1}, resp: {2}, expect: {3}, success: {4}",
            case_a_print, case_b_print, resp, expe, success);
    }
    match all_true {
        true => println!("solution{0} success", SOLUTION_NUMBER),
        false => println!("solution{0} failed", SOLUTION_NUMBER),
    }
}
