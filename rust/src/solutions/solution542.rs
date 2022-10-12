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

        let len_x = matrix.len();
        if len_x <= 0 {
            return Vec::new();
        }

        let len_y = matrix[0].len();
        if len_y <= 0 {
            return Vec::new();
        }

        let mut result: Vec<Vec<i32>> = Vec::with_capacity(len_x);
        for i in 0..len_x {
            result.push(Vec::with_capacity(len_y));
            for j in 0..len_y {
                let num = matrix[i][j];
                if num == 0 {
                    result[i].push(0);
                    continue;
                }
                let mut min = i32::MAX;
                if i > 0 {
                    if matrix[i - 1][j] == 0 {
                        result[i].push(1);
                        continue;
                    }
                    let previous = result[i - 1][j];
                    if previous != i32::MAX {
                        min = min.min(previous + 1);
                    }
                }
                if j > 0 {
                    if matrix[i][j - 1] == 0 {
                        result[i].push(1);
                        continue;
                    }
                    let previous = result[i][j - 1];
                    if previous != i32::MAX {
                        min = min.min(previous + 1);
                    }
                }
                result[i].push(min);
            }
        }
        let i_m_x = len_x - 1;
        let i_m_y = len_y - 1;
        for i in (0..len_x).rev() {
            for j in (0..len_y).rev() {
                let mut min = result[i][j];
                if min == 0 || min == 1 {
                    continue;
                }
                if i < i_m_x {
                    if matrix[i + 1][j] == 0 {
                        result[i][j] = 1;
                        continue;
                    }
                    min = min.min(result[i + 1][j] + 1);
                }
                if j < i_m_y {
                    if matrix[i][j + 1] == 0 {
                        result[i][j] = 1;
                        continue;
                    }
                    min = min.min(result[i][j + 1] + 1);
                }
                result[i][j] = min;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test1() {
        let case = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let case_print = format!("{:?}", case);
        let resp = Solution::update_matrix(case);
        let expe = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(
            resp, expe,
            "[1] case: {0:?}, resp: {1:?}, expect: {2:?}",
            case_print, &resp, &expe
        );
    }

    #[test]
    pub fn test2() {
        let case = vec![
            vec![0, 1, 0, 1, 1],
            vec![1, 1, 0, 0, 1],
            vec![0, 0, 0, 1, 0],
            vec![1, 0, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
        ];
        let case_print = format!("{:?}", case);
        let resp = Solution::update_matrix(case);
        let expe = vec![
            vec![0, 1, 0, 1, 2],
            vec![1, 1, 0, 0, 1],
            vec![0, 0, 0, 1, 0],
            vec![1, 0, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
        ];

        assert_eq!(
            resp, expe,
            "[1] case: {0:?}, resp: {1:?}, expect: {2:?}",
            case_print, &resp, &expe
        );
    }

    #[test]
    pub fn test3() {
        let case = vec![
            vec![1, 0, 1, 0, 0, 0, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 1, 1, 1, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 1, 0, 1, 0, 0],
            vec![1, 0, 1, 1, 1, 0, 1, 1, 1, 1],
            vec![1, 1, 0, 1, 1, 1, 1, 0, 0, 0],
            vec![1, 1, 0, 0, 1, 0, 1, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 0, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 1, 1, 1, 0, 0, 1, 0, 1, 1],
            vec![1, 1, 0, 0, 0, 1, 0, 1, 1, 0],
        ];
        let case_print = format!("{:?}", case);
        let resp = Solution::update_matrix(case);
        let expe = vec![
            vec![1, 0, 1, 0, 0, 0, 1, 2, 1, 2],
            vec![2, 1, 1, 0, 1, 1, 1, 1, 0, 1],
            vec![1, 0, 1, 1, 2, 1, 0, 1, 0, 0],
            vec![1, 0, 1, 2, 1, 0, 1, 1, 1, 1],
            vec![2, 1, 0, 1, 2, 1, 1, 0, 0, 0],
            vec![2, 1, 0, 0, 1, 0, 1, 1, 0, 1],
            vec![2, 2, 1, 1, 1, 1, 2, 2, 1, 1],
            vec![1, 1, 0, 0, 0, 1, 2, 1, 0, 0],
            vec![0, 1, 1, 1, 0, 0, 1, 0, 1, 1],
            vec![1, 1, 0, 0, 0, 1, 0, 1, 1, 0],
        ];

        assert_eq!(
            resp, expe,
            "[1] case: {0:?}, resp: {1:?}, expect: {2:?}",
            case_print, &resp, &expe
        );
    }
}
