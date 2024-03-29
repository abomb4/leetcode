/**
 * 8. 字符串转换整数 (atoi)
 * <p>
 * 请你来实现一个 myAtoi(string s) 函数，使其能将字符串转换成一个 32 位有符号整数（类似 C/C++ 中的 atoi 函数）。
 * <p>
 * 函数 myAtoi(string s) 的算法如下：
 * <p>
 * 读入字符串并丢弃无用的前导空格
 * 检查下一个字符（假设还未到字符末尾）为正还是负号，读取该字符（如果有）。 确定最终结果是负数还是正数。 如果两者都不存在，则假定结果为正。
 * 读入下一个字符，直到到达下一个非数字字符或到达输入的结尾。字符串的其余部分将被忽略。
 * 将前面步骤读入的这些数字转换为整数（即，"123" -> 123， "0032" -> 32）。如果没有读入数字，则整数为 0 。必要时更改符号（从步骤 2 开始）。
 * 如果整数数超过 32 位有符号整数范围 [−231,  231 − 1] ，需要截断这个整数，使其保持在这个范围内。具体来说，小于 −231 的整数应该被固定为 −231 ，大于 231 − 1 的整数应该被固定为 231 − 1 。
 * 返回整数作为最终结果。
 * <p>
 * 注意：
 * <p>
 * 本题中的空白字符只包括空格字符 ' ' 。
 * 除前导空格或数字后的其余字符串外，请勿忽略 任何其他字符。
 * <p>
 * <p>
 * <p>
 * 示例 1：
 * <p>
 * 输入：s = "42"
 * 输出：42
 * 解释：加粗的字符串为已经读入的字符，插入符号是当前读取的字符。
 * 第 1 步："42"（当前没有读入字符，因为没有前导空格）
 * ^
 * 第 2 步："42"（当前没有读入字符，因为这里不存在 '-' 或者 '+'）
 * ^
 * 第 3 步："42"（读入 "42"）
 * ^
 * 解析得到整数 42 。
 * 由于 "42" 在范围 [-231, 231 - 1] 内，最终结果为 42 。
 * <p>
 * 示例 2：
 * <p>
 * 输入：s = "   -42"
 * 输出：-42
 * 解释：
 * 第 1 步："   -42"（读入前导空格，但忽视掉）
 * ^
 * 第 2 步："   -42"（读入 '-' 字符，所以结果应该是负数）
 * ^
 * 第 3 步："   -42"（读入 "42"）
 * ^
 * 解析得到整数 -42 。
 * 由于 "-42" 在范围 [-231, 231 - 1] 内，最终结果为 -42 。
 * <p>
 * 示例 3：
 * <p>
 * 输入：s = "4193 with words"
 * 输出：4193
 * 解释：
 * 第 1 步："4193 with words"（当前没有读入字符，因为没有前导空格）
 * ^
 * 第 2 步："4193 with words"（当前没有读入字符，因为这里不存在 '-' 或者 '+'）
 * ^
 * 第 3 步："4193 with words"（读入 "4193"；由于下一个字符不是一个数字，所以读入停止）
 * ^
 * 解析得到整数 4193 。
 * 由于 "4193" 在范围 [-231, 231 - 1] 内，最终结果为 4193 。
 * <p>
 * <p>
 * <p>
 * 提示：
 * <p>
 * 0 <= s.length <= 200
 * s 由英文字母（大写和小写）、数字（0-9）、' '、'+'、'-' 和 '.' 组成
 * <p>
 * 来源：力扣（LeetCode）
 * 链接：https://leetcode.cn/problems/string-to-integer-atoi
 * 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 *
 * @author abomb4 2022-09-27
 */
public class Solution8 {
    public static void main(String[] args) {
        Solution8 s = new Solution8();
        class Case {
            String in;
            int expected;

            public Case(String in, int expected) {
                this.in = in;
                this.expected = expected;
            }
        }

        final Case[] cases = new Case[]{
                new Case("   -1234321_2321", -1234321),
                new Case("-6147483648", -2147483648),
                new Case("2147483648", 2147483647),
        };

        for (Case c : cases) {
            int result = s.myAtoi(c.in);
            System.out.printf("(%s) in: %s, real: %d, expected: %d%n",
                    result == c.expected, c.in, result, c.expected);
        }
    }

    public int myAtoi(String s) {
        // 0: find spaces or '-' or number
        // 1: find numbers
        int resolve = 0;
        boolean first = true;
        boolean negative = false;
        int sum = 0;
        int length = s.length();
        for (int i = 0; i < length; i++) {
            char c = s.charAt(i);
            if (resolve == 0) {
                if (c == ' ') {
                    continue;
                } else if (c == '-') {
                    negative = true;
                    resolve = 1;
                } else if (c == '+') {
                    resolve = 1;
                } else if (Character.isDigit(c)) {
                    resolve = 1;
                    first = false;
                    sum = toNum(c);
                } else {
                    return 0;
                }
            } else if (Character.isDigit(c)) {
                int num = toNum(c);
                if (first) {
                    if (num == 0) {
                        continue;
                    }
                    sum = negative ? -num : num;
                    first = false;
                } else {
                    final int newSum;
                    if (negative) {
                        newSum = sum * 10 - num;
                        if (newSum > 0 || (newSum + num) / 10 != sum) {
                            // overflow
                            return Integer.MIN_VALUE;
                        }
                    } else {
                        newSum = sum * 10 + num;
                        if (newSum < 0 || (newSum - num) / 10 != sum) {
                            return Integer.MAX_VALUE;
                        }
                    }
                    sum = newSum;
                }
            } else {
                // not num
                break;
            }
        }
        return sum;
    }

    private static int toNum(char c) {
        return c - '0';
    }
}
