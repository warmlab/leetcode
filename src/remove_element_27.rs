mod solution;

pub use solution::Solution;

impl Solution {
    pub fn remove_element(&self, nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i: i32 = 0;
        let mut pos: i32 = nums.len() as i32 - 1;

        while i <= pos && pos >= 0 {
            if nums[i as usize] == val {
                if pos > i {
                    nums[i as usize] = nums[pos as usize];
                    nums[pos as usize] = -1;
                    pos -= 1;
                } else {
                    nums[i as usize] = -1;
                    break;
                }
            } else {
                i += 1;
            }
        }

        i
    }
}