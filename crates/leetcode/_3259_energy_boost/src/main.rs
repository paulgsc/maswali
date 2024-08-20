fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i32 {
    let n = energy_drink_a.len(); 
    if n == 0 {
        return 0;
    }

    let mut dp_a = vec![0; n];
    let mut dp_b = vec![0; n];

    dp_a[0] = energy_drink_a[0];
    dp_b[0] = energy_drink_b[0];

    if n > 1 {
        dp_a[1] = std::cmp::max(energy_drink_a[1], dp_b[0] + energy_drink_a[1]);
        dp_b[1] = std::cmp::max(energy_drink_b[1], dp_a[0] + energy_drink_b[1]);
    }


    for i in 2..n {
        dp_a[i] = std::cmp::max(dp_a[i-1] + energy_drink_a[i], dp_b[i-2] + energy_drink_a[i]);
        dp_b[i] = std::cmp::max(dp_b[i-1] + energy_drink_b[i], dp_a[i-2] + energy_drink_b[i]);
    }

    std::cmp::max(dp_a[n-1], dp_b[n-1])
}





fn main() {
    let energy_drink_a = vec![1, 3, 1];
    let energy_drink_b = vec![3, 1, 1];

    let result = max_energy_boost(energy_drink_a, energy_drink_b);
    println!("Maximum total energy boost: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let energy_drink_a = vec![1, 3, 5, 4, 8];
        let energy_drink_b = vec![2, 2, 10, 1, 6];
        assert_eq!(max_energy_boost(energy_drink_a, energy_drink_b), 22);
    }

    #[test]
    fn test_empty_arrays() {
        let energy_drink_a: Vec<i32> = vec![];
        let energy_drink_b: Vec<i32> = vec![];
        assert_eq!(max_energy_boost(energy_drink_a, energy_drink_b), 0);
    }

    #[test]
    fn test_single_element_arrays() {
        let energy_drink_a = vec![7];
        let energy_drink_b = vec![9];
        assert_eq!(max_energy_boost(energy_drink_a, energy_drink_b), 9);
    }

    #[test]
    fn test_all_values_same() {
        let energy_drink_a = vec![5, 5, 5, 5];
        let energy_drink_b = vec![5, 5, 5, 5];
        assert_eq!(max_energy_boost(energy_drink_a, energy_drink_b), 20);
    }

    #[test]
    fn test_no_switching_needed() {
        let energy_drink_a = vec![10, 10, 10, 10, 10];
        let energy_drink_b = vec![5, 5, 5, 5, 5];
        assert_eq!(max_energy_boost(energy_drink_a, energy_drink_b), 50);
    }

    #[test]
    fn test_large_input() {
        let n = 1000;
        let energy_drink_a: Vec<i32> = vec![3; n];
        let energy_drink_b: Vec<i32> = vec![5; n];
        assert_eq!(max_energy_boost(energy_drink_a, energy_drink_b), 5000);
    }
}


