pub mod advent_of_code {
    pub mod problem_one {
        /// This function takes a file as an argument at reads it to a string.
        /// From there each line is compared to the previous line and result is incremented 
        /// if line[i - 1] < line[i].
        /// 
        /// return type: Result<i64, Error>
        pub fn get_num_of_ints_greater_than_previous_int(file_path: String) -> Result<i64, Box<dyn std::error::Error + 'static>> {
            let mut result = 0;
            let file_contents: String =  std::fs::read_to_string(file_path).expect("unable to read from file!");
            let mut previous = i64::MAX;
            for line in file_contents.lines() {
                let current_number: i64 = line.parse().unwrap();
                if current_number > previous {
                    result += 1;
                }
                previous = current_number;
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
}