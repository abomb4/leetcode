import java.util.Arrays;

/**
 * @author abomb4 2022-09-26
 */
public class Solution1719 {

    public static void main(String[] args) {
        Solution1719 s = new Solution1719();

        System.out.println(Arrays.toString(s.missingTwo(new int[]{1})));
    }

    public int[] missingTwo(int[] nums) {
        final int n = nums.length + 2;
        int sum = 0;
        for (int i : nums) {
            sum = sum ^ i;
        }
        for (int i = 1; i <= n; i++) {
            sum = sum ^ i;
        }
        // sum = a ^ b
        final int specialBit = sum & -sum;
        int s1 = 0, s2 = 0;
        for (int i : nums) {
            if ((i & specialBit) == specialBit) {
                s1 = s1 ^ i;
            } else {
                s2 = s2 ^ i;
            }
        }
        for (int i = 1; i <= n; i++) {
            if ((i & specialBit) == specialBit) {
                s1 = s1 ^ i;
            } else {
                s2 = s2 ^ i;
            }
        }
        return new int[]{s1, s2};
    }
//    public int[] missingTwo(int[] nums) {
//        final int length = nums.length * 2;
//        final int totalSum = (length + 1) * length / 2;
//        final int totalSum2;
//        {
//            int sum = 1;
//            for (int i = 2; i <= length; i++) {
//                sum += i * i;
//            }
//            totalSum2 = sum;
//        }
//        // a + b
//        final int m;
//        // aa + bb
//        final int n;
//        {
//            int sum1 = 0, sum2 = 0;
//            for (int i : nums) {
//                sum1 += i;
//                sum2 += i * i;
//            }
//            m = totalSum - sum1;
//            n = totalSum2 - sum2;
//        }
//        // (a+b)^2 = aa +2ab + bb = m^2
//        // aa + bb = n
//        // 2ab = m^2 - n
//        // b = (mm - n) / 2a
//        // a = (mm - n) / 2b
//        // (mm - n)^2 / 4bb + bb = n
//    }
}
