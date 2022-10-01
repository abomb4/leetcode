/**
 * 面试题 01.09. 字符串轮转
 * <p>
 * 字符串轮转。给定两个字符串s1和s2，请编写代码检查s2是否为s1旋转而成（比如，waterbottle是erbottlewat旋转后的字符串）。
 * <p>
 * 示例1:
 * <p>
 * 输入：s1 = "waterbottle", s2 = "erbottlewat"
 * 输出：True
 * <p>
 * 示例2:
 * <p>
 * 输入：s1 = "aa", s2 = "aba"
 * 输出：False
 * <p>
 * 提示：
 * <p>
 * 字符串长度在[0, 100000]范围内。
 * <p>
 * 说明:
 * <p>
 * 你能只调用一次检查子串的方法吗？
 * <p>
 * 来源：力扣（LeetCode）
 * 链接：https://leetcode.cn/problems/string-rotation-lcci
 * 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */
public class Solution0109 {

    public static void main(String[] args) {
        Solution0109 s = new Solution0109();
        class C {
            final String s1, s2;
            final boolean exp;

            C(String s1, String s2, boolean exp) {
                this.s1 = s1;
                this.s2 = s2;
                this.exp = exp;
            }
        }

        C[] cs = new C[]{
                new C("abcde", "eabcd", true)
        };

        for (C c : cs) {
            final boolean r = s.isFlipedString(c.s1, c.s2);
            System.out.printf("yes: %s, s1: %s, s2: %s, expect: %s, actual: %s%n",
                    r == c.exp, c.s1, c.s2, c.exp, r);
        }
    }

    public boolean isFlipedString(String s1, String s2) {
        if (s1.length() != s2.length()) {
            return false;
        }
        return (s1 + s1).contains(s2);
//        int len = s1.length();
//        for (int i = 0; i < len; i++) {
//
//        }
    }
}