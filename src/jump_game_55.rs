mod solution;

pub use solution::Solution;

impl Solution {
    pub fn can_jump(&self, nums: Vec<i32>) -> bool {
        let last_index: i32 = nums.len() as i32 - 1;
        let mut steps: Vec<i32> = Vec::new();
        let mut pos: i32;

        if last_index == 0 {
            return true;
        }

        for (index, n) in nums.into_iter().enumerate() {
            steps.push(index as i32 + n);
        }

        pos = last_index;
        let mut index: usize = 0;
        while (index as i32) < pos {
            if steps[index] >= pos { // can just to the target position
                pos = index as i32;
                if pos == 0 {
                    return true;
                }

                index = 0;
            } else {
                index += 1;
            }
        }

        false
    }
}