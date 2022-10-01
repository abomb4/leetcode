//
// 974. 和可被 K 整除的子数组
//
// 给定一个整数数组 A，返回其中元素之和可被 K 整除的（连续、非空）子数组的数目。
//
//
//
// 示例：
//
// 输入：A = [4,5,0,-2,-3,1], K = 5
// 输出：7
// 解释：
// 有 7 个子数组满足其元素之和可被 K = 5 整除：
// [4, 5, 0, -2, -3, 1], [5], [5, 0], [5, 0, -2, -3], [0], [0, -2, -3], [-2, -3]
//
//
//
// 提示：
//
//     1 <= A.length <= 30000
//     -10000 <= A[i] <= 10000
//     2 <= K <= 10000
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/subarray-sums-divisible-by-k
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
//

pub struct Solution;
impl Solution {
    pub fn subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
        let mut array = vec![vec![0; a.len()]; 2];
        let mut sum = 0;
        let mut i_prev = 0;
        for i in 0..a.len() {
            for j in 0..(i + 1) {
                let add;
                if j == 0 {
                    add = a[i];
                } else {
                    let prev = array[i_prev][j - 1];
                    add = prev + a[i];
                }
                array[i_prev ^ 1][j] = add;
                if add % k == 0 {
                    sum += 1;
                    println!("at [{}, {}], is {}", i, j, add);
                };
            }
            i_prev = i_prev ^ 1;
        }
        println!("{:?}", array);

        sum
    }
}

pub fn test() {
    let mut all_true = true;
    {
        let case_a = vec![4,5,0,-2,-3,1];
        let case_b = 5;
        let case_print = format!("{:?}", case_a);
        let resp = Solution::subarrays_div_by_k(case_a, case_b);
        let expe = 7;
        let success = resp == expe;
        if !success { all_true = false; }
        println!("[1] case_a: {:?}, case_b: {}, resp: {:?}, expect: {:?}, success: {}",
                 case_print, case_b, &resp, &expe, success);
    }
    {
        let case_a = vec![0, -5];
        let case_b = 10;
        let case_print = format!("{:?}", case_a);
        let resp = Solution::subarrays_div_by_k(case_a, case_b);
        let expe = 1;
        let success = resp == expe;
        if !success { all_true = false; }
        println!("[1] case_a: {:?}, case_b: {}, resp: {:?}, expect: {:?}, success: {}",
                 case_print, case_b, &resp, &expe, success);
    }
    match all_true {
        true => println!("solution974 success"),
        false => println!("solution974 failed"),
    }
}
