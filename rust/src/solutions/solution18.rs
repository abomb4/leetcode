//
// 18. 四数之和
// 难度：中等
// 
// 给你一个由 n 个整数组成的数组 nums ，和一个目标值 target 。
// 请你找出并返回满足下述全部条件且不重复的四元组 [nums[a], nums[b], nums[c], nums[d]]
// （若两个四元组元素一一对应，则认为两个四元组重复）：
//
//     0 <= a, b, c, d < n
//     a、b、c 和 d 互不相同
//     nums[a] + nums[b] + nums[c] + nums[d] == target
//
// 你可以按 任意顺序 返回答案 。
//
//
//
// 示例 1：
//
// 输入：nums = [1,0,-1,0,-2,2], target = 0
// 输出：[[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
//
// 示例 2：
//
// 输入：nums = [2,2,2,2,2], target = 8
// 输出：[[2,2,2,2]]
//
//
//
// 提示：
//
//     1 <= nums.length <= 200
//     -109 <= nums[i] <= 109
//     -109 <= target <= 109
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/4sum
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
//
pub struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = nums.len();

        if len < 4 {
            return Vec::new();
        }

        nums.sort();

        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut first1 = true;
        let mut first2;
        let mut first3;
        let mut first4;
        let mut buf1: i32;
        let mut buf2: i32;

        for p1 in 0..len {
            buf1 = nums[p1];
            if !first1 && buf1 == nums[p1 - 1] {
                continue;
            }
            first1 = false;
            first2 = true;
            for p2 in p1 + 1..len {
                buf2 = nums[p2];
                if !first2 && buf2 == nums[p2 - 1] {
                    continue;
                }
                first2 = false;
                let mut p3 = p2 + 1;
                let mut p4 = len - 1;
                first3 = true;
                first4 = true;
                while p3 < p4 {
                    let buf3 = nums[p3];
                    let buf4 = nums[p4];
                    if !first3 && buf3 == nums[p3 - 1] {
                        p3 += 1;
                        continue;
                    }
                    if !first4 && buf4 == nums[p4 + 1] {
                        p4 -= 1;
                        continue;
                    }
                    let sum: i64 = buf1 as i64 + buf2 as i64 + buf3 as i64 + buf4 as i64;
                    if sum == target as i64 {
                        result.push(vec!(buf1, buf2, buf3, buf4));
                        p3 += 1;
                        first3 = false;
                    } else if sum > target as i64 {
                        p4 -= 1;
                        first4 = false;
                    } else {
                        p3 += 1;
                        first3 = false;
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::solution18::Solution;

    #[test]
    fn test1() {
        let nums: Vec<i32> = vec![1, 0, -1, 0, -2, 2];
        let nums_copy = nums.clone();
        let target = 0;
        let expect = vec!(
            vec!(-2, -1, 1, 2),
            vec!(-2, 0, 0, 2),
            vec!(-1, 0, 0, 1),
        );
        let result = Solution::four_sum(nums, target);
        assert_eq!(result, expect, "nums: {:?}, target: {:?}, expect: {:?}, actual: {:?}",
                   nums_copy, target, expect, result);
    }

    #[test]
    fn test2() {
        let nums: Vec<i32> = vec![2, 2, 2, 2, 2];
        let nums_copy = nums.clone();
        let target = 8;
        let expect = vec!(
            vec!(2, 2, 2, 2),
        );
        let result = Solution::four_sum(nums, target);
        assert_eq!(result, expect, "nums: {:?}, target: {:?}, expect: {:?}, actual: {:?}",
                   nums_copy, target, expect, result);
    }

    #[test]
    fn test3() {
        let nums: Vec<i32> = vec![1000000000,1000000000,1000000000,1000000000];
        let nums_copy = nums.clone();
        let target = -294967296;
        let expect: Vec<Vec<i32>> = vec!();
        let result = Solution::four_sum(nums, target);
        assert_eq!(result, expect, "nums: {:?}, target: {:?}, expect: {:?}, actual: {:?}",
                   nums_copy, target, expect, result);
    }
}
