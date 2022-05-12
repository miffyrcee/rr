use std::cell::RefCell;

use std::rc::Rc;

#[derive(Debug)]
struct Node {
    val: String,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: &str) -> Self {
        Node {
            val: val.to_string(),
            left: None,
            right: None,
        }
    }
}
fn dfs(eles: &str) -> Option<Rc<RefCell<Node>>> {
    // println!("{:?}", node);
    if eles.len() == 1 {
        return Some(Rc::new(RefCell::new(Node::new(eles))));
    }
    let src = eles.chars().collect::<Vec<char>>();

    let mut node = Rc::new(RefCell::new(Node::new(src[0].to_string().as_str())));

    let mut flag = 0;
    for i in 0..src.len() {
        if src[i] == '{' {
            flag += 1;
        } else if src[i] == '}' {
            flag -= 1;
            if flag == 0 {
                (*node.borrow_mut()).left = dfs(src[i - 1].to_string().as_str());
            }
        } else if src[i] == ',' && flag == 1 {
            // let l = &raw_node.borrow_mut().left.unwrap();
            if i > 2 {
                let l_str = &*src[2..i].iter().collect::<String>();
                (*node.borrow_mut()).left = dfs(l_str);
            }
            if src.len() - 1 > i + 1 {
                let r_str = &*src[i + 1..src.len() - 1].into_iter().collect::<String>();
                (*node.borrow_mut()).right = dfs(r_str);
            }
            break;
            // println!("{:?}", raw_node);
        }
    }
    Some(node)
}

fn main() {
    let src = r"a{b,c}";
    let root = dfs(&src);
    println!("{:?}", root);
}
