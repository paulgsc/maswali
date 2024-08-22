use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Node {
    val: i32,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            val,
            children: Vec::new(),
        }
    }
}

fn serialize(root: Option<Rc<RefCell<Node>>>) -> String {
    if let Some(node) = root {
        let node = node.borrow();
        let mut result = node.val.to_string();
        if !node.children.is_empty() {
            result.push('(');
            for (i, child) in node.children.iter().enumerate() {
                if i > 0 {
                    result.push(',');
                }
                result.push_str(&serialize(Some(child.clone())));
            }
            result.push(')');
        }
        result
    } else {
        String::new()
    }
}

fn deserialize(data: String) -> Option<Rc<RefCell<Node>>> {
    if data.is_empty() {
        return None;
    }

    let mut chars = data.chars().peekable();
    deserialize_helper(&mut chars)
}

fn deserialize_helper(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<Rc<RefCell<Node>>> {
    let mut val_str = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_digit(10) || c == '-' {
            val_str.push(chars.next().unwrap());
        } else {
            break;
        }
    }

    let val = val_str.parse::<i32>().unwrap();
    let node = Rc::new(RefCell::new(Node::new(val)));

    if chars.peek() == Some(&'(') {
        chars.next();
        while chars.peek() != Some(&')') {
            if let Some(child) = deserialize_helper(chars) {
                node.borrow_mut().children.push(child);
            }
            if chars.peek() == Some(&',') {
                chars.next();
            }
        }
        chars.next();
    }

    Some(node)
}




fn main() {
    let root = Rc::new(RefCell::new(Node::new(1)));
    let child1 = Rc::new(RefCell::new(Node::new(3)));
    let child2 = Rc::new(RefCell::new(Node::new(2)));
    let child3 = Rc::new(RefCell::new(Node::new(4)));
    let grandchild1 = Rc::new(RefCell::new(Node::new(5)));
    let grandchild2 = Rc::new(RefCell::new(Node::new(6)));

    root.borrow_mut().children.push(child1.clone());
    root.borrow_mut().children.push(child2);
    root.borrow_mut().children.push(child3);
    child1.borrow_mut().children.push(grandchild1);
    child1.borrow_mut().children.push(grandchild2);

    let serialized = serialize(Some(root.clone()));
    println!("Serialized tree: {}", serialized);

    let deserialized = deserialize(serialized);

    let reserialized = serialize(deserialized);
    println!("Re-serialized tree: {}", reserialized);

}
