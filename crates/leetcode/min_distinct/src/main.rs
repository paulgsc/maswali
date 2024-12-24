use std::collections::HashSet;
struct Solution;

impl Solution {
	// 1. Given a set S, of integers, the set will be distinct if the there are no repeated
	//    elements or the set S is the empty set.
	//  2. If C represents a collection of integers that requies k operations to be distinct. Where
	//     k >= 0, then the following are true after k operations.
	//     a. The set R the result of k operations is equal to C iif k = 0.
	//     b. The set R is the subset of C made up of all the elements in C after the index 3 * k
	//     c. If the index 3 * k is greater than the length of C then R is the empty set.

	pub fn minimum_operations(nums: Vec<i32>) -> i32 {
		let mut set = HashSet::new();

		for i in (0..nums.len()).rev() {
			if set.contains(&nums[i]) {
				return (i as i32 + 1 + 3 - 1) / 3;
			}
			set.insert(nums[i]);
		}
		0
	}
}

fn main() {
	let nums = vec![1, 2, 2, 3, 3];
	let result = Solution::minimum_operations(nums);
	println!("Minimum number of operations: {}", result);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example_1() {
		let nums = vec![1, 2, 2, 3, 3];
		let result = Solution::minimum_operations(nums);
		assert_eq!(result, 2);
	}

	#[test]
	fn test_example_2() {
		let nums = vec![1, 1, 1, 1];
		let result = Solution::minimum_operations(nums);
		assert_eq!(result, 2);
	}

	#[test]
	fn test_empty_array() {
		let nums = vec![];
		let result = Solution::minimum_operations(nums);
		assert_eq!(result, 0);
	}

	#[test]
	fn test_all_distinct() {
		let nums = vec![1, 2, 3, 4, 5];
		let result = Solution::minimum_operations(nums);
		assert_eq!(result, 0);
	}
}
