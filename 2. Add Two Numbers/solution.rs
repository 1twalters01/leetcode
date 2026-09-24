#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry: bool = false;
        let mut vec_repr: Vec<i32> = Vec::new();

        let mut l1 = l1;
        let mut l2 = l2;

        loop {
            match (l1.take(), l2.take()) {
                (Some(node_1), Some(node_2)) => {
                    let val_1 = node_1.val;
                    let val_2 = node_2.val;
                    
                    let ans = if carry {
                        val_1 + val_2 + 1 
                    } else {
                        val_1 + val_2
                    };

                    if ans >= 10 {
                        carry = true;
                    } else {
                        carry = false;
                    }

                    vec_repr.push(ans % 10);

                    l1 = node_1.next;
                    l2 = node_2.next;
                },
                (Some(node_1), None) => {
                    let val_1 = node_1.val;

                    let ans = if carry {
                        val_1 + 1
                    } else {
                        val_1
                    };

                    if ans >= 10 {
                        carry = true;
                    } else {
                        carry = false;
                    }

                    vec_repr.push(ans % 10);

                    l1 = node_1.next;
                },
                (None, Some(node_2)) => {
                    let val_2 = node_2.val;

                    let ans = if carry {
                        val_2 + 1
                    } else {
                        val_2
                    };

                    if ans >= 10 {
                        carry = true;
                    } else {
                        carry = false;
                    }

                    vec_repr.push(ans % 10);

                    l2 = node_2.next;
                },
                (None, None) => {
                    if carry {
                        vec_repr.push(1);
                    }

                    break
                },
            }
        }

        let mut response = Some(Box::new(ListNode::new(*vec_repr.last().unwrap_or(&0))));
        for &val in vec_repr[..vec_repr.len - 1].iter().rev() {
            let mut node = ListNode::new(val);
            node.next = response;
            response = Some(Box::new(node));
        }
        return response
    }
}
