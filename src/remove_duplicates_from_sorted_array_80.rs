mod solution;

pub use solution::Solution;

impl Solution {
    pub fn remove_duplicates(&self, nums: &mut Vec<i32>) -> i32 {
        let mut length = nums.len();

        if length == 0 {
            return 0;
        }

        let mut item = nums[0];
        let mut i: usize = 1;
        let mut count: i32 = 1;

        while i < length {
            if nums[i] == item {
                if count < 2 {
                    i += 1;
                    count += 1;
                } else {
                    nums.remove(i);
                    length -= 1;
                }
            } else {
                item = nums[i];
                count = 1;
                i += 1;
            }
        }

        length as i32
    }
}