import java.util.Iterator;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;

import static util.Asserts.assertArrayEquals;
import static util.Asserts.assertNotNull;

/**
 * 根据每日 气温 列表，请重新生成一个列表，对应位置的输入是你需要再等待多久温度才会升高超过该日的天数。
 * 如果之后都不会升高，请在该位置用 0 来代替。
 * <p>
 * 例如，给定一个列表 temperatures = [73, 74, 75, 71, 69, 72, 76, 73]，你的输出应该是 [1, 1, 4, 2, 1, 1, 0, 0]。
 * <p>
 * 提示：气温 列表长度的范围是 [1, 30000]。每个气温的值的均为华氏度，都是在 [30, 100] 范围内的整数。
 * <p>
 * <p>
 * 来源：力扣（LeetCode）
 * 链接：https://leetcode-cn.com/problems/daily-temperatures
 * 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 *
 * @author abomb4 2020-01-12
 */
public class Solution739 {

    public int[] dailyTemperatures(int[] t) {
        // Store temperature and index
        final int[] resultList = new int[t.length];
        final TreeMap<Integer, List<Integer>> map = new TreeMap<>();
        for (int i = 0; i < t.length; i++) {
            final int tem = t[i];
            resultList[i] = 0;
            final List<Integer> list = map.get(tem);
            if (list == null) {
                final LinkedList<Integer> l = new LinkedList<>();
                l.add(i);
                map.put(tem, l);
            } else {
                list.add(i);
            }

            final Iterator<Map.Entry<Integer, List<Integer>>> it = map.entrySet().iterator();
            while (it.hasNext()) {
                final Map.Entry<Integer, List<Integer>> entry = it.next();
                final Integer loopTem = entry.getKey();
                final List<Integer> loopIndexes = entry.getValue();
                if (tem > loopTem) {
                    for (final Integer loopIndex : loopIndexes) {
                        resultList[loopIndex] = i - loopIndex;
                    }
                    it.remove();
                } else {
                    break;
                }
            }
        }
        return resultList;
    }

    public static void main(String[] args) {
        final Solution739 s = new Solution739();
        {
            final int[] tst = new int[]{73, 74, 75, 71, 69, 72, 76, 73};
            final int[] rst = new int[]{1, 1, 4, 2, 1, 1, 0, 0};
            final int[] result = s.dailyTemperatures(tst);
            assertNotNull(result, "结果1");
            assertArrayEquals(rst, result, "计算1");
        }
        {
            final int[] tst = new int[]{89, 62, 70, 58, 47, 47, 46, 76, 100, 70};
            final int[] rst = new int[]{8, 1, 5, 4, 3, 2, 1, 1, 0, 0};
            final int[] result = s.dailyTemperatures(tst);
            assertNotNull(result, "结果2");
            assertArrayEquals(rst, result, "计算2");
        }
        System.out.println("OK");
    }
}
