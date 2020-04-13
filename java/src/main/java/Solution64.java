import java.util.Arrays;

class Solution64 {
    public static void main(String[] args) {
        int[][] grid = new int[][] {
                new int[]{7, 0, 8, 8, 0, 3, 5, 8, 5, 4},
                new int[]{4, 1, 2, 9, 9, 6, 0, 8, 6, 9},
                new int[]{9, 7, 1, 1, 0, 1, 2, 4, 1, 7}
        }
;
        System.out.println(Arrays.deepToString(grid).replaceAll("], ", "],\n"));
        System.out.println(new Solution64().minPathSum(grid));
    }
    public int minPathSum(int[][] grid) {
        // 应该是先找出一条作为最小值，然后比这个大的就没必要再找了
        // 看看能不能把递归化成平的

        final int length1 = grid.length;
        final int length2 = grid[0].length;
        final int[][] totalArray = new int[grid.length][grid[0].length];

        // find a way
        int min = 0;
        for (int i = 0; i < length1; i++) {
            min += grid[i][0];
            totalArray[i][0] = min;
        }
        for (int j = 1; j < length2; j++) {
            min += grid[length1 - 1][j];
            totalArray[length1 - 1][j] = min;

            totalArray[0][j] = totalArray[0][j - 1] + grid[0][j];
        }

        // find all way
        int total = 0;
        for (int i = 1; i < length1; i++) {
            for (int j = 0; j < length2; j++) {
                if (j == 0) {
                    totalArray[i][j] = totalArray[i - 1][j] + grid[i][j];
                } else {
                    totalArray[i][j] =
                        Math.min(totalArray[i - 1][j], totalArray[i][j - 1])
                            + grid[i][j];
                }

                System.out.println(Arrays.deepToString(totalArray).replaceAll("], ", "],\n").replaceAll("]]", "]\n]").replaceAll("\\[\\[", "[\n["));
                // if (totalArray[i][j] > min) {
                //     // System.out.println("break");
                //     break;
                // }
            }
        }

        System.out.println(Arrays.deepToString(grid).replaceAll("], ", "],\n").replaceAll("]]", "]\n]").replaceAll("\\[\\[", "[\n["));
        System.out.println(Arrays.deepToString(totalArray).replaceAll("], ", "],\n"));
        return totalArray[length1 - 1][length2 - 1];
    }
}
