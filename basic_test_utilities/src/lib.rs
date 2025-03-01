use basic_data_structures::ListNode;

mod macros {
#[macro_export]
macro_rules! process_params {
    ($dict:expr, $name:expr, $($type:tt)*) => {{
        let converter = type_converter!($($type)*);
        converter(&$dict[$name])
    }};
}

#[macro_export]
macro_rules! type_converter {
    (String) => {
        |val: &Value| -> String {
            val.as_str().unwrap().to_string()
        }
    };
    (bool) => {
        |val: &Value| -> bool {
            val.as_bool().unwrap()
        }
    };
    (i32) => {
        |val: &Value| -> i32 {
            val.as_i64().unwrap() as i32
        }
    };
    (Vec<$inner:tt>) => {
        |val: &Value| -> Vec<$inner> {
            val.as_array()
                .unwrap()
                .iter()
                .map(|v| {
                    let converter = type_converter!($inner);
                    converter(v)
                })
                .collect()
        }
    };
    (Vec<Vec<$inner:tt>>) => {
        |val: &Value| -> Vec<Vec<$inner>> {
            val.as_array()
                .unwrap()
                .iter()
                .map(|v| {
                    let converter = type_converter!(Vec<$inner>);
                    converter(v)
                })
                .collect()
        }
    };
    (HashMap < $k:tt, $v:tt >) => {
        |val: &Value| -> std::collections::HashMap<$k, $v> {
            val.as_object()
                .unwrap()
                .iter()
                .map(|(key, value)| {
                    let k_converter = type_converter!($k);
                    let v_converter = type_converter!($v);
                    (k_converter(key), v_converter(value))
                })
                .collect()
        }
    };
}
}

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
