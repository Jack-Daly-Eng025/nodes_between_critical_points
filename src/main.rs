pub struct Solution;

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
impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let (mut top, mut end, mut least, mut i, u) = (-1, -1, i32::MAX, 0, vec![-1, -1]);
        let Some(head) = head else { return u }; let mut a = head.val;
        let Some(mut curr) = head.next else { return u }; let mut b = curr.val;
        while let Some(next) = curr.next {
            if a > b && b < next.val || a < b && b > next.val {
                if top == -1 { top = i } else {
                    least = least.min(i - end.max(top));
                    end = i
                }
            }
            i += 1; a = b; b = next.val; curr = next
        }
        if end > 0 { vec![least, end - top] } else { u }
    }
}

fn main(){
    let list = vec![5,3,1,2,5,1,2];
    let mut nodes = None;
    let mut current = &mut nodes;
    for x in list {
        *current = Some(Box::new(ListNode::new(x)));
        current = &mut current.as_mut().unwrap().next;
    }
    let c_p_b_n = Solution::nodes_between_critical_points(nodes);

    for val in c_p_b_n {
        println!("{val}");
    }
}

