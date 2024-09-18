
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn max_consecutive_robots(
        charge_times: Vec<i32>,
        running_costs: Vec<i32>,
        budget: i64,
        ) -> i32 {
        let mut max_consecutive = 0;
        let mut running_sum = 0i64;
        let mut max_deque: VecDeque<usize> = VecDeque::new();
        let mut left = 0;

        for right in 0..charge_times.len() {

            running_sum += running_costs[right] as i64;

            while !max_deque.is_empty() && charge_times[*max_deque.back().unwrap()] <= charge_times[right] {
                max_deque.pop_back();
            }
            max_deque.push_back(right);

            let k = (right - left + 1) as i64;
            let max_charge = charge_times[*max_deque.front().unwrap()] as i64;
            let total_cost = max_charge + k * running_sum;

            if total_cost > budget {
                running_sum -= running_costs[left] as i64;
                if max_deque.front() == Some(&left) {
                    max_deque.pop_front();
                }
                left += 1;
            }

            max_consecutive = max_consecutive.max((right - left + 1) as i32);
        }

        max_consecutive
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let charge_times = vec![3, 6, 1, 3, 4];
        let running_costs = vec![2, 1, 3, 4, 5];
        let budget = 25;
        let result = Solution::max_consecutive_robots(charge_times, running_costs, budget);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_case_2() {
        let charge_times = vec![11, 12, 19];
        let running_costs = vec![10, 8, 7];
        let budget = 19;
        let result = Solution::max_consecutive_robots(charge_times, running_costs, budget);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_case_3() {
        let charge_times = vec![1, 3, 5, 7];
        let running_costs = vec![1, 1, 1, 1];
        let budget = 10;
        let result = Solution::max_consecutive_robots(charge_times, running_costs, budget);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_case_4() {
        let charge_times = vec![3, 6, 8, 10, 12];
        let running_costs = vec![1, 1, 1, 1, 1];
        let budget = 30;
        let result = Solution::max_consecutive_robots(charge_times, running_costs, budget);
        assert_eq!(result, 5);
    }
}

