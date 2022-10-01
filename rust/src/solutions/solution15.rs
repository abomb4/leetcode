//
// 15. 三数之和
//
// 给你一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0 ？请你找出所有满足条件且不重复的三元组。
//
// 注意：答案中不可以包含重复的三元组。
//
// 示例：
//
// 给定数组 nums = [-1, 0, 1, 2, -1, -4]，
//
// 满足要求的三元组集合为：
// [
//   [-1, 0, 1],
//   [-1, -1, 2]
// ]
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/3sum
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
//

pub struct Solution;
impl Solution {
    fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
        let v = nums[i];
        nums[i] = nums[j];
        nums[j] = v;
    }

    fn qsort(nums: &mut Vec<i32>, left_bound: usize, right_bound: usize) {
        if right_bound <= left_bound {
            return;
        }

        let value = nums[right_bound];
        let mut i = left_bound;
        let mut j = right_bound;
        while i < j {
            while i < j && nums[i] <= value {
                i += 1;
            }
            while i < j && nums[j] >= value {
                j -= 1;
            }
            if i == j {
                Self::swap(nums, right_bound, j);
                break;
            } else {
                Self::swap(nums, i, j);
            }
        }

        if i > 1 {
            // usize may overflow
            Self::qsort(nums, left_bound, i - 1);
        }
        Self::qsort(nums, i + 1, right_bound);
    }

    fn add_to_result(result: &mut Vec<Vec<i32>>, mut a: Vec<i32>) {
        let len = a.len();
        Self::qsort(&mut a, 0, len - 1);
        if !result.contains(&a) {
            result.push(a);
        }
    }

    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        if len < 3 {
            return Vec::new();
        }
        Self::qsort(&mut nums, 0, len - 1);

        if nums[0] <= 0 && nums[len - 1] >= 0 {
            let mut result = Vec::new();

            for i in 0 .. len {
                let v = nums[i];
                let target = -v;
                let mut j = 0 as usize;
                let mut k = len - 1;
                loop {
                    if j >= k { break; }
                    if j == i { j += 1; continue; }
                    if k == i { k -= 1; continue; }
                    let left = nums[j];
                    let right = nums[k];
                    let sum = left + right;
                    if (left < v && right < v) || (left > v && right > v) {
                        break;
                    }
                    if sum == target {
                        Self::add_to_result(&mut result, vec!(v, left, right));
                        while j < k && nums[j] == nums[j + 1] {
                            j += 1;
                        }
                        while j < k && k > 1 && nums[k] == nums[k - 1] {
                            k -= 1;
                        }
                        j += 1;
                        k -= 1;
                    } else if sum < target {
                        j += 1;
                    } else {
                        k -= 1;
                    }
                }
            }
            result
        } else {
            Vec::new()
        }
    }
}

pub fn test() {
    let mut all_true = true;
    {
        let case = vec!(-1, 0, 1, 2, -1, -4);
        let case_print = format!("{:?}", case);
        let resp = Solution::three_sum(case);
        let resp_print = format!("{:?}", resp);
        let expe = vec!(vec!(-1, -1, 2), vec!(-1, 0, 1));
        let expe_print = format!("{:?}", expe);
        let success = resp == expe;
        if !success { all_true = false; }
        println!("[1] case: {0}, resp: {1}, expect: {2}, success: {3}", (case_print), resp_print, expe_print, success);
    }
    {
        let case = Vec::new();
        let case_print = format!("{:?}", case);
        let resp = Solution::three_sum(case);
        let resp_print = format!("{:?}", resp);
        let expe: Vec<Vec<i32>> = Vec::new();
        let expe_print = format!("{:?}", expe);
        let success = resp == expe;
        if !success { all_true = false; }
        println!("[1] case: {0}, resp: {1}, expect: {2}, success: {3}", (case_print), resp_print, expe_print, success);
    }
    match all_true {
        true => println!("solution11 success"),
        false => println!("solution11 failed"),
    }
}
