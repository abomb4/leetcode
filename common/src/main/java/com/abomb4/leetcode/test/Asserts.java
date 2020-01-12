package com.abomb4.leetcode.test;

import java.util.Objects;

/**
 * @author abomb4 2020-01-12
 */
public class Asserts {

    public static void assertEquals(Object expected, Object target, String info) {
        if (!isEquals(expected, target)) {
            throw new RuntimeException("【"+info+"】目标 ["+target+"] 与期望 ["+expected+"] 不符");
        }
    }

    public static void assertNotNull(Object target, String info) {
        if (target == null) {
            throw new RuntimeException("【"+info+"】目标 ["+target+"] 是 null");
        }
    }

    public static boolean isEquals(Object expected, Object target) {
        return (expected == target || Objects.equals(expected, target));
    }
}
