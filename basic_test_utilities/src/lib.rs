use basic_data_structures::ListNode;

pub fn list_to_nodes(nums: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;
    for &num in nums {
        *current = Some(Box::new(ListNode::new(num))); 
        current = &mut (*current).as_mut().unwrap().next;
    }
    head
}

pub fn nodes_to_list(mut node: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = vec![];
    while node.is_some() {
        let cur_node = node.as_ref().unwrap();
        result.push(cur_node.val);
        node = &cur_node.next;
    }
    result
}

