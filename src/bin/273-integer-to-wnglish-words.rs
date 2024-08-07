struct Solution;

static LOWER_TWENTY: &'static [&'static str] = &["Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"];

static TENS: &'static [&'static str] = &["Zeroty", "Tenty", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];
static BIGGER: &'static [&'static str] = &["", "Thousand", "Million", "Billion"];

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num < 20 {
            return LOWER_TWENTY[num as usize].to_string();
        }
        let mut chunks: Vec<i32> = Vec::new();
        let mut current_num = num;

        while current_num > 0 {
            let new_chunk = current_num % 1000;
            current_num /= 1000;
            chunks.push(new_chunk);
        }

        chunks.reverse();

        let mut result: Vec<String> = Vec::new();


        for (forward_index, chunk) in (chunks.iter().enumerate()) {
            let backward_index = chunks.len() - 1 - forward_index;
            let chunk_word = Solution::chunk_to_word(*chunk);
            if chunk_word.is_some() {
                result.push(chunk_word.unwrap());
                if backward_index > 0 {
                    result.push(BIGGER[backward_index].to_string())
                }
            }
        }


        return result.join(" ");
    }

    fn chunk_to_word(chunk: i32) -> Option<String> {
        if chunk == 0 {
            return None;
        }

        let mut number_words: Vec<String> = Vec::new();

        let hundreds = chunk / 100;
        if hundreds > 0 {
            number_words.push(LOWER_TWENTY[hundreds as usize].to_string() + " Hundred")
        }

        let rest = chunk - (hundreds * 100);

        if rest < 20 {
            if rest > 0 {
                number_words.push(LOWER_TWENTY[rest as usize].to_string())
            }
        } else {
            let tens = rest / 10;
            if tens > 0 {
                number_words.push(TENS[tens as usize].to_string());
            }
            let ones = rest - (tens * 10);
            if ones > 0 {
                number_words.push(LOWER_TWENTY[ones as usize].to_string());
            }
        }


        return Some(number_words.join(" "));
    }
}

fn main() {
    // assert_eq!("One Hundred Twenty Three", Solution::number_to_words(123));
    // assert_eq!("Twelve Thousand Three Hundred Forty Five", Solution::number_to_words(12345));
    assert_eq!("One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven", Solution::number_to_words(1234567));


    assert_eq!(Solution::number_to_words(69), "Sixty Nine");
    assert_eq!(Solution::number_to_words(0), "Zero");
    assert_eq!(Solution::number_to_words(1234567), "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven");
    assert_eq!(Solution::number_to_words(1000000000), "One Billion");
    assert_eq!(Solution::number_to_words(999999999),
               "Nine Hundred Ninety Nine Million Nine Hundred Ninety Nine Thousand Nine Hundred Ninety Nine"
    );
    assert_eq!(Solution::number_to_words(19999999), "Nineteen Million Nine Hundred Ninety Nine Thousand Nine Hundred Ninety Nine");
}