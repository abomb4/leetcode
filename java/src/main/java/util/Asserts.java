package util;

import java.util.Arrays;
import java.util.Objects;

/**
 * @author abomb4 2020-01-12
 */
public class Asserts {

    public static void assertEquals(Object expected, Object target, String info) {
        if (isNotEquals(expected, target)) {
            throw new RuntimeException("【" + info + "】目标 [" + target + "] 与期望 [" + expected + "] 不符");
        }
    }

    public static void assertArrayEquals(int[] expected, int[] target, String info) {
        if (expected == null || target == null) {
            throw new RuntimeException("【" + info + "】两个 array 中存在 null");
        }
        if (isNotEquals(expected.length, target.length)) {
            throw new RuntimeException("【" + info + "】目标 " + Arrays.toString(target)
                    + " 与期望 " + Arrays.toString(expected) + " 长度不符");
        }

        for (int i = 0; i < expected.length; i++) {
            if (isNotEquals(expected[i], target[i])) {
                throw new RuntimeException("【" + info + "】目标 " + Arrays.toString(target)
                        + " 与期望 " + Arrays.toString(expected) + " 的第 [" + i + "] 项不符");
            }
        }
    }

    public static <T> void assertArrayEquals(T[] expected, T[] target, String info) {
        if (expected == null || target == null) {
            throw new RuntimeException("【" + info + "】两个 array 中存在 null");
        }
        if (isNotEquals(expected.length, target.length)) {
            throw new RuntimeException("【" + info + "】目标 " + Arrays.toString(target)
                    + " 与期望 " + Arrays.toString(expected) + " 长度不符");
        }

        for (int i = 0; i < expected.length; i++) {
            if (isNotEquals(expected[i], target[i])) {
                throw new RuntimeException("【" + info + "】目标 " + Arrays.toString(target)
                        + " 与期望 " + Arrays.toString(expected) + " 的第 [" + i + "] 项不符");
            }
        }
    }

    public static void assertNotNull(Object target, String info) {
        if (target == null) {
            throw new RuntimeException("【" + info + "】目标 [" + target + "] 是 null");
        }
    }

    public static boolean isNotEquals(Object expected, Object target) {
        return (expected != target && !Objects.equals(expected, target));
    }
}
