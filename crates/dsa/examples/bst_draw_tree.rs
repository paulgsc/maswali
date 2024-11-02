use dsa::BST;

fn main() {
	let values = vec![1, 4, 5, 10, 16, 17, 21];

	for height in 2..=6 {
		let mut bst = BST::new();
		bst.root = bst.insert_with_height(&values, height);
		println!("Binary Search Tree with height {}:", height);
		println!("{}", bst);
		println!();
	}
}
