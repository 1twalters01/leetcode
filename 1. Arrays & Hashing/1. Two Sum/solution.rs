use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut complements: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for (current_idx, &num) in nums.iter().enumerate() {
            if let Some(&stored_idx) = complements.get(&num) {
                return Vec::from([stored_idx, current_idx as i32])
            }

            complements.insert(target - num, current_idx as i32);
        }

        unreachable!()
    }
}

fn main() {
    let nums = Vec::from([2, 7, 11, 15]);
    let target = 9;

    let solution = Solution::two_sum(nums, target);
    println!("solution: {:?}", solution);
}
