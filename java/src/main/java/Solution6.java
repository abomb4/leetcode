/**
 * 6. Z 字形变换
 * <p>
 * 将一个给定字符串 s 根据给定的行数 numRows ，以从上往下、从左到右进行 Z 字形排列。
 * <p>
 * 比如输入字符串为 "PAYPALISHIRING" 行数为 3 时，排列如下：
 * <pre>
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 * </pre>
 * 之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："PAHNAPLSIIGYIR"。
 * <p>
 * 请你实现这个将字符串进行指定行数变换的函数：
 * <p>
 * string convert(string s, int numRows);
 * <p>
 * <p>
 * <p>
 * 示例 1：
 * <p>
 * 输入：s = "PAYPALISHIRING", numRows = 3
 * 输出："PAHNAPLSIIGYIR"
 * <p>
 * 示例 2：
 * <p>
 * 输入：s = "PAYPALISHIRING", numRows = 4
 * 输出："PINALSIGYAHRPI"
 * 解释：
 * <pre>
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 * </pre>
 * 示例 3：
 * <p>
 * 输入：s = "A", numRows = 1
 * 输出："A"
 * <p>
 * <p>
 * <p>
 * 提示：
 * <p>
 * 1 <= s.length <= 1000
 * s 由英文字母（小写和大写）、',' 和 '.' 组成
 * 1 <= numRows <= 1000
 * 将一个给定字符串 s 根据给定的行数 numRows ，以从上往下、从左到右进行 Z 字形排列。
 * <pre>
 * 来源：力扣（LeetCode）
 * 链接：https://leetcode.cn/problems/zigzag-conversion
 * 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 * </pre>
 */
public class Solution6 {

    public static void main(String[] args) {
        class Case {
            String in;
            int n;
            String expected;

            public Case(String in, int n, String expected) {
                this.in = in;
                this.n = n;
                this.expected = expected;
            }
        }
        Case[] cs = new Case[]{
                new Case("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR"),
                new Case("PAYPALISHIRINGOHWEIOFJINSIODV", 5, "PHWSASIHENIYIROIIOPLIGOJDANFV"),
        };
        Solution6 s = new Solution6();
        for (Case c : cs) {
            String result = s.convert(c.in, c.n);
            System.out.printf("(%s) in: %s, n: %d, real: %s, expect: %s%n",
                    result.equals(c.expected), c.in, c.n, result, c.expected);
        }
    }

    public String convert(String s, int numRows) {
        // 0 1 2 3

        // 0 2 4
        // 1 3 5

        // 0   4   8
        // 1 3 5 7 9
        // 2   6   10

        // 0     6
        // 1   5 7
        // 2 4   8
        // 3     9

        // 0       8
        // 1     7 9
        // 2   6   10
        // 3 5     11
        // 4       12
        final int len = s.length();
        final StringBuilder sb = new StringBuilder(len);
        final int[] step = new int[2];
        for (int i = 0; i < numRows; i++) {
            if (i == 0 || i == numRows - 1) {
                step[0] = Math.max((numRows - 1) * 2, 1);
                step[1] = step[0];
            } else {
                step[0] = (numRows - 1 - i) * 2;
                step[1] = (numRows - 1) * 2 - step[0];
            }
            int current = 0;
            for (int j = i; j < len; j += step[current], current = current ^ 1) {
                sb.append(s.charAt(j));
            }
        }
        return sb.toString();
    }
}