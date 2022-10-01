/**
 * 整数转罗马数字
 * <p>
 * 罗马数字包含以下七种字符： I， V， X， L，C，D 和 M。
 * <p>
 * 字符          数值
 * I             1
 * V             5
 * X             10
 * L             50
 * C             100
 * D             500
 * M             1000
 * <p>
 * 例如， 罗马数字 2 写做 II ，即为两个并列的 1。12 写做 XII ，即为 X + II 。 27 写做  XXVII, 即为 XX + V + II 。
 * <p>
 * 通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。数字 1 在数字 5 的左边，所表示的数等于大数 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况：
 * <p>
 * I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
 * X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。
 * C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
 * <p>
 * 给你一个整数，将其转为罗马数字。
 * <p>
 * <p>
 * <p>
 * 示例 1:
 * <p>
 * 输入: num = 3
 * 输出: "III"
 * <p>
 * 示例 2:
 * <p>
 * 输入: num = 4
 * 输出: "IV"
 * <p>
 * 示例 3:
 * <p>
 * 输入: num = 9
 * 输出: "IX"
 * <p>
 * 示例 4:
 * <p>
 * 输入: num = 58
 * 输出: "LVIII"
 * 解释: L = 50, V = 5, III = 3.
 * <p>
 * 示例 5:
 * <p>
 * 输入: num = 1994
 * 输出: "MCMXCIV"
 * 解释: M = 1000, CM = 900, XC = 90, IV = 4.
 * <p>
 * <p>
 * <p>
 * 提示：
 * <p>
 * 1 <= num <= 3999
 * <p>
 * 来源：力扣（LeetCode）
 * 链接：https://leetcode.cn/problems/integer-to-roman
 * 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */
public class Solution12 {

    public static void main(String[] args) {
        Solution12 s = new Solution12();
        class C {
            final int n;
            final String e;

            C(int n, String e) {
                this.n = n;
                this.e = e;
            }
        }

        C[] cs = new C[]{
                new C(1, "I"),
                new C(3, "III"),
                new C(4, "IV"),
                new C(5, "V"),
                new C(58, "LVIII"),
                new C(1994, "MCMXCIV"),
        };

        for (C c : cs) {
            String r = s.intToRoman(c.n);
            System.out.printf("yes: %s, num: %s, result: %s, expected: %s%n",
                    c.e.equals(r), c.n, r, c.e);
        }

    }

    public String intToRoman(int num) {
        // 1 I
        // 4 IV
        // 5 V
        // 9 IX
        // 10 X
        // 40 XL
        // 50 L
        // 90 XC
        // 100 C
        // 400 CD
        // 500 D
        // 900 CM
        // 1000 M
        final Pair[] pairs = new Pair[]{
                new Pair(1, "I"),
                new Pair(4, "IV"),
                new Pair(5, "V"),
                new Pair(9, "IX"),
                new Pair(10, "X"),
                new Pair(40, "XL"),
                new Pair(50, "L"),
                new Pair(90, "XC"),
                new Pair(100, "C"),
                new Pair(400, "CD"),
                new Pair(500, "D"),
                new Pair(900, "CM"),
                new Pair(1000, "M"),
        };

        final StringBuilder sb = new StringBuilder(32);
        int p = pairs.length - 1;
        Pair pair = pairs[p];
        int tmp = num;
        while (tmp > 0) {
            if (tmp >= pair.num) {
                sb.append(pair.str);
                tmp = tmp - pair.num;
            } else {
                pair = pairs[--p];
            }
        }
        return sb.toString();
    }

    private static class Pair {
        final int num;
        final String str;

        Pair(int n, String s) {
            this.num = n;
            this.str = s;
        }
    }
}