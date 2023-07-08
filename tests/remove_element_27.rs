#[cfg(test)]
#[path="../src/remove_element_27.rs"]

mod remove_element_27;

mod test_remove_element {
    use super::remove_element_27::Solution;

    #[test]
    fn test_remove_element_1() {
        let s: Solution = Solution;
        let mut nums: Vec<i32> = vec![0,1,2,2,3,0,4,2];
        let val: i32 = 2;

        let r = s.remove_element(&mut nums, val);
        assert_eq!(r, 5);
        assert_eq!(nums, vec![0,1,4,0,3,-1,-1,-1]);
    }

    #[test]
    fn test_remove_element_2() {
        let s: Solution = Solution;
        let mut nums: Vec<i32> = vec![1];
        let val: i32 = 1;

        let r = s.remove_element(&mut nums, val);
        assert_eq!(r, 0);
        assert_eq!(nums, vec![-1]);
    }

    #[test]
    fn test_remove_element_3() {
        let s: Solution = Solution;
        let mut nums: Vec<i32> = vec![];
        let val: i32 = 0;

        let r: i32 = s.remove_element(&mut nums, val);
        assert_eq!(r, 0);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_remove_element_4() {
        let s: Solution = Solution;
        let mut nums: Vec<i32> = vec![4,4,4,4,4];
        let val: i32 = 4;

        let r: i32 = s.remove_element(&mut nums, val);
        assert_eq!(r, 0);
        assert_eq!(nums, vec![-1,-1,-1,-1,-1]);
    }
}
