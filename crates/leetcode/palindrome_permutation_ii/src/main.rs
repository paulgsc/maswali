use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn generate_palindromic_permutations(s: String) -> Vec<String> {
        let char_counts = Self::count_chars(&s);
        let mut odd_char = None;
        let mut half_palindrome = String::new();

        for (ch, count) in char_counts.iter() {
            if count % 2 != 0 {
                if odd_char.is_some() {
                    return Vec::new();
                }
                odd_char = Some(*ch);
            }
            half_palindrome.push_str(&ch.to_string().repeat(count / 2));
        }

        let mut results = Vec::new();
        let mut half_chars: Vec<char> = half_palindrome.chars().collect();
        half_chars.sort_unstable();

        Self::generate_permutations(&mut half_chars, 0, &odd_char, &mut results);
        results
    }

    fn count_chars(s: &str) -> HashMap<char, usize> {
        s.chars().fold(HashMap::new(), |mut map, ch| {
            *map.entry(ch).or_insert(0) += 1;
            map
        })
    }

    fn generate_permutations(chars: &mut Vec<char>, start: usize, odd_char: &Option<char>, results: &mut Vec<String>) {
        if start == chars.len() {
            let half = chars.iter().collect::<String>();
            let mut palindrome = half.clone();
            if let Some(ch) = odd_char {
                palindrome.push(*ch);
            }
            palindrome.push_str(&half.chars().rev().collect::<String>());
            results.push(palindrome);
            return;
        }

        let mut used = std::collections::HashSet::new();
        for i in start..chars.len() {
            if used.contains(&chars[i]) {
                continue;
            }
            used.insert(chars[i]);
            chars.swap(start, i);
            Self::generate_permutations(chars, start + 1, odd_char, results);
            chars.swap(start, i);
        }
    }
}





fn main() {
    let test_cases = vec![
        String::from("aabb"),
        String::from("abc"),
        String::from("aaaaaa"),
    ];

    for s in test_cases {
        println!("Input {}", s);
        let result = Solution::generate_palindromic_permutations(s);
        println!("Output: {:?}\n", result);
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn check_palindrome(s: &str) -> bool {
        s.chars().eq(s.chars().rev())
    }

    fn are_permutations(s1: &str, s2: &str) -> bool {
        let mut chars1: Vec<char> = s1.chars().collect();
        let mut chars2: Vec<char> = s2.chars().collect();
        chars1.sort_unstable();
        chars2.sort_unstable();
        chars1 == chars2
    }

    #[test]
    fn test_empty_string() {
        let result = Solution::generate_palindromic_permutations(String::new());
        assert_eq!(result, vec!["".to_string()]);
    }

    #[test]
    fn test_single_character() {
        let result = Solution::generate_palindromic_permutations("a".to_string());
        assert_eq!(result, vec!["a".to_string()]);
    }

    #[test]
    fn test_two_same_characters() {
        let result = Solution::generate_palindromic_permutations("aa".to_string());
        assert_eq!(result, vec!["aa".to_string()]);
    }

    #[test]
    fn test_two_different_characters() {

        let result = Solution::generate_palindromic_permutations("ab".to_string());
        assert!(result.is_empty());
    }

    #[test]
    fn test_aabb() {
        let result = Solution::generate_palindromic_permutations("aabb".to_string());
        let expected: HashSet<String> = ["abba".to_string(), "baab".to_string()].into_iter().collect();
        assert_eq!(result.len(), 2);
        assert!(result.into_iter().all(|s| expected.contains(&s)));
    }

    #[test]
    fn test_aabbc() {
        let result = Solution::generate_palindromic_permutations("aabbc".to_string());
        let expected: HashSet<String> = [
            "abcba".to_string(),
            "bacab".to_string(),
        ].into_iter().collect();
        assert_eq!(result.len(), 2);
        assert!(result.into_iter().all(|s| expected.contains(&s)));
    }

    #[test]
    fn test_no_palindrome_possible() {
        let result = Solution::generate_palindromic_permutations("abc".to_string());
        assert!(result.is_empty());
    }

    #[test]
    fn test_all_same_characters() {
        let result = Solution::generate_palindromic_permutations("aaaa".to_string());
        assert_eq!(result, vec!["aaaa".to_string()]);
    }

    #[test]
    fn test_large_input() {
        let result  = Solution::generate_palindromic_permutations("aabbccddee".to_string());
        assert!(!result.is_empty());
        assert!(result.iter().all(|s| check_palindrome(s)));
        assert!(result.iter().all(|s| are_permutations(s, "aabbccddee")));
        assert_eq!(result.len(), 1900800);
    }
}

