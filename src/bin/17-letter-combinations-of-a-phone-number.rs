struct Solution;

static DIGITTOCHARS: &'static [&'static [char]] = &[
    &[],
    &[], &['a', 'b', 'c'], &['d', 'e', 'f']
    , &['g', 'h', 'i'], &['j', 'k', 'l'], &['m', 'n', 'o']
    , &['p', 'q', 'r', 's'], &['t', 'u', 'v'], &['w', 'x', 'y', 'z']
];

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut current_chars: Vec<char> = Vec::new();
        let mut result: Vec<String> = Vec::new();
        Self::process_keys_index(0, &mut current_chars, &digits, &mut result);

        result
    }
    fn process_keys_index(index: i32, current_chars: &mut Vec<char>, digits: &String, result: &mut Vec<String>) {
        if index < (digits.len() as i32){
            let current_key: i32 = (((*digits).as_str()).chars().nth(index as usize).unwrap() as i32) - 48;
            for current_char in DIGITTOCHARS[current_key as usize] {
                (*current_chars).push(*current_char);
                Self::process_keys_index(index + 1, current_chars, digits, result);
                (*current_chars).pop();
            }
        } else if(current_chars.len() > 0){
            let new_result = String::from((*current_chars).iter().collect::<String>());
            (*result).push(new_result);
        }
    }
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn main() {
    let mut expected = vec_of_strings!("ad","ae","af","bd","be","bf","cd","ce","cf");
    expected.sort();
    let mut result = Solution::letter_combinations(String::from("23"));
    result.sort();
    assert_eq!(expected, result);
}