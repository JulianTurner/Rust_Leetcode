struct Solution;
impl Solution {
    pub fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut l_to_r: Vec<i32> = Vec::with_capacity(nums.len());

        for (current_index, current_value) in nums.iter().enumerate() {
            while stack.len() > 0 && nums[*stack.last().unwrap()] >= *current_value {
                stack.pop();
            }
            if stack.len() > 0 {
                l_to_r.push(*stack.last().unwrap() as i32);
            } else {
                l_to_r.push(-1);
            }
            stack.push(current_index);
        }

        stack.clear();

        let mut r_to_l: Vec<i32> = Vec::with_capacity(nums.len());

        for (current_index, current_value) in nums.iter().enumerate().rev() {
            while stack.len() > 0 && nums[*stack.last().unwrap()] >= *current_value {
                stack.pop();
            }
            if stack.len() > 0 {
                r_to_l.push(*stack.last().unwrap() as i32);
            } else {
                r_to_l.push(nums.len() as i32);
            }
            stack.push(current_index);
        }
        r_to_l.reverse();

        let threshold_float = threshold as f32;

        for min_index in 0..(nums.len()) {
            let min_index_i32 = min_index as i32;
            let sub_arr_size = (min_index_i32 - l_to_r[min_index]) + (r_to_l[min_index] - min_index_i32) - 1;
            if nums[min_index] as f32 > threshold_float / (sub_arr_size as f32){
                return sub_arr_size;
            }
        }


        return -1;
    }
}

fn main() {
    let nums: Vec<i32> = vec![1, 3, 4, 3, 1];
    let threshold = 6;
    let result = Solution::valid_subarray_size(nums, threshold);
    println!("{result}");
}
