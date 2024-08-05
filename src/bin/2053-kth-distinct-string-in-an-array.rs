use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut distinct_arr: Vec<bool> = Vec::with_capacity(arr.len());
        distinct_arr.resize_with(arr.len(), || true);
        let mut first_occurrence: HashMap<String, usize> = HashMap::new();

        for (current_index, current_str) in arr.iter().enumerate() {
            let current_entry = first_occurrence.get(current_str);
            if current_entry.is_some() {
                let prev_index = current_entry.unwrap();
                distinct_arr[current_index] = false;
                distinct_arr[*prev_index] = false;
            } else {
                first_occurrence.insert(current_str.to_string(), current_index);
            }
        }
        let mut counter = 0;
        for (index, is_distinct) in distinct_arr.iter().enumerate() {
            if *is_distinct {
                counter += 1;
                if counter == k {
                   return  arr[index].to_string();
                }
            }
        }

        String::new()
    }
}
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn main() {
    let arr: Vec<String> = vec_of_strings!("d","b","c","b","c","a");
    let k = 2;
    let result = Solution::kth_distinct(arr, k);
    println!("{result}");
}

