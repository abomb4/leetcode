//
// 16. 最接近的三数之和
//
// 给你一个长度为 n 的整数数组 nums 和 一个目标值 target。请你从 nums 中选出三个整数，使它们的和与 target 最接近。
//
// 返回这三个数的和。
//
// 假定每组输入只存在恰好一个解。
//
//
//
// 示例 1：
//
// 输入：nums = [-1,2,1,-4], target = 1
// 输出：2
// 解释：与 target 最接近的和是 2 (-1 + 2 + 1 = 2) 。
//
// 示例 2：
//
// 输入：nums = [0,0,0], target = 1
// 输出：0
//
//
//
// 提示：
//
// 3 <= nums.length <= 1000
// -1000 <= nums[i] <= 1000
// -104 <= target <= 104
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/3sum-closest
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
//
pub struct Solution;
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        if len < 3 {
            panic!("CANNOT!");
        }
        let mut sum:i32 = 0;
        if len == 3 {
            for num in nums {
                sum += num;
            }
            return sum;
        }

        Self::qsort(&mut nums, 0, len - 1);

        let mut result: i32 = 0;
        let mut distance: u32 = u32::MAX;
        let mut r = len - 1;
        let mut i = 0;
        while i < r - 1 {
            let a = nums[i];
            let mut j = i + 1;
            while j < r {
                let b = nums[j];
                let c = nums[r];
                let sum = a + b + c;
                let d = (sum - target).abs() as u32;
                if d < distance {
                    result = sum;
                    distance = d;
                }
                if d == 0 {
                    return sum;
                }

                if sum < target {
                    j += 1;
                } else {
                    r -= 1;
                }
            }
            i += 1;
            r = len - 1;
        }

        result
    }

    fn qsort(nums: &mut Vec<i32>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mut i_base = right;
        let base = nums[i_base];
        let mut i_left = left;
        let mut i_right = right;

        while i_left < i_right {
            while nums[i_left] < base && i_left < i_right {
                i_left += 1;
            }
            if i_left < i_right {
                nums[i_base] = nums[i_left];
                i_base = i_left;
                i_left += 1;
            }

            while nums[i_right] > base && i_left < i_right {
                i_right -= 1;
            }
            if i_left < i_right {
                nums[i_base] = nums[i_right];
                i_base = i_right;
                i_right -= 1;
            }
        }
        nums[i_base] = base;
        if i_base > 0 {
            Self::qsort(nums, left, i_base - 1);
        }
        Self::qsort(nums, i_base + 1, right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        for c in vec!(
            (vec!(1, 2, 3), 0, 6),
            (vec!(-1, 2, 1, -4), 1, 2),
            (vec!(1,1,1,1), 0, 3),
            (vec!(-1,2,3,4,245,658,3,2,45,-213,46,56), 532, 501),
            (vec!(13,252,-87,-431,-148,387,-290,572,-311,-721,222,673,538,919,483,-128,-518,7,-36,-840,233,-184,-541,522,-162,127,-935,-397,761,903,-217,543,906,-503,-826,-342,599,-726,960,-235,436,-91,-511,-793,-658,-143,-524,-609,-728,-734,273,-19,-10,630,-294,-453,149,-581,-405,984,154,-968,623,-631,384,-825,308,779,-7,617,221,394,151,-282,472,332,-5,-509,611,-116,113,672,-497,-182,307,-592,925,766,-62,237,-8,789,318,-314,-792,-632,-781,375,939,-304,-149,544,-742,663,484,802,616,501,-269,-458,-763,-950,-390,-816,683,-219,381,478,-129,602,-931,128,502,508,-565,-243,-695,-943,-987,-692,346,-13,-225,-740,-441,-112,658,855,-531,542,839,795,-664,404,-844,-164,-709,167,953,-941,-848,211,-75,792,-208,569,-647,-714,-76,-603,-852,-665,-897,-627,123,-177,-35,-519,-241,-711,-74,420,-2,-101,715,708,256,-307,466,-602,-636,990,857,70,590,-4,610,-151,196,-981,385,-689,-617,827,360,-959,-289,620,933,-522,597,-667,-882,524,181,-854,275,-600,453,-942,134), -2805, -2805),
        ) {
            let r = Solution::three_sum_closest(c.0.clone(), c.1);
            assert_eq!(r, c.2,
                       "exp: {}, real: {}, vec: {:?}", c.2, r, c.0.clone());
        }
    }
}
