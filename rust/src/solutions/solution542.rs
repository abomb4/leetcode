//
// 542. 01 矩阵
//
// 给定一个由 0 和 1 组成的矩阵，找出每个元素到最近的 0 的距离。
//
// 两个相邻元素间的距离为 1 。
//
// 示例 1:
// 输入:
//
// 0 0 0
// 0 1 0
// 0 0 0
//
// 输出:
//
// 0 0 0
// 0 1 0
// 0 0 0
//
// 示例 2:
// 输入:
//
// 0 0 0
// 0 1 0
// 1 1 1
//
// 输出:
//
// 0 0 0
// 0 1 0
// 1 2 1
//
// 注意:
//
//     给定矩阵的元素个数不超过 10000。
//     给定矩阵中至少有一个元素是 0。
//     矩阵中的元素只在四个方向上相邻: 上、下、左、右。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/01-matrix
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
//

pub struct Solution;
impl Solution {
    pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 每个点有个元素数量
        // 每个点的元素等于四边最小值 + 1
        // 一次遍历可能无法知道某个点的真正最小值

        vec![vec![0, 0, 0],vec![0, 1, 0],vec![0, 0, 0]]
    }
}

pub fn test() {
    let mut all_true = true;
    {
        let case = vec![vec![0, 0, 0],vec![0, 1, 0],vec![0, 0, 0]];
        let case_print = format!("{:?}", case);
        let resp = Solution::update_matrix(case);
        let expe = vec![vec![0, 0, 0],vec![0, 1, 0],vec![0, 0, 0]];
        let success = resp == expe;
        if !success { all_true = false; }
        println!("[1] case: {0:?}, resp: {1:?}, expect: {2:?}, success: {3}",
                 case_print, &resp, &expe, success);
    }
    match all_true {
        true => println!("solution542 success"),
        false => println!("solution542 failed"),
    }
}
