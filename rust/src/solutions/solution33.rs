//
// 33. 搜索旋转排序数组
//
// 假设按照升序排序的数组在预先未知的某个点上进行了旋转。
//
// ( 例如，数组 [0,1,2,4,5,6,7] 可能变为 [4,5,6,7,0,1,2] )。
//
// 搜索一个给定的目标值，如果数组中存在这个目标值，则返回它的索引，否则返回 -1 。
//
// 你可以假设数组中不存在重复的元素。
//
// 你的算法时间复杂度必须是 O(log n) 级别。
//
// 示例 1:
//
// 输入: nums = [4,5,6,7,0,1,2], target = 0
// 输出: 4
//
// 示例 2:
//
// 输入: nums = [4,5,6,7,0,1,2], target = 3
// 输出: -1
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/search-in-rotated-sorted-array
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
//

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // 先判断是不是那种非循环的情况
        let is_sorted;
        match (nums.first(), nums.last()) {
            (Some(n1), Some(n2)) => { is_sorted = n1 < n2 },
            _ => return -1
        }

        if nums.len() == 1 {
            return if *nums.first().unwrap() == target {
                0
            } else {
                -1
            }
        }

        if is_sorted {
            Self::bin_search(&nums, target)
        } else {
            let end_index = Self::find_loop_end(&nums);
            let largest: i32 = nums[end_index];
            let smallest: i32 = nums[end_index + 1];
            let middle: i32 = nums.last().unwrap().clone();
            if target > largest && target < smallest {
                return -1;
            }
            if target > middle {
                Self::bin_search(&nums[0 .. end_index + 1], target)
            } else {
                let result = Self::bin_search(&nums[(end_index + 1) .. nums.len()], target);
                if result != -1 {
                    result + end_index as i32 + 1
                } else {
                    -1
                }
            }
        }
    }

    fn bin_search(nums: &[i32], target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(v) => v as i32,
            Err(_err) => -1
        }
    }

    // The biggest number's index
    fn find_loop_end(nums: &[i32]) -> usize {
        let right_value: i32 = nums.last().unwrap().clone();
        let len = nums.len();
        let index = (len - 1) / 2;
        loop {
            match (nums.get(index), nums.get(index + 1)) {
                (Some(num), Some(next)) => {
                    if *num > *next {
                        return index;
                    }
                    return if *num > right_value {
                        Self::find_loop_end(&nums[index + 1 .. len]) + index + 1
                    } else {
                        Self::find_loop_end(&nums[0 .. index + 1])
                    }
                },
                _ => {
                    println!("{:?}", nums);
                    panic!("Impossible here, this program must have bug")
                }
            }
        }
    }
}

pub fn test() {
    let mut all_true = true;
    {
        let case_nums = vec![4, 5, 6, 7, 0, 1, 2];
        let case_nums_debug = format!("{:?}", case_nums);
        let case_target = 0;
        let resp = Solution::search(case_nums, case_target);
        let expect = 4;
        let success = expect == resp;
        if !success { all_true = false; }
        println!("[1] nums: {0}, target: {1}, resp: {2}, expect: {3}, success: {4}",
                 case_nums_debug, case_target, resp, expect, success);
    }
    {
        let case_nums = vec![4, 5, 6, 7, 0, 1, 2];
        let case_nums_debug = format!("{:?}", case_nums);
        let case_target = 3;
        let resp = Solution::search(case_nums, case_target);
        let expect = -1;
        let success = expect == resp;
        if !success { all_true = false; }
        println!("[2] nums: {0}, target: {1}, resp: {2}, expect: {3}, success: {4}",
                 case_nums_debug, case_target, resp, expect, success);
    }
    {
        let case_nums = vec![3, 1];
        let case_nums_debug = format!("{:?}", case_nums);
        let case_target = 0;
        let resp = Solution::search(case_nums, case_target);
        let expect = -1;
        let success = expect == resp;
        if !success { all_true = false; }
        println!("[3] nums: {0}, target: {1}, resp: {2}, expect: {3}, success: {4}",
                 case_nums_debug, case_target, resp, expect, success);
    }
    {
        let case_nums = vec![5, 1, 3];
        let case_nums_debug = format!("{:?}", case_nums);
        let case_target = 0;
        let resp = Solution::search(case_nums, case_target);
        let expect = -1;
        let success = expect == resp;
        if !success { all_true = false; }
        println!("[4] nums: {0}, target: {1}, resp: {2}, expect: {3}, success: {4}",
                 case_nums_debug, case_target, resp, expect, success);
    }
    {
        let case_nums = vec![5, 1, 3];
        let case_nums_debug = format!("{:?}", case_nums);
        let case_target = 3;
        let resp = Solution::search(case_nums, case_target);
        let expect = 2;
        let success = expect == resp;
        if !success { all_true = false; }
        println!("[5] nums: {0}, target: {1}, resp: {2}, expect: {3}, success: {4}",
                 case_nums_debug, case_target, resp, expect, success);
    }
    {
        let case_nums = vec![3, 4, 5, 6, 1, 2];
        let case_nums_debug = format!("{:?}", case_nums);
        let case_target = 2;
        let resp = Solution::search(case_nums, case_target);
        let expect = 5;
        let success = expect == resp;
        if !success { all_true = false; }
        println!("[6] nums: {0}, target: {1}, resp: {2}, expect: {3}, success: {4}",
                 case_nums_debug, case_target, resp, expect, success);
    }
    {
        let case_nums = vec![4,5,6,7,8,1,2,3];
        let case_nums_debug = format!("{:?}", case_nums);
        let case_target = 8;
        let resp = Solution::search(case_nums, case_target);
        let expect = 4;
        let success = expect == resp;
        if !success { all_true = false; }
        println!("[7] nums: {0}, target: {1}, resp: {2}, expect: {3}, success: {4}",
                 case_nums_debug, case_target, resp, expect, success);
    }
    match all_true {
        true => println!("solution33 success"),
        false => println!("solution33 failed"),
    }
}
