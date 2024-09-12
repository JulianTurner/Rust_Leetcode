struct Solution;
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut allowed_chars: [bool; 26] = [false; 26];
        for c in allowed.chars() {
            let index = ((c as i32) - ('a' as i32)) as usize;
            allowed_chars[index] = true;
        }
        let mut result: i32 = 0;

        'word: for word in words.iter() {
            for c_word in word.chars() {
                let index = ((c_word as i32) - ('a' as i32)) as usize;
                if allowed_chars[index] == false {
                    continue 'word;
                }
            }
            result += 1;
        }

        result
    }
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn main() {
    let allowed = String::from("ab");
    let words: Vec<String> = vec_of_strings!["ad","bd","aaab","baa","badab"];
    assert_eq!(2, Solution::count_consistent_strings(allowed, words));
}