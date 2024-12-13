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
    #[arg(short, long, help="Day number to run, 0 to run all")]
    day: i32,
    
    #[arg(short, long, default_value="input")]
    test: Testdata,

    #[arg(short, long, default_value="default", help="The name of the solution, where multiple are present")]
    solution: String
}


fn main() {
    let args = Args::parse();

    let data_suffix = format!("_{}.txt", args.test);

    match args.day {
        0 => {

            let ans = aoc2024::day1::run(format!("day1{}", data_suffix).as_str());
            println!("DAY 1 --- Part 1: {}; Part 2: {}", ans.0, ans.1);
        }
        1 => {
            let ans = aoc2024::day1::run(format!("day1{}", data_suffix).as_str());
            println!("DAY 1 --- Part 1: {}; Part 2: {}", ans.0, ans.1);
        }
        i => println!("Day {} not found", i)
    }
}
