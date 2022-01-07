use advent_of_code::advent_of_code;
fn main() {
    let result = advent_of_code::problem_one::get_num_of_ints_greater_than_previous_int(
        "./resources/problemOne.txt".to_string(),
        3,
    )
    .expect("failed to get answer!");
    println!("{}", result);

    let result_prob_2 = advent_of_code::problem_two::get_postional_product(
        "./resources/problemTwo.txt".to_string(),
        true,
    )
    .expect("failed to get answer to problem two");
    println!("{}", result_prob_2);
}
