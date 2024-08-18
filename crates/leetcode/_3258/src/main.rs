pub fn count_k_constraint_substrings(s: &str, k: usize) -> usize {
    let n = s.len();
    let mut count = 0;

    for i in 0..n {
        let mut zeros = 0;
        let mut ones = 0;


        for j in i..n {
            match s.chars().nth(j).unwrap() {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => (),
            }

            if zeros <= k || ones <= k {
                count += 1;
            } else {
                break;
            }
        }
    }
    count 
}

fn main() {
    let binary_string = "00110";
    let k = 1;

    let result = count_k_constraint_substrings(binary_string, k);
    println!("Number of substrings that satisify the k-constraint: {}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_k_constraint_substrings() {
        assert_eq!(count_k_constraint_substrings("10101", 1), 12);
        assert_eq!(count_k_constraint_substrings("0001", 2), 10);
        assert_eq!(count_k_constraint_substrings("111", 0), 3);
        assert_eq!(count_k_constraint_substrings("101", 1), 6);
        assert_eq!(count_k_constraint_substrings("0110", 2), 9);
    }
}

