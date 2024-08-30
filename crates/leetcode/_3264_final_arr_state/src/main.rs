pub fn get_final_state(nums: &Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    let mut nums_copy = nums.clone();

    for _ in 0..k {
        if let Some((min_index, _)) = nums_copy
            .iter()
                .enumerate()
                .min_by_key(|&(_, &val)| val)
                {
                    nums_copy[min_index] *= multiplier;
                }
    }
    nums_copy
}


fn main() {
    let nums = vec![3, 2, 1, 4, 5];
    let k = 5;
    let multiplier = 2;

    let result = get_final_state(&nums, k, multiplier);
    println!("Final nums: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![2, 1, 3, 5, 6];
        let k = 5;
        let multiplier = 2;
        let result = get_final_state(&nums, k, multiplier);
        assert_eq!(result, vec![8, 4, 6, 5, 6]);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 1, 1, 1];
        let k = 2;
        let multiplier = 3;
        let result = get_final_state(&nums, k, multiplier);
        assert_eq!(result, vec![3, 3, 1, 1]);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![5, 6, 7, 8];
        let k = 1;
        let multiplier = 4;
        let result = get_final_state(&nums, k, multiplier);
        assert_eq!(result, vec![20, 6, 7, 8]);
    }

    #[test]
    fn test_example_4() {
        let nums = vec![100];
        let k = 1;
        let multiplier = 5;
        let result = get_final_state(&nums, k, multiplier);
        assert_eq!(result, vec![500]);
    }

    #[test]
    fn test_example_5() {
        let nums = vec![2, 2, 2];
        let k = 3;
        let multiplier = 5;
        let result = get_final_state(&nums, k, multiplier);
        assert_eq!(result, vec![10, 10, 10]);
    }
}

