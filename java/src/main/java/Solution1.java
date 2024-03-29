import java.util.HashMap;

import static util.Asserts.assertEquals;

/**
 * 给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
 *
 * 你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。
 *
 * 示例:
 *
 * 给定 nums = [2, 7, 11, 15], target = 9
 *
 * 因为 nums[0] + nums[1] = 2 + 7 = 9
 * 所以返回 [0, 1]
 *
 * 来源：力扣（LeetCode）
 * 链接：https://leetcode-cn.com/problems/two-sum
 * 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 *
 * @author abomb4 2020-01-12
 */
public class Solution1 {

    public int[] twoSum(int[] nums, int target) {
        final HashMap<Integer, Integer> map = new HashMap<>(nums.length);
        for (int i = 0; i < nums.length; i++) {
            final int num = nums[i];
            final int need = target - num;
            final Integer oldIndex = map.get(need);
            if (oldIndex != null) {
                return new int[]{oldIndex, i};
            }
            map.put(num, i);
        }
        throw new IllegalArgumentException("Not found");
    }

    public static void main(String[] args) {
        final Solution1 s = new Solution1();
        final int[] result = s.twoSum(new int[]{2, 7, 11, 15}, 9);
        assertEquals(2, result.length, "result.length");
        assertEquals(0, result[0], "result[0]");
        assertEquals(1, result[1], "result[1]");
    }
}
