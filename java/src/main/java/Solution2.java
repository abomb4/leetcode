import static util.Asserts.assertEquals;
import static util.Asserts.assertNotNull;

/**
 * 给出两个非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照逆序的方式存储的，并且它们的每个节点只能存储一位数字。
 * <p>
 * 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
 * <p>
 * 您可以假设除了数字 0 之外，这两个数都不会以 0开头。
 * <p>
 * 示例：
 * <p>
 * 输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
 * 输出：7 -> 0 -> 8
 * 原因：342 + 465 = 807
 * <p>
 * <p>
 * 来源：力扣（LeetCode）
 * 链接：https://leetcode-cn.com/problems/add-two-numbers
 * 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 *
 * @author abomb4 2020-01-12
 */
public class Solution2 {

    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
        ListNode r = null;
        ListNode current = r;

        boolean plusOne = false;
        ListNode n1 = l1;
        ListNode n2 = l2;
        while (n1 != null || n2 != null) {

            int v1, v2;
            if (n1 == null) {
                v1 = 0;
            } else {
                v1 = n1.val;
                n1 = n1.next;
            }

            if (n2 == null) {
                v2 = 0;
            } else {
                v2 = n2.val;
                n2 = n2.next;
            }

            int sum = v1 + v2;
            if (plusOne) {
                sum += 1;
            }
            int toPush;
            if (sum > 9) {
                toPush = sum % 10;
                plusOne = true;
            } else {
                toPush = sum;
                plusOne = false;
            }

            if (r == null) {
                r = new ListNode(toPush);
                current = r;
            } else {
                current.next = new ListNode(toPush);
                current = current.next;
            }
        }

        if (plusOne) {
            current.next = new ListNode(1);
        }

        return r;
    }

    public static class ListNode {
        int val;
        ListNode next;

        ListNode(int x) {
            val = x;
        }
    }

    public static void main(String[] args) {
        final Solution2 s = new Solution2();
        final ListNode l1 = new ListNode(2);
        {
            l1.next = new ListNode(4);
            l1.next.next = new ListNode(3);
        }
        final ListNode l2 = new ListNode(5);
        {
            l2.next = new ListNode(6);
            l2.next.next = new ListNode(4);
        }
        final ListNode result = s.addTwoNumbers(l1, l2);
        assertNotNull(result, "结果");
        assertNotNull(result.next, "结果.next");
        assertNotNull(result.next.next, "结果.next.next");
        assertEquals(7, result.val, "Node 1");
        assertEquals(0, result.next.val, "Node 2");
        assertEquals(8, result.next.next.val, "Node 3");
        System.out.println("OK");
    }
}
