use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for (i, &num) in nums.iter().enumerate() {
            if let Some(&idx) = map.get(&num) {
                return Vec::from([i as i32, idx])
            }

            map.insert(target - num, i as i32);
        }

        Vec::new()
    }
}

fn main() {
    let nums = Vec::from([2, 7, 11, 15]);
    let target = 9;

    let res = Solution::two_sum(nums, target);
    println!("result: {:?}", res);
}
