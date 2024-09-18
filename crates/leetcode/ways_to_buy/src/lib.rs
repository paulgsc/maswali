
struct Solution;

impl Solution {
    pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
        let mut ways = 0;

        for pens in 0..=total / cost1 {
            let remaining = (total - pens * cost1) as i64;

            ways += remaining / cost2  as i64 + 1;
        }

        ways
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let total = 20;
        let cost1 = 10;
        let cost2 = 5;
        assert_eq!(Solution::ways_to_buy_pens_pencils(total, cost1, cost2), 9);
    }
}

