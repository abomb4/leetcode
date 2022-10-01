/**
 * 62. 不同路径
 *
 * @author abomb4 2020-04-05
 */
class Solution62 {
    public int uniquePaths(int m, int n) {

        // 思路是记得某种走法之前有几种到这的方法；
        // 也就是，想要到达 [a, b] ，只要 [a-1, b] 和 [a, b-1] 的次数相加就好。
        // 若 a - 1 或 b - 1 超出范围，则按 0 计算。
        // 已知了第一个点，后面都可以公式算出。

        if (m == 1 || n == 1) {
            return 1;
        }

        // arr[i, j] means from [0, 0] to [i, j] have n different paths
        int[][] arr = new int[m][n];
        arr[0][0] = 1;
        for (int i = 1; i < m; i++) {
            for (int j = 1; j < n; j++) {
                arr[i][j] = getPaths(arr, i - 1, j) + getPaths(arr, i, j - 1);
            }
        }

        return arr[m - 1][n - 1];
    }

    /**
     * This method assume arr[i, j] what i and j is in range is set
     */
    int getPaths(int[][] arr, int i, int j) {
        if (i == 0 || j == 0) {
            return 1;
        } else if (i < 0 || j < 0) {
            return 0;
        }
        return arr[i][j];
    }
}
