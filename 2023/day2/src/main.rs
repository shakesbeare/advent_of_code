use anyhow::Result;

#[derive(Debug)]
struct Game {
    id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

struct Bag {
    red: i32,
    green: i32,
    blue: i32,
}

fn get_input() -> Vec<String> {
    std::fs::read_to_string("day2_test.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>()
}

fn parse_games() -> Result<Vec<Game>> {
    let mut games = vec![];
    let lines = get_input();
    for line in lines.iter() {
        if line.is_empty() { break; }
        let mut s = line.split(':');
        let id= s.clone().next().unwrap().split_whitespace().nth(1).unwrap();
        s.next();
        let rounds = s.collect::<String>().trim().to_owned();
        let rounds = rounds.split([',', ';']);
        let (mut r, mut g, mut b) = (0, 0, 0);
        for round in rounds {
            let count = round.split_whitespace().next().unwrap().parse::<i32>()?;
            let color = round.split_whitespace().nth(1).unwrap().to_string();
            if color == "red" && count > r {
                r = count
            } else if color == "green" && count > g {
                g = count
            } else if color == "blue" && count > b {
                b = count
            }
        }
        games.push(Game {
            id: id.parse::<i32>()?,
            red: r,
            green: g,
            blue: b,
        });
    }
    Ok(games)
}

fn check_games(games: Vec<Game>, bag: Bag) -> i32 {
    games
        .iter()
        .map(|g| {
            dbg!(&g);
            if g.red > bag.red || g.green > bag.green || g.blue > bag.blue {
                0
            } else {
                g.id
            }
        })
        .reduce(|a, b| a + b)
        .unwrap()
}

fn main() {
    let Ok(games) = parse_games() else {
        eprintln!("Parse Error");
        std::process::exit(1);
    };

    let sum = check_games(
        games,
        Bag {
            red: 12,
            blue: 14,
            green: 13,
        },
    );

    println!("{sum}");
}
