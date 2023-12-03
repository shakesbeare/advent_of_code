#[derive(Debug)]
enum Token {
    Dot,
    Symbol(String),
    Number(String),
    Placeholder(String),
}

type Line = Vec<Token>;

fn get_input() -> Vec<String> {
    std::fs::read_to_string("day3_input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>()
}

fn parse_line(line: &String) -> Line {
    let mut output = vec![];
    let mut chars = line.chars();
    while let Some(ch) = chars.next() {
        match ch {
            '.' => output.push(Token::Dot),
            '0'..='9' => {
                let mut num_str = ch.to_string();
                let mut ch_o = chars.next();
                'inner: while let Some(ch) = ch_o {
                    if ch.is_ascii_digit() {
                        num_str.push(ch);
                    } else {
                        output.push(Token::Number(num_str.clone()));
                        for _ in 0..num_str.len() - 1 {
                            output.push(Token::Placeholder(num_str.clone()));
                        }
                        match ch {
                            '.' => output.push(Token::Dot),
                            '0'..='9' => {}
                            c => output.push(Token::Symbol(c.to_string())),
                        }
                        break 'inner;
                    }
                    ch_o = chars.next();
                }

                if ch_o.is_none() {
                    output.push(Token::Number(num_str.clone()));
                    for _ in 0..num_str.len() - 1 {
                        output.push(Token::Placeholder(num_str.clone()));
                    }
                }
            }
            c => output.push(Token::Symbol(c.to_string())),
        }
    }
    return output;
}

fn get_adjacent_offsets(s: &Token) -> Vec<(isize, isize)> {
    match s {
        Token::Number(inner) | Token::Symbol(inner) => {
            let mut out = vec![];
            for i in 0..3 {
                for j in 0..inner.len() + 2 {
                    let y = (i as isize) - 1;
                    let x = (j as isize) - 1;

                    if y == 0 && x >= 0 && x < (inner.len()) as isize {
                        continue;
                    }

                    out.push((x, y));
                }
            }
            out
        }
        _ => vec![(-1, 0), (1, 0), (-1, -1), (1, 1), (1, -1), (-1, 1)],
    }
}

fn has_adjacent_symbol(board: &Vec<Line>, coord: (usize, usize)) -> bool {
    let Some(line) = board.get(coord.1) else {
        return false;
    };

    let Some(token) = line.get(coord.0) else {
        return false;
    };

    let Token::Number(_) = &token else {
        return false;
    };

    let offsets = get_adjacent_offsets(token);
    for offset in offsets {
        let offset_coord =
            (coord.0 as isize + offset.0, coord.1 as isize - offset.1);

        if offset_coord.0.is_negative() || offset_coord.1.is_negative() {
            continue;
        }

        if let Some(line) =
            board.get::<usize>(offset_coord.1.try_into().unwrap())
        {
            if let Some(Token::Symbol(_)) =
                line.get::<usize>(offset_coord.0.try_into().unwrap())
            {
                return true;
            }
        }
    }
    return false;
}

fn get_gear_numbers(
    board: &Vec<Line>,
    coord: (usize, usize),
) -> Option<(i32, i32)> {
    let Some(line) = board.get(coord.1) else {
        return None;
    };

    let Some(token) = line.get(coord.0) else {
        return None;
    };

    let Token::Symbol(sym) = &token else {
        return None;
    };

    if sym != "*" { return None; }

    let offsets = get_adjacent_offsets(token);
    let mut adj_nums = vec![];
    for offset in offsets {
        let offset_coord =
            (coord.0 as isize + offset.0, coord.1 as isize - offset.1);

        if offset_coord.0.is_negative() || offset_coord.1.is_negative() {
            continue;
        }

        if let Some(line) =
            board.get::<usize>(offset_coord.1.try_into().unwrap())
        {
            if let Some(token) =
                line.get::<usize>(offset_coord.0.try_into().unwrap())
            {
                match token {
                    Token::Number(num) | Token::Placeholder(num) => {
                        if !adj_nums.contains(&num) { adj_nums.push(num); }
                    }
                    _ => {}
                };
            }
        }
    }

    dbg!(&adj_nums);
    if adj_nums.len() == 2 {
        // its a gear!
        let nums: Vec<i32> = adj_nums
            .iter()
            .map(|n| {
                let Ok(n) = n.parse::<i32>() else {
                    panic!("parse error")
                };
                n
            })
            .collect();

        let i = nums.first().unwrap();
        let j = nums.get(1).unwrap();
        return Some((*i, *j));
    }
    return None;
}

fn main() {
    let input = get_input();
    let board: Vec<Line> = input.iter().map(parse_line).collect();
    let x_max = &board.get(0).unwrap().len();

    let mut nums = vec![];
    let mut gears = vec![];
    for y in 0..board.len() {
        for x in 0..*x_max {
            let Some(line) = board.get(y) else { continue };
            let Some(symbol) = line.get(x) else { continue };

            if has_adjacent_symbol(&board, (x, y)) {
                nums.push(symbol);
            }

            if let Some(gp) = get_gear_numbers(&board, (x, y)) {
                gears.push(gp);
            }
        }
    }

    let ans: i32 = nums
        .iter()
        .map(|sym| {
            let Token::Number(num) = sym else {
                panic!("not a number?")
            };
            let Ok(num) = num.parse::<i32>() else {
                panic!("parse error")
            };
            num
        })
        .sum();

    let p2: i32 = gears.iter().map(|t| {
        t.0 * t.1
    }).sum();

    dbg!(ans, p2);
}
