mod solution;

pub use solution::Solution;

impl Solution {
    pub fn candy(&self, ratings: Vec<i32>) -> i32 {
        let mut compares: Vec<i32> = Vec::new();
        let mut weight: i32 = 0;
        let mut to_weight: i32 = 0;
        let mut vetex_weight: i32 = 0;
        let mut direction: i8 = 0;
        let mut r: i32 = 0;

        let length: usize = ratings.len();
        let mut index: usize = 0;
        while index < length - 1 {
            compares.push(ratings[index + 1] - ratings[index]);
            index += 1;
        }

        println!("Compares: {:?}", compares);

        for compare in compares {
            if compare > 0 {
                if direction > 0 {
                    weight = vetex_weight;
                    vetex_weight += 1;
                    //} else if direction < 0 {
                    //weight = 1;
                    r += weight;
                } else {
                    //r += to_weight;
                r += if to_weight > vetex_weight {
                    to_weight
                } else {
                    vetex_weight
                };
                    vetex_weight = 1;
                    to_weight = 0;
                    direction = 1;
                }
                if to_weight > 0 {
                    r += to_weight;
                    to_weight = 0;
                }
                //r += weight;
            } else if compare < 0 {
                if direction < 0 {
                    to_weight += 1;
                    weight += 1;
                //} else if direction > 0 {
                //    to_weight = 1;
                //    weight = 0;
                //    direction = -1;
                    r += weight;
                } else {
                    to_weight = 1;
                    weight = 0;
                    direction = -1;
                }
            } else {
                weight = 0;
                r += if to_weight > vetex_weight {
                    to_weight
                } else {
                    vetex_weight
                };
        /*if direction > 0 {
            r += vetex_weight;
        } else if direction < 0 {
            r += to_weight;
        }*/
                direction = 0;
                to_weight = 0;
                vetex_weight = 0;
            }
            println!("result: [{}], weight=[{}], to=[{}], vetex=[{}]", r, weight, to_weight, vetex_weight);
        }

        /*
        if direction > 0 {
            r += vetex_weight;
        } else if direction < 0 {
            r += to_weight;
        } */
        r += if to_weight > vetex_weight {
            to_weight
        } else {
            vetex_weight
        };

        r + (length as i32)
    }
}
