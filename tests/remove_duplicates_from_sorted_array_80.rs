#[cfg(test)]
#[path="../src/remove_duplicates_from_sorted_array_80.rs"]
mod remove_duplicates_from_sorted_array_80;

mod test_duplicates_from_sorted_array {
    use super::remove_duplicates_from_sorted_array_80::Solution;

    #[test]
    fn test_remove_duplicates_1() {
        let s: Solution = Solution;
        let mut nums: Vec<i32> = vec![1,1,1,2,2,3];

        let r: i32 = s.remove_duplicates(&mut nums);

        println!("{:?}", nums);
        assert_eq!(r, 5);
        assert_eq!(nums, vec![1,1,2,2,3]);
    }

    #[test]
    fn test_remove_duplicates_2() {
        let s: Solution = Solution;
        let mut nums: Vec<i32> = vec![0,0,1,1,1,1,2,3,3];

        let r: i32 = s.remove_duplicates(&mut nums);

        println!("{:?}", nums);
        assert_eq!(r, 7);
        assert_eq!(nums, vec![0,0,1,1,2,3,3]);
    }
}