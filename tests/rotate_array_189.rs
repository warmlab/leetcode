#[cfg(test)]
#[path="../src/rotate_array_189.rs"]

mod rotate_array_189;

mod test_rotate_array {
    use super::rotate_array_189::Solution;
    #[test]
    fn test_rotate_array_1() {
        let s: Solution = Solution;
        let mut nums: Vec<i32> = vec![1,2,3,4,5,6,7];
        let k: i32 = 3;

        s.rotate(&mut nums, k);

        assert_eq!(nums, vec![5,6,7,1,2,3,4]);
    }

    #[test]
    fn test_rotate_array_2() {
        let s: Solution = Solution;
        let mut nums: Vec<i32> = vec![1,2];
        let k: i32 = 3;

        s.rotate(&mut nums, k);

        assert_eq!(nums, vec![2,1]);
    }
}
