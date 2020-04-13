/**
 * 一条包含字母 A-Z 的消息通过以下方式进行了编码：
 *
 * 'A' -> 1
 * 'B' -> 2
 * ...
 * 'Z' -> 26
 *
 * 给定一个只包含数字的非空字符串，请计算解码方法的总数。
 *
 * 示例 1:
 *
 * 输入: "12"
 * 输出: 2
 * 解释: 它可以解码为 "AB"（1 2）或者 "L"（12）。
 *
 * 示例 2:
 *
 * 输入: "226"
 * 输出: 3
 * 解释: 它可以解码为 "BZ" (2 26), "VF" (22 6), 或者 "BBF" (2 2 6) 。
 *
 * 来源：力扣（LeetCode）
 * 链接：https://leetcode-cn.com/problems/decode-ways
 * 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */
class Solution91 {

    public static void main(String[] args) {
        final Solution91 s = new Solution91();
        System.out.println(s.numDecodings("1"));
        System.out.println(s.numDecodings("10"));
        System.out.println(s.numDecodings("101"));
        System.out.println(s.numDecodings("1010"));
        System.out.println(s.numDecodings("10101"));
        System.out.println(s.numDecodings("101010"));
    }
    public int numDecodings(String s) {
        // 设 G(n) 是该串前 n 位能够组成的编码总数
        // 当字符长度增加时，新字符可以选择单独组成编码（肯定可以），
        // 也可以尝试与前面的数字组成编码；
        //
        // 当可以与前面数字组成编码时，其公式为 G(n - 2)，即“不算前面那个字符能够组成的编码总数”
        // G("1234") -> G("12")(if can) + G("123")
        int l = s.length();
        if (l == 0) {
            return 0;
        }
        if ('0' == s.charAt(0)) {
            return 0;
        }
        int[] dp = new int[l + 1];
        dp[0] = 0;
        dp[1] = 1;
        for (int i = 2; i <= l; i++) {
            // i means 'current length'
            char current = s.charAt(i - 1);
            char previous = s.charAt(i - 2);
            if (current != '0') {
                dp[i] = dp[i - 1];
            }
            if (previous != '0') {
                int num = Integer.parseInt(previous + "" + current);
                if (num > 0 && num <= 26) {
                    dp[i] += Math.max(1, dp[i - 2]);
                }
            }
            if (dp[i] == 0) {
                return 0;
            }
        }
        return dp[l];
    }
}
