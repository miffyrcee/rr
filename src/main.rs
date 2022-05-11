use std::cell::RefCell;

use std::rc::Rc;

#[derive(Debug)]
struct Node {
    val: String,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: char) -> Self {
        Node {
            val: "".to_string(),
            left: None,
            right: None,
        }
    }
}
fn dfs(node: Option<Rc<RefCell<Node>>>, eles: &str) -> Option<Rc<RefCell<Node>>> {
    // println!("{:?}", node);
    match node.clone() {
        None => None,
        Some(raw_node) => {
            if eles.len() == 1 {
                return Some(Rc::new(RefCell::new(Node::new(
                    eles.chars().next().unwrap(),
                ))));
            }
            let _stack: Vec<char> = Vec::new();
            let src = eles.chars().collect::<Vec<char>>();
            let mut flag = 0;
            for i in 0..src.len() {
                if src[i] == '{' {
                    flag += 1;
                } else if src[i] == '}' {
                    flag -= 1
                } else if src[i] == ',' && flag == 1 {
                    // let l = &raw_node.borrow_mut().left.unwrap();
                    let l_str = &*src[2..i].into_iter().collect::<String>();
                    let r_str = &*src[i..src.len() - 1].into_iter().collect::<String>();

                    (*raw_node.borrow_mut()).left = dfs(node.clone(), l_str);
                    (*raw_node.borrow_mut()).right = dfs(node.clone(), r_str);
                    break;
                    // println!("{:?}", raw_node);
                }
            }
            Some(raw_node)
        }
    }
}

fn main() {
    let src = r"a{b,c}";
    let mut root = Some(Rc::new(RefCell::new(Node::new(
        src.chars().next().unwrap(),
    ))));
    let root = dfs(root, src);
    println!("{:?}", root);
}
