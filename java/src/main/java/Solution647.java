import static util.Asserts.*;

/**
 * 给定一个字符串，你的任务是计算这个字符串中有多少个回文子串。
 *
 * 具有不同开始位置或结束位置的子串，即使是由相同的字符组成，也会被计为是不同的子串。
 *
 * 示例 1:
 *
 * 输入: "abc"
 * 输出: 3
 * 解释: 三个回文子串: "a", "b", "c".
 * 示例 2:
 *
 * 输入: "aaa"
 * 输出: 6
 * 说明: 6个回文子串: "a", "a", "a", "aa", "aa", "aaa".
 * 注意:
 *
 * 输入的字符串长度不会超过1000。
 *
 * 来源：力扣（LeetCode）
 * 链接：https://leetcode-cn.com/problems/palindromic-substrings
 * 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 *
 * @author abomb4 2020-01-12
 */
public class Solution647 {

    /**
     * 已知有回文的情况下，向两端寻找更长的回文
     *
     * @param chars 原始 chars
     * @param left  左侧指针
     * @param right 右侧指针
     * @return 回文次数，至少为 1
     */
    private int count(char[] chars, int left, int right) {
        int sum = 1;
        final int length = chars.length;
        while(true) {
            left = left - 1;
            right = right + 1;
            if (left < 0 || right >= length) {
                return sum;
            }
            if (chars[left] != chars[right]) {
                return sum;
            }
            sum++;
        }
    }

    public int countSubstrings(String s) {
        // 要满足回文要求，则当前元素与前一个或向前两个元素是相同的（condition 1）。
        // 满足 condition 1 的时候，向两侧扩展搜索

        int sum = s.length();

        final char[] chars = s.toCharArray();
        for (int i = 1; i < chars.length; i++) {
            final char current = chars[i];
            final int i1 = i - 1;
            final char previous = chars[i1];

            if (current == previous) {
                // 进入回文搜索状态
                sum += count(chars, i1, i);
            }

            final int i2 = i - 2;
            if (i2 >= 0) {
                final char previous2 = chars[i2];
                if (current == previous2) {
                    sum += count(chars, i2, i);
                }
            }
        }
        return sum;
    }

    public static void main(String[] args) {
        final Solution647 s = new Solution647();
        {
            final String tst = "abc";
            final int rst = 3;
            final int result = s.countSubstrings(tst);

            assertEquals(rst, result, "计算1");
        }
        {
            final String tst = "aaa";
            final int rst = 6;
            final int result = s.countSubstrings(tst);

            assertEquals(rst, result, "计算2");
        }
        {
            // 'a', 'c', 'a', 'aca'
            final String tst = "aca";
            final int rst = 4;
            final int result = s.countSubstrings(tst);

            assertEquals(rst, result, "计算3");
        }
        {
            // 8, 'ff', 'dffd', 'sdffds', 'asdffdsa'
            final String tst = "asdffdsa";
            final int rst = 12;
            final int result = s.countSubstrings(tst);

            assertEquals(rst, result, "计算4");
        }
        {
            // 5, '101', '010', '10101', '101'
            final String tst = "10101";
            final int rst = 9;
            final int result = s.countSubstrings(tst);

            assertEquals(rst, result, "计算5");
        }
        System.out.println("OK");
    }
}
