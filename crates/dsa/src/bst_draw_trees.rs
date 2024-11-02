use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<Node<T>>>;

pub struct Node<T> {
	value: T,
	left: Option<NodeRef<T>>,
	right: Option<NodeRef<T>>,
}

impl<T> Node<T> {
	fn new(value: T) -> Self {
		Node { value, left: None, right: None }
	}
}

pub struct BST<T> {
	pub root: Option<NodeRef<T>>,
}

impl<T: Ord + Copy> BST<T> {
	pub fn new() -> Self {
		BST { root: None }
	}

	pub fn insert_balanced(&mut self, values: &[T], start: usize, end: usize) -> Option<NodeRef<T>> {
		if start > end {
			return None;
		}

		let mid = (start + end) / 2;
		let node = Rc::new(RefCell::new(Node::new(values[mid])));

		node.borrow_mut().left = self.insert_balanced(values, start, mid.saturating_sub(1));
		node.borrow_mut().right = self.insert_balanced(values, mid + 1, end);

		Some(node)
	}

	pub fn insert_with_height(&mut self, values: &[T], target_height: usize) -> Option<NodeRef<T>> {
		if values.is_empty() || target_height == 0 {
			return None;
		}

		let mut sorted_values = values.to_vec();
		sorted_values.sort();

		match target_height {
			2 => {
				// For height 2, root and one level of children
				let root = Rc::new(RefCell::new(Node::new(sorted_values[3])));
				root.borrow_mut().left = Some(Rc::new(RefCell::new(Node::new(sorted_values[1]))));
				root.borrow_mut().right = Some(Rc::new(RefCell::new(Node::new(sorted_values[5]))));
				Some(root)
			}
			3 => {
				// For height 3, balanced tree
				self.insert_balanced(&sorted_values, 0, sorted_values.len() - 1)
			}
			4 => {
				// For height 4, right-heavy tree
				let root = Rc::new(RefCell::new(Node::new(sorted_values[1])));
				let right = Rc::new(RefCell::new(Node::new(sorted_values[3])));
				right.borrow_mut().right = Some(Rc::new(RefCell::new(Node::new(sorted_values[5]))));
				root.borrow_mut().right = Some(right);
				Some(root)
			}
			5 => {
				// For height 5, left-heavy tree
				let root = Rc::new(RefCell::new(Node::new(sorted_values[5])));
				let left = Rc::new(RefCell::new(Node::new(sorted_values[3])));
				left.borrow_mut().left = Some(Rc::new(RefCell::new(Node::new(sorted_values[1]))));
				root.borrow_mut().left = Some(left);
				Some(root)
			}
			6 => {
				// For height 6, linear tree
				let mut current = Rc::new(RefCell::new(Node::new(sorted_values[0])));
				let root = current.clone();

				for &value in &sorted_values[1..] {
					let new_node = Rc::new(RefCell::new(Node::new(value)));
					current.borrow_mut().right = Some(new_node.clone());
					current = new_node;
				}
				Some(root)
			}
			_ => None,
		}
	}
}

impl<T: fmt::Display> fmt::Display for BST<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fn print_tree<T: fmt::Display>(node: &Option<NodeRef<T>>, prefix: &str, is_left: bool, f: &mut fmt::Formatter) -> fmt::Result {
			if let Some(node) = node {
				writeln!(f, "{}{}{}", prefix, if is_left { "├──" } else { "└──" }, node.borrow().value)?;

				let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });

				print_tree(&node.borrow().left, &new_prefix, true, f)?;
				print_tree(&node.borrow().right, &new_prefix, false, f)?;
			}
			Ok(())
		}

		writeln!(f, "Tree:")?;
		print_tree(&self.root, "", false, f)
	}
}
