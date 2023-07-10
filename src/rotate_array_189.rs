mod solution;

pub use solution::Solution;

impl Solution {
    pub fn rotate(&self, nums: &mut Vec<i32>, k: i32) {
        let length: usize = nums.len();
        let mut mid: usize = k as usize;

        while mid > length {
            mid -= length;
        }

        nums.rotate_right(mid);
    }
}