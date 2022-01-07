pub mod advent_of_code {
    pub mod problem_one {
        use std::collections::VecDeque;
        /// Function: get_nums_of_increases_by_window_sum(file_path: String, window_size: usize)
        /// argument: file_path : the file path of the input we consume.
        /// argument: window_size : the size of the slice we sum.
        /// This function and reads a file into a string, from there we
        /// consume the first window_size elements to build our first sum.
        /// We then compare each sum to its previous (excluding the first window)
        /// and increment result for each increase.
        pub fn get_num_of_ints_greater_than_previous_int(file_path: String, window_size: usize) -> Result<i64, Box<dyn std::error::Error + 'static>> {
            let mut result = 0;
            let mut current_window_sum = 0;
            let mut previous_sum = i64::MAX;
            let mut sum_queue: VecDeque<i64> = VecDeque::from([]);
            let file_contents: String =  std::fs::read_to_string(file_path).expect("unable to read from file!");
            for line in file_contents.lines() {
                if sum_queue.len() >= window_size {
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

    pub mod problem_two {
        struct Submarine {
            horizontal_pos: i64,
            depth: i64,
        }

        struct Command {
            direction: String,
            value: i64,
        }

        impl Submarine {
            pub fn new() -> Self {
                Submarine {
                    horizontal_pos: 0,
                    depth: 0,
                }
            }
        }

        pub fn get_postional_product(input: String) -> Result<i64, Box<dyn std::error::Error + 'static>> {
            let mut sub = Submarine::new();
            let file_contents = std::fs::read_to_string(input).expect("failed to read file for problem two!");
            for line in file_contents.lines() {
                let current_command = parse_command(line);
                // move sub
                let new_direction = current_command.direction;
                match new_direction.as_str() {
                    "forward" => {
                        sub.horizontal_pos += current_command.value;
                    }
                    "down" => {
                        sub.depth += current_command.value;
                    }

                    "up" => {
                        sub.depth -= current_command.value;
                    }

                    _ => println!("something went wrong while executing command!")
                }
            }

            let product = sub.depth * sub.horizontal_pos;
            Ok(product)
        }

        fn parse_command(command: &str) -> Command {
            let variables: Vec<&str> = command.split(' ').collect();
            let direction = String::from(variables[0]);
            let value = variables[1].parse().unwrap();
            Command {direction, value}
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
        let answer = advent_of_code::problem_one::get_num_of_ints_greater_than_previous_int("./resources/testing/problem1-test5.txt".to_string(), 3).expect("failed to retrieve answer for problem one part two!");
        assert_eq!(3, answer);
    }
    #[test]
    fn advent_problem_two_part_one() {
        let answer = advent_of_code::problem_two::get_postional_product("./resources/testing/problem2-test1.txt".to_string()).expect("failed to retrieve answer for problem 2");
        assert_eq!(150, answer);
    }
}
