class Solution72 {
    public static void main(String[] args) {
        System.out.println(new Solution72().minDistance("horse", "ros"));
    }


    public int minDistance(String word1, String word2) {
        // 把问题化小，计算某一段 word1 转化成 word2 的最小次数
        // 设 dp[i][j] 表示长度为 i 的 word1 片段构造长度为 j 的 word2 所需操作次数。
        // 若需要删除某字符才能与目标串相同，则次数 = 构造当前目标串所需次数 + 1
        //     构造当前目标串所需次数 = dp[i - 1][j]
        //
        // 若需要插入，次数 = 最早构造目标串的次数（dp[i][j - 1]） ??? 。
        //     因为从
        //
        // 若需要修改，次数 = dp[i - 1][j - 1] ????

        final char[] chars1 = word1.toCharArray();
        final char[] chars2 = word2.toCharArray();
        final int l1 = word1.length();
        final int l2 = word2.length();
        int[][] dp = new int[l1 + 1][l2 + 1];

        for (int i = 0; i <= l1; i++) {
            dp[i][0] = i;
        }
        for (int j = 0; j <= l2; j++) {
            dp[0][j] = j;
        }

        for (int i = 1; i <= l1; i++) {
            for (int j = 1; j <= l2; j++) {
                // 长度为 i 的 word1 ，想要构造长度为 j 的 word2 所需次数
                // （1）长度为 i - 1 的 word1 ，想要构造长度为 j 的 word2 所需次数 （再补充一下子）
                // （2）长度为 i 的 word1 ，想要构造长度为 j - 1 的 word2 所需次数
                // （3）长度为 i - 1 的 word1 ，想要构造长度为 j - 1 的 word2 所需次数
                // 若字符相同，等于（3）的次数
                // 若字符不同，需要删掉才能与 j 相同，等于从 i - 1 长度构建 j 的次数 + 1（删除）
                // 若字符不同，需要加一个才能与 j 形同，说明长度不够，从构造短一截的 [i][j - 1] 的基础上新增即可
                // 若需要修改，[i - 1][j - 1] 的数字上替换 [i, j] 上的字符即可
                if (chars1[i - 1] == chars2[j - 1]) {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = Math.min(Math.min(dp[i - 1][j - 1], dp[i][j - 1]), dp[i - 1][j]) + 1;
                }
            }
        }
        return dp[l1][l2];
    }
}
