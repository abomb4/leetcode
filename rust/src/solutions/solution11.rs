//
// 11. 盛最多水的容器
// 难度中等
// 给你 n 个非负整数 a1，a2，...，an，每个数代表坐标中的一个点 (i, ai) 。
//
// 在坐标内画 n 条垂直线，垂直线 i 的两个端点分别为 (i, ai) 和 (i, 0)。
//
// 找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。
//
// 说明：你不能倾斜容器，且 n 的值至少为 2。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/container-with-most-water
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
//

use std::cmp::min;

pub struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut left = 0;
        let mut right = len - 1;
        let mut max_size = 0;
        while left != right {
            let left_v = height[left];
            let right_v = height[right];
            let size = min(left_v, right_v) * (right - left) as i32;
            if size > max_size { max_size = size; }
            if left_v > right_v {
                right -= 1;
            } else {
                left += 1;
            }
        }
        return max_size;
    }
}

pub fn test() {
    let mut all_true = true;
    {
        let case = vec!(1,8,6,2,5,4,8,3,7);
        let case_print = format!("{:?}", case);
        let resp = Solution::max_area(case);
        let expe = 49;
        let success = resp == expe;
        if !success { all_true = false; }
        println!("[1] case: {0}, resp: {1}, expect: {2}, success: {3}", (case_print), resp, expe, success);
    }
    match all_true {
        true => println!("solution11 success"),
        false => println!("solution11 failed"),
    }
}
