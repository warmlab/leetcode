mod solution;

pub use solution::Solution;

impl Solution {
    pub fn merge(&self, nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut c: usize = m as usize;

        while j < n as usize {
            if i == c {
                nums1.insert(i, nums2[j]);
                j += 1;
                i += 1;
                c += 1;
            } else {
                if nums2[j] > nums1[i] {
                    i += 1;
                } else {
                    nums1.insert(i, nums2[j]);
                    j += 1;
                    i += 1;
                    c += 1;
                }
            }
        }

        while nums1.len() > (m + n) as usize {
            nums1.pop();
        }
    }
}
