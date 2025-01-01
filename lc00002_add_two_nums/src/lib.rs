use basic_data_structures::ListNode;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut carrier = false;
    let mut head = ListNode::new(0);
    let mut res = &mut head;
    let mut l1p = &l1;
    let mut l2p = &l2;

    while l1p.is_some() || l2p.is_some() || carrier {
        let mut cur = if carrier {1} else {0};
        
        if let Some(v) = l1p {
            cur += v.val;
            l1p = &v.next;
        }
        if let Some(v) = l2p {
            cur += v.val;
            l2p = &v.next;
        }

        carrier = cur / 10 > 0;
        cur = cur % 10;
        let next = ListNode::new(cur);
        res.next = Some(Box::new(next));
        res = res.next.as_mut().unwrap();
    }

    head.next
}

