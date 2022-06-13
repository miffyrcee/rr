use std::borrow::Borrow;
use std::cell::RefCell;

use std::mem::replace;
use std::rc::Rc;

use std::collections::LinkedList;


#[derive(Debug, Clone)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}

type Link = Option<Rc<RefCell<Node>>>;
type BorrowLink<'a> = Option<&'a Rc<RefCell<Node>>>;

impl Node {
    fn new(val: i32) -> Self {
        Node { val, next: None }
    }
    fn next(&mut self) -> Link {
        if self.next.is_none() {
            return None;
        } else {
            return self.next.clone();
        }
    }
}
fn arr_to_linked(arr: Vec<i32>) -> Link {
    if arr.len() == 0 {
        return None;
    } else {
        let head = Rc::new(RefCell::new(Node::new(arr[0])));
        if arr.len() >= 1 {
            (*head.borrow_mut()).next = arr_to_linked(arr[1..].to_vec());
        }
        Some(head)
    }
}

// fn merge(mut left: Link, mut right: Link) -> Link {
//     if left.as_ref().is_none() {
//         return right;
//     }
//     if right.as_ref().is_none() {
//         return left;
//     }
//     // let tmp = Node::new(-1);
//     loop {
//         if left.unwrap().borrow().val < right.as_ref().unwrap().borrow().val {
//             left = left.as_ref().unwrap().borrow().next.clone();
//         }
//     }
//     merge(left, right)
// }
fn merge_sort(head: Link) {
    if head.as_ref().is_none() {
        return;
    }
    // let kk = head.borrow().unwrap().next.clone();
    // let mut mid = None;
    let mut right = head;
    replace(None, right)
    // unsafe {
    //     let src = right.unwrap().as_ref().as_ptr();
    //     replace(None, src)
    // }
    // replace(right.unwrap().borrow().as_ref(), right)
}

fn main() {
    let mut arr = vec![10, 3, 2, 4, 7, 6, 7, 8, 9, 10];
    let mut head = arr_to_linked(arr);
    merge_sort(head);
    // println!("{:?}", head);
}
