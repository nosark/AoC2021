pub mod advent_of_code {
    pub mod problem_one {
        use std::collections::VecDeque;
        /// This function takes a file as an argument at reads it to a string.
        /// From there each line is compared to the previous line and result is incremented 
        /// if line[i - 1] < line[i].
        /// 
        /// return type: Result<i64, Error>
        pub fn get_num_of_ints_greater_than_previous_int(file_path: String) -> Result<i64, Box<dyn std::error::Error + 'static>> {
            let mut result = 0;
            let mut current_sum = 0;
            let mut previous_sum = i64::MAX;
            let mut sum_queue: VecDeque<i64> = VecDeque::from([]);
            let file_contents: String =  std::fs::read_to_string(file_path).expect("unable to read from file!");
            for line in file_contents.lines() {
                if sum_queue.len() >= 3 {
                    //TODO: see if subtracting first number in queue and adding new line number instead
                    // of adding all three numbers everytime.
                    current_sum = sum_queue[0] + sum_queue[1] + sum_queue[2];
                    if current_sum > previous_sum {
                        result += 1;
                    }

                    let current_number: i64 = line.parse().unwrap();
                    sum_queue.pop_front();
                    sum_queue.push_back(current_number);
                    previous_sum = current_sum;
                } else {
                    let current_number: i64 = line.parse().unwrap();
                    sum_queue.push_back(current_number);
                }
            }

            current_sum = sum_queue[0] + sum_queue[1] + sum_queue[2];
            if current_sum > previous_sum {
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
        let mut answer = advent_of_code::problem_one::get_num_of_ints_greater_than_previous_int("./resources/testing/problem1-test1.txt".to_string()).expect("failed to retrieve answer");
        assert_eq!(0, answer);

        answer = advent_of_code::problem_one::get_num_of_ints_greater_than_previous_int("./resources/testing/problem1-test2.txt".to_string()).expect("cannot retrieve answer for input file!");
        assert_eq!(1, answer);

        answer = advent_of_code::problem_one::get_num_of_ints_greater_than_previous_int("./resources/testing/problem1-test3.txt".to_string()).expect("failed to retrieve answer to input file 3");
        assert_eq!(3, answer);

        answer = advent_of_code::problem_one::get_num_of_ints_greater_than_previous_int("./resources/testing/problem1-test4.txt".to_string()).expect("failed to retrieve answer for test 4 of problem 1");
        assert_eq!(0, answer);
    }

    #[test]
    fn advent_problem_one_part_two() {
        let mut answer = advent_of_code::problem_one::get_num_of_ints_greater_than_previous_int("./resources/testing/problem1-test5.txt".to_string()).expect("failed to retrieve answer for problem one part two!");
        assert_eq!(3, answer);
    }
}
