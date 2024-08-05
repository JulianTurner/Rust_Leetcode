struct Solution;

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut team_count = 0;
        for (pos_1, e_1) in rating.iter().enumerate() {
            for pos_2 in (pos_1 + 1)..rating.len() {
                let e_2 = rating[pos_2];
                if e_1 < &e_2 {
                    for pos_3 in (pos_2 + 1)..rating.len() {
                        let e_3 = rating[pos_3];
                        if e_2 < e_3 {
                            team_count += 1;
                        }
                    }
                }
                else if e_1 > &e_2 {
                    for pos_3 in (pos_2 + 1)..rating.len() {
                        let e_3 = rating[pos_3];
                        if e_2 > e_3 {
                            team_count += 1;
                        }
                    }
                }
            }
        }

        return team_count;
    }
}

fn main() {
    let result = Solution::num_teams(vec![2, 5, 3, 4, 1]);
    println!("{result}");
}