import java.util.StringJoiner;

import static util.Asserts.assertArrayEquals;
import static util.Asserts.assertEquals;
import static util.Asserts.assertNotNull;

/**
 * <pre>
 * 23. 合并K个升序链表
 *
 * 难度：困难
 *
 * 给你一个链表数组，每个链表都已经按升序排列。
 *
 * 请你将所有链表合并到一个升序链表中，返回合并后的链表。
 *
 *
 *
 * 示例 1：
 *
 * 输入：lists = [[1,4,5],[1,3,4],[2,6]]
 * 输出：[1,1,2,3,4,4,5,6]
 * 解释：链表数组如下：
 * [
 *   1->4->5,
 *   1->3->4,
 *   2->6
 * ]
 * 将它们合并到一个有序链表中得到。
 * 1->1->2->3->4->4->5->6
 *
 * 示例 2：
 *
 * 输入：lists = []
 * 输出：[]
 *
 * 示例 3：
 *
 * 输入：lists = [[]]
 * 输出：[]
 *
 *
 *
 * 提示：
 *
 *     k == lists.length
 *     0 <= k <= 10^4
 *     0 <= lists[i].length <= 500
 *     -10^4 <= lists[i][j] <= 10^4
 *     lists[i] 按 升序 排列
 *     lists[i].length 的总和不超过 10^4
 *
 * 来源：力扣（LeetCode）
 * 链接：https://leetcode.cn/problems/merge-k-sorted-lists
 * 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 * </pre>
 *
 * @author abomb4 2022-10-12 22:21:22
 */
public class Solution23 {
    public ListNode mergeKLists(ListNode[] lists) {
        if (lists.length == 0) {
            return null;
        }
        SmallHeap heap = new SmallHeap(32 - Integer.numberOfLeadingZeros(lists.length));
        for (ListNode node : lists) {
            if (node != null) {
                heap.add(node);
            }
        }
        ListNode head = null;
        ListNode current = null;
        while (heap.size > 0) {
            final ListNode peek = heap.replaceTopNode();
            if (peek == null) {
                break;
            }
            if (head == null) {
                head = peek;
                current = head;
            } else {
                current.next = peek;
                current = current.next;
            }
        }
        return head;
    }

    public static class SmallHeap {
        ListNode[] base;
        int size = 0;

        SmallHeap(int level) {
            // 1: root only
            // 2: 3 nodes
            base = new ListNode[(int)Math.pow(2, level) - 1];
        }

        public void add(ListNode node) {
            base[size] = node;
            shiftUp(size);
            size += 1;
        }

        ListNode peek() {
            if (size == 0) {
                return null;
            }

            return base[0];
        }

        ListNode replaceTopNode() {
            if (size == 0) {
                return null;
            }
            ListNode top = base[0];
            ListNode next = top.next;
            if (top.next == null) {
                // remove top
                swap(0, size - 1);
                size -= 1;
                if (size > 0) {
                    shiftDown(0);
                }
            } else {
                base[0] = next;
                if (top.val != next.val) {
                    shiftDown(0);
                }
            }
            return top;
        }

        void shiftDown(int index) {
            int downTo = index;
            final int leftIndex = indexLeftChild(index);
            if (leftIndex < size && base[leftIndex].val < base[downTo].val) {
                downTo = leftIndex;
            }
            final int rightIndex = indexRightChild(index);
            if (rightIndex < size && base[rightIndex].val < base[downTo].val) {
                downTo = rightIndex;
            }
            if (downTo != index) {
                swap(downTo, index);
                shiftDown(downTo);
            }
        }

        void shiftUp(int index) {
            int parentIndex = indexParent(index);
            if (parentIndex == -1) {
                return;
            }
            int parentVal = base[parentIndex].val;
            int thisVal = base[index].val;
            if (parentVal > thisVal) {
                swap(parentIndex, index);
                shiftUp(parentIndex);
            }
        }

        void swap(int i1, int i2) {
            final ListNode tmp = base[i1];
            base[i1] = base[i2];
            base[i2] = tmp;
        }

        static int indexParent(int index) {
            return index == 0 ? -1 : (index - 1) / 2;
        }
        static int indexLeftChild(int index) {
            return index * 2 + 1;
        }
        static int indexRightChild(int index) {
            return index * 2 + 2;
        }
    }

    public static void main(String[] args) {
        final Solution23 s = new Solution23();
        {
            String in = "[[1,4,5],[1,3,4],[2,6]]";
            String expect = "[1,1,2,3,4,4,5,6]";
            final ListNode result = s.mergeKLists(toNodes(in));
            final String real = toString(result);
            assertEquals(expect, real, "in: " + in);
        }
        {
            String in = "[[]]";
            String expect = "[]";
            final ListNode result = s.mergeKLists(toNodes(in));
            final String real = toString(result);
            assertEquals(expect, real, "in: " + in);
        }
        System.out.println("OK");
    }

    static String toString(ListNode node) {
        if (node == null) {
            return "[]";
        }
        final StringJoiner sb = new StringJoiner(",", "[", "]");
        ListNode current = node;
        while (current != null) {
            sb.add(String.valueOf(current.val));
            current = current.next;
        }
        return sb.toString();
    }

    static ListNode[] toNodes(String in) {
        final String sub = in.substring(2, in.length() - 2);
        final String[] groups = sub.split("],\\[");
        final ListNode[] result = new ListNode[groups.length];
        for (int i = 0; i < groups.length; i++) {
            String group = groups[i];
            if (group == null || group.isEmpty()) {
                continue;
            }
            final String[] nums = group.split(",");
            ListNode root = null;
            ListNode current = null;
            for (String str : nums) {
                final int num = Integer.parseInt(str);
                if (root == null) {
                    root = new ListNode(num);
                    current = root;
                } else {
                    current.next = new ListNode(num);
                    current = current.next;
                }
            }
            result[i] = root;
        }
        return result;
    }

    static class ListNode {
        int val;
        ListNode next;
        ListNode() {}
        ListNode(int val) { this.val = val; }
        ListNode(int val, ListNode next) { this.val = val; this.next = next; }
    }
}

