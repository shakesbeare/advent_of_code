use super::get_input;

fn parse_input(input: &[String]) -> Vec<(i64, i64)> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    input.iter().for_each(|line| {
        let (a, b) = line.split_once(" ").unwrap();
        left.push(a.trim().parse().unwrap());
        right.push(b.trim().parse().unwrap());
    });

    left.sort();
    right.sort();

    left.into_iter().zip(right).collect()
}

fn pair_distance(pair: &(i64, i64)) -> i64 {
    (pair.0 - pair.1).abs()
}

fn similarity_scores(pairs: &[(i64, i64)]) -> Vec<i64> {
    let mut out = Vec::new();

    pairs.iter().for_each(|(left, _)| {
        let count = pairs
            .iter()
            .filter_map(|(_, b)| if b == left { Some(()) } else { None })
            .count();
        out.push(left * count as i64);
    });

    out
}

pub fn run<F>(filename: F) -> (i64, i64)
where
    F: AsRef<str> + std::fmt::Debug,
{
    let input = get_input(filename.as_ref());
    let parsed = parse_input(&input);
    let part1: i64 = parsed.iter().map(pair_distance).sum();
    let part2: i64 = similarity_scores(&parsed).into_iter().sum();
    (part1, part2)
}
