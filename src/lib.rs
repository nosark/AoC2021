pub mod advent_of_code {
    pub mod problem_one {
        use std::collections::VecDeque;
        /// TODO: function description
        pub fn get_num_of_ints_greater_than_previous_int(file_path: String, window_size: usize) -> Result<i64, Box<dyn std::error::Error + 'static>> {
            let mut result = 0;
            let mut current_window_sum = 0;
            let mut previous_sum = i64::MAX;
            let mut sum_queue: VecDeque<i64> = VecDeque::from([]);
            let file_contents: String =  std::fs::read_to_string(file_path).expect("unable to read from file!");
            for line in file_contents.lines() {
                if sum_queue.len() >= window_size {
                    // loop through all lines in file
                    // add all line values until we fill first window
                    // else we pop the front node and subtract it from the current_window_sum
                    // and push back the new line and add it to the sum.
                    // This method saves us from doing redunant additions over and over.
                 
                    if current_window_sum > previous_sum {
                        result += 1;
                    }
                    previous_sum = current_window_sum;
                    let current_number: i64 = line.parse().unwrap();
                    let subtract = sum_queue.pop_front();
                    current_window_sum -= subtract.unwrap();
                    
                    sum_queue.push_back(current_number);
                    current_window_sum += current_number;
                } else {
                    let current_number: i64 = line.parse().unwrap();
                    current_window_sum += current_number;
                    sum_queue.push_back(current_number);
                }
            }

            if current_window_sum > previous_sum {
                result += 1;
            }
            
            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::advent_of_code;

    #[test]
    fn advent_problem_one() {
        let mut answer = advent_of_code::problem_one::get_num_of_ints_greater_than_previous_int("./resources/testing/problem1-test1.txt".to_string(), 1).expect("failed to retrieve answer");
        assert_eq!(0, answer);

        answer = advent_of_code::problem_one::get_num_of_ints_greater_than_previous_int("./resources/testing/problem1-test2.txt".to_string(), 1).expect("cannot retrieve answer for input file!");
        assert_eq!(1, answer);

        answer = advent_of_code::problem_one::get_num_of_ints_greater_than_previous_int("./resources/testing/problem1-test3.txt".to_string(), 1).expect("failed to retrieve answer to input file 3");
        assert_eq!(3, answer);

        answer = advent_of_code::problem_one::get_num_of_ints_greater_than_previous_int("./resources/testing/problem1-test4.txt".to_string(), 1).expect("failed to retrieve answer for test 4 of problem 1");
        assert_eq!(0, answer);
    }

    #[test]
    fn advent_problem_one_part_two() {
        let mut answer = advent_of_code::problem_one::get_num_of_ints_greater_than_previous_int("./resources/testing/problem1-test5.txt".to_string(), 3).expect("failed to retrieve answer for problem one part two!");
        assert_eq!(3, answer);
    }
}
