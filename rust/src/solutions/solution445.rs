//
// 445. 两数相加 II
//
// 给你两个 非空 链表来代表两个非负整数。数字最高位位于链表开始位置。
// 它们的每个节点只存储一位数字。将这两数相加会返回一个新的链表。
//
// 你可以假设除了数字 0 之外，这两个数字都不会以零开头。
//
// 进阶：
//
// 如果输入链表不能修改该如何处理？换句话说，你不能对列表中的节点进行翻转。
//
// 示例：
//
// 输入：(7 -> 2 -> 4 -> 3) + (5 -> 6 -> 4)
// 输出：7 -> 8 -> 0 -> 7
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/add-two-numbers-ii
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
//

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn to_string(&self) -> String {
        let mut nums: Vec<i32> = Vec::new();
        self._to_string(&mut nums);
        let strs: Vec<_> = nums.iter().map(|v| { v.to_string() }).collect();
        strs.join(" -> ")
    }

    fn _to_string(&self, vec: &mut Vec<i32>) {
        vec.push(self.val);
        match &self.next {
            Some(next) => next._to_string(vec),
            None => ()
        }
    }
}

pub struct Solution;

impl Solution {
    fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // We must know the length
        let mut vector1 = Vec::with_capacity(100);
        let mut vector2 = Vec::with_capacity(100);

        let mut current_l1 = l1;
        loop {
            match current_l1 {
                Some(node) => {
                    vector1.push(node.val);
                    current_l1 = node.next;
                }
                None => break
            }
        }
        let mut current_l2 = l2;
        loop {
            match current_l2 {
                Some(node) => {
                    vector2.push(node.val);
                    current_l2 = node.next;
                }
                None => break
            }
        }

        let mut pop1 = vector1.pop();
        let mut pop2 = vector2.pop();
        let mut plus = 0;
        let mut next_may_means_previous = None;
        loop {
            let sum;
            match (pop1, pop2) {
                (Some(num1), Some(num2)) => sum = num1 + num2 + plus,
                (Some(num), None) | (None, Some(num)) => sum = num + plus,
                (None, None) => break
            }

            plus = sum / 10;
            let new_node = Some(Box::new(ListNode { val: sum % 10, next: next_may_means_previous }));
            next_may_means_previous = new_node;
            pop1 = vector1.pop();
            pop2 = vector2.pop();
        }
        if plus > 0 {
            Some(Box::new(ListNode { val: plus, next: next_may_means_previous }))
        } else {
            next_may_means_previous
        }
    }
}

pub fn test() {
    let mut all_true = true;
    {
        let case1 = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: None,
                    })),
                })),
            })),
        }));

        let case2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None,
                })),
            })),
        }));
        let str1 = case1.as_ref().map(|v| { v.to_string() }).unwrap();
        let str2 = case2.as_ref().map(|v| { v.to_string() }).unwrap();

        let resp = Solution::add_two_numbers(case1, case2);
        let resp_str = resp.unwrap().to_string();

        let expe = "7 -> 8 -> 0 -> 7";
        let success = resp_str == String::from(expe);
        if !success { all_true = false; }
        println!("[1] l1: {0}, l2: {1}, resp: {2}, expect: {3}, success: {4}",
                 str1, str2, resp_str, expe, success);
    }
    match all_true {
        true => println!("solution445 success"),
        false => println!("solution445 failed"),
    }
}
