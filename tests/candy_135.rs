#[cfg(test)]
#[path="../src/candy_135.rs"]
mod candy_135;

mod test_candy {
    use super::candy_135::Solution;
/**/
    #[test]
    fn test_candy_1() {
        let s: Solution = Solution;
        let ratings = vec![1,0,2];
        let r = s.candy(ratings);

        assert_eq!(r, 5);
    }

    #[test]
    fn test_candy_2() {
        let s: Solution = Solution;
        let ratings = vec![1,2,3,4];
        let r = s.candy(ratings);

        assert_eq!(r, 10);
    }

    #[test]
    fn test_candy_3() {
        let s: Solution = Solution;
        let ratings = vec![7,6,5,4,3];
        let r = s.candy(ratings);

        assert_eq!(r, 15);
    }

    #[test]
    fn test_candy_4() {
        let s: Solution = Solution;
        let ratings = vec![1,2,2];
        let r = s.candy(ratings);

        assert_eq!(r, 4);
    }

    #[test]
    fn test_candy_5() {
        let s: Solution = Solution;
        let ratings = vec![1,3,2,2,1];
        let r = s.candy(ratings);

        assert_eq!(r, 7);
    }

    #[test]
    fn test_candy_6() {
        let s: Solution = Solution;
        let ratings = vec![1,6,10,8,7,3,2];
        //let ratings = vec![10,9,8,6,8,9,10,11];
        let r = s.candy(ratings);

        assert_eq!(r, 18);
    }
 
    #[test]
    fn test_candy_7() {
        let s: Solution = Solution;
        let ratings = vec![0,1,2,5,3,2,7];
        //let ratings = vec![10,9,8,6,8,9,10,11];
        let r = s.candy(ratings);

        assert_eq!(r, 15);
    }
}