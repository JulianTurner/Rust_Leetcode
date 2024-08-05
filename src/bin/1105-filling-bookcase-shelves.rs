use std::cmp::{max, min};

struct Solution;

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let mut dp: Vec<Option<i32>> = Vec::with_capacity(books.len());
        dp.resize_with(books.len(), || None);


        let result = Self::optimal_height(&books, & mut dp, shelf_width,0,0,shelf_width);
        println!("{:?}", dp);
        return result;
        
    }

    fn optimal_height(books: &[Vec<i32>], dp: &mut [Option<i32>], shelf_with: i32, book_index: i32, pre_height: i32, remaining_width: i32) -> i32 {
        if book_index == (*books).len() as i32{
            return 0;
        }

        let book = &(*books)[book_index as usize];
        let book_thickness = book[0];
        let book_height = book[1];

        let dp_value: Option<i32> = *(dp.get(book_index as usize).unwrap());
        let result_new_shelf: i32 = dp_value.or_else(|| -> Option<i32> {
            let temp = book_height + Self::optimal_height(books, dp, shelf_with, book_index + 1, book_height, shelf_with - book_thickness);
            dp[book_index as usize] = Some(temp);
            Some(temp)
        }).unwrap();

        let result_same_shelf: Option<i32> = if remaining_width >= book_thickness {
            let new_pre_height = max(pre_height, book_height);
            let actual_height_addition = new_pre_height - pre_height;
            Some(actual_height_addition + Self::optimal_height(books, dp, shelf_with, book_index + 1, new_pre_height, remaining_width - book_thickness))
        } else {
            None
        };


        return result_same_shelf.and_then(|i: i32| -> Option<i32> {
           Some(min(i, result_new_shelf))
        }).or(Some(result_new_shelf)).unwrap();
    }
}

fn main() {
    let result = Solution::min_height_shelves(vec![vec![1, 1], vec![2, 3], vec![2, 3], vec![1, 1], vec![1, 1], vec![1, 1], vec![1, 2]], 4);
    println!("{result}");
}