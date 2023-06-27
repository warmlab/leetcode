/*#[cfg(test)]
#[path = "../src/solution.rs"]
mod solution;*/
#[cfg(test)]
#[path = "../src/merge_sorted_array_88.rs"]
mod merge_sorted_array_88;

mod test_merge_sorted_array {
    //use super::MergeSortedArray_88::Solution;
    use super::merge_sorted_array_88::Solution;

    #[test]
    fn test_merge_1() {
        let s: Solution = Solution{};
        let mut nums1: Vec<i32> = vec![0];
        let mut nums2: Vec<i32> = vec![1];
        let m: i32 = 0;
        let n: i32 = 1;

        s.merge(&mut nums1, m, &mut nums2, n);
        println!("{:?}", nums1);
    }

    #[test]
    fn test_merge_2() {
        let s: Solution = Solution{};
        let mut nums1: Vec<i32> = vec![1,2,3,0,0,0];
        let mut nums2: Vec<i32> = vec![2,5,6];
        let m: i32 = 3;
        let n: i32 = 3;

        s.merge(&mut nums1, m, &mut nums2, n);
        println!("{:?}", nums1);
    }

    #[test]
    fn test_merge_3() {
        let s: Solution = Solution{};
        let mut nums1: Vec<i32> = vec![];
        let mut nums2: Vec<i32> = vec![1];
        let m: i32 = 0;
        let n: i32 = 1;

        s.merge(&mut nums1, m, &mut nums2, n);
        println!("{:?}", nums1);
    }
}
