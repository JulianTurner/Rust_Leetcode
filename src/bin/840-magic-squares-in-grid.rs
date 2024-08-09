use std::ops::{Range, RangeBounds};

struct Solution;
static MAGIC_SQUARE_RANGE: &'static Range<i32> = &(1..10);

impl Solution {
    fn check_if_magic_square(start_row_index: usize, start_col_index: usize, grid: &Vec<Vec<i32>>) -> bool {
        // check if distinct from 1..9
        let mut numbers = [false; 10];
        for row_index in start_row_index..start_row_index + 3 {
            for col_index in start_col_index..start_col_index + 3 {
                let number: &i32 = &(*grid)[row_index][col_index];
                if !MAGIC_SQUARE_RANGE.contains(number) || numbers[(*number) as usize] {
                    return false;
                }
                numbers[*number as usize] = true;
            }
        }

        let reference_sum: i32 = grid[start_row_index][start_col_index..start_col_index + 3].iter().sum();

        // check if same row sum
        for row_index in start_row_index + 1..start_row_index + 3 {
            if grid[row_index][start_col_index..start_col_index + 3].iter().sum::<i32>() != reference_sum {
                return false;
            }
        }


        // check if same col sum
        for col_index in start_col_index..start_col_index + 2 {
            let sum = grid[start_row_index][col_index] + grid[start_row_index + 1][col_index] + grid[start_row_index + 2][col_index];

            if sum != reference_sum {
                return false;
            }
        }

        // check if same diag sum
        if grid[start_row_index][start_col_index] + grid[start_row_index + 1][start_col_index + 1] + grid[start_row_index + 2][start_col_index + 2] != reference_sum {
            return false;
        }

        if grid[start_row_index][start_col_index + 2] + grid[start_row_index + 1][start_col_index + 1] + grid[start_row_index + 2][start_col_index] != reference_sum {
            return false;
        }

        true
    }
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() < 3 || grid[0].len() < 3 {
            return 0;
        }
        let mut magic_squares: i32 = 0;
        for row_index in 0..grid.len() - 2 {
            for col_index in 0..grid[row_index].len() - 2 {
                if Solution::check_if_magic_square(row_index, col_index, &grid) {
                    magic_squares += 1;
                }
            }
        }
        magic_squares
    }
}

fn main() {
    let grid: Vec<Vec<i32>> = vec![
        vec![7, 6, 2, 2, 4],
        vec![4, 4, 9, 2, 10],
        vec![9, 7, 8, 3, 10],
        vec![8, 1, 9, 7, 5],
        vec![7, 10, 4, 11, 6]
    ];
    let result = Solution::num_magic_squares_inside(grid);
    println!("{result}");
}