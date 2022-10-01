import java.util.Arrays;

import static util.Asserts.assertEquals;

/**
 * 给定一个用字符数组表示的 CPU 需要执行的任务列表。其中包含使用大写的 A - Z 字母表示的26 种不同种类的任务。
 * 任务可以以任意顺序执行，并且每个任务都可以在 1 个单位时间内执行完。
 * CPU 在任何一个单位时间内都可以执行一个任务，或者在待命状态。
 * <p>
 * 然而，两个相同种类的任务之间必须有长度为 n 的冷却时间，因此至少有连续 n 个单位时间内 CPU 在执行不同的任务，或者在待命状态。
 * <p>
 * 你需要计算完成所有任务所需要的最短时间。
 * <p>
 * 示例 1：
 * <p>
 * 输入: tasks = ["A","A","A","B","B","B"], n = 2
 * 输出: 8
 * 执行顺序: A -> B -> (待命) -> A -> B -> (待命) -> A -> B.
 * 注：
 * <p>
 * 任务的总个数为 [1, 10000]。
 * n 的取值范围为 [0, 100]。
 * <p>
 * 来源：力扣（LeetCode）
 * 链接：https://leetcode-cn.com/problems/task-scheduler
 * 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 *
 * @author abomb4 2020-01-12
 */
public class Solution621 {

    private int partition(int[] array, int left, int right) {
        // 中轴数值
        int pivot = array[right];

        int i = left - 1;
        int j = right + 1;
        while (true) {
            do {
                j -= 1;
            } while (array[j] < pivot);
            do {
                i += 1;
            } while (array[i] > pivot);
            if (i < j) {
                int temp = array[j];
                array[j] = array[i];
                array[i] = temp;
            } else {
                return i;
            }
        }
    }

    /**
     * 快速排序 Hoare 版（经典快排？），倒序（大的在前）
     *
     * @param array 数组
     */
    private void qsort(int[] array, int left, int right) {
        if (left >= right) {
            return;
        }
        final int middle = partition(array, left, right);
        qsort(array, left, middle);
        qsort(array, middle, right);
    }

    public void sort(int[] array) {
        qsort(array, 0, array.length - 1);
    }

    public int leastInterval(char[] tasks, int n) {
        final int[] counts = new int[26];
        for (final char task : tasks) {
            final int index = task - 'A';
            counts[index]++;
        }
        sort(counts);
        System.out.println(Arrays.toString(counts));
        return 1;
    }

    public static void main(String[] args) {
        final Solution621 s = new Solution621();
        {
            final char[] var1 = new char[]{'A', 'A', 'A', 'B', 'B', 'B'};
            final int var2 = 2;
            final int expect = 8;
            final int result = s.leastInterval(var1, var2);
            assertEquals(expect, result, "计算1");
        }
        System.out.println("OK");
    }
}
