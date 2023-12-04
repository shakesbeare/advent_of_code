use clap::Parser;

#[derive(Debug, Clone)]
enum Testdata {
    Test,
    Input,
}

impl From<&str> for Testdata {
    fn from(s: &str) -> Self {
        match s {
            "test" => Testdata::Test,
            "input" => Testdata::Input,
            _ => panic!("Invalid testdata"),
        }
    }
}

impl std::fmt::Display for Testdata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Testdata::Test => write!(f, "test"),
            Testdata::Input => write!(f, "input"),
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, help="Day number to run, -1 to run all")]
    day: i32,
    
    #[arg(short, long, default_value="input")]
    test: Testdata
}


fn main() {
    let args = Args::parse();

    let data_suffix = format!("_{}.txt", args.test);

    match args.day {
        -1 => {
            let ans = aoc2023::day2::run(format!("day2{}", data_suffix).as_str());
            println!("DAY 3 --- Part 1: {}; Part 2: {}", ans.0, ans.1);
            let ans = aoc2023::day3::run(format!("day3{}", data_suffix).as_str());
            println!("DAY 3 --- Part 1: {}; Part 2: {}", ans.0, ans.1);
            let ans = aoc2023::day4::run(format!("day4{}", data_suffix).as_str());
            println!("DAY 4 --- Part 1: {}; Part 2: {}", ans.0, ans.1);
        }
        2 => {
            let ans = aoc2023::day2::run(format!("day2{}", data_suffix).as_str());
            println!("DAY 2 --- Part 1: {}; Part 2: {}", ans.0, ans.1);
        }
        3 => {
            let ans = aoc2023::day3::run(format!("day3{}", data_suffix).as_str());
            println!("DAY 3 --- Part 1: {}; Part 2: {}", ans.0, ans.1);
        }
        4 => {
            let ans = aoc2023::day4::run(format!("day4{}", data_suffix).as_str());
            println!("DAY 4 --- Part 1: {}; Part 2: {}", ans.0, ans.1);
        }
        i => println!("Day {} not found", i)
    }
}
