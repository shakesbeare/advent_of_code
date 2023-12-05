pub mod day2;
pub mod day3;
pub mod day4_naive;
pub mod day4;

fn get_input(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>()
}
