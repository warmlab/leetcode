#[cfg(test)]
#[path="../src/jump_game_55.rs"]
mod jump_game_55;

mod test_jump_game {
    use super::jump_game_55::Solution;

    #[test]
    fn test_jump_game_1() {
        let s: Solution = Solution;
        let nums: Vec<i32> = vec![2,3,1,1,4];

        let r: bool = s.can_jump(nums);

        assert!(r);
    }

    #[test]
    fn test_jump_game_2() {
        let s: Solution = Solution;
        let nums: Vec<i32> = vec![3,2,1,0,4];

        let r: bool = s.can_jump(nums);

        assert_eq!(r, false);
    }

    #[test]
    fn test_jump_game_3() {
        let s: Solution = Solution;
        let nums: Vec<i32> = vec![0];

        let r: bool = s.can_jump(nums);

        assert_eq!(r, true);
    }
}