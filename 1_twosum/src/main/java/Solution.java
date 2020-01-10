import java.util.*;

public class Solution {

    public int[] twoSum(int[] nums, int target) {
        return new int[]{};
    }

    public static void main(String[] args) {
        final Solution s = new Solution();
        final int[] result = s.twoSum(new int[]{2, 7, 11, 15}, 9);
        assertEquals(2, result.length, "result.length");
        assertEquals(0, result[0], "result[0]");
        assertEquals(1, result[1], "result[1]");
    }

    private static void assertEquals(Object expected, Object target, String info) {
        if (!isEquals(expected, target)) {
            throw new RuntimeException("【"+info+"】目标 ["+target+"] 与期望 ["+expected+"] 不符");
        }
    }

    private static boolean isEquals(Object expected, Object target) {
        return (expected == target || Objects.equals(expected, target));
    }
}
