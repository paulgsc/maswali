use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();

	let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

	for _ in 0..t {
		let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

		let mut adjacency_matrix = vec![vec![0; n]; n];
		for i in 0..n {
			adjacency_matrix[i] = lines.next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
		}

		let adjacency_list = adjacency_matrix_to_list(&adjacency_matrix);
		for neighbors in adjacency_list {
			let line = neighbors.iter().map(|&v| v.to_string()).collect::<Vec<_>>().join(" ");
			println!("{}", line);
		}
	}
}

fn adjacency_matrix_to_list(matrix: &[Vec<i32>]) -> Vec<Vec<usize>> {
	let n = matrix.len();
	let mut adjacency_list = vec![Vec::new(); n];

	for i in 0..n {
		for j in 0..n {
			if matrix[i][j] == 1 {
				adjacency_list[i].push(j);
			}
		}
	}

	adjacency_list
}
