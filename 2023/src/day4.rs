use crate::get_input;
use std::collections::VecDeque;

type Card = (i32, Vec<i32>, Vec<i32>);

fn parse_lines(input: Vec<String>) -> Vec<Card> {
    let mut cards = vec![];
    for line in input.iter() {
        let (header, rest) = line.split_once(':').unwrap();
        let (winning_num_str, my_num_str) = rest.split_once('|').unwrap();

        // get num from header
        let card_id = header
            .split_once(' ')
            .unwrap()
            .1
            .trim()
            .parse::<i32>()
            .unwrap();

        // get winning nums
        let winning_nums = winning_num_str
            .trim()
            .split(' ')
            .filter(|s| *s != " " && !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // get my nums
        let my_nums = my_num_str
            .trim()
            .split(' ')
            .filter(|s| *s != " " && !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        cards.push((card_id, winning_nums, my_nums));
    }

    cards
}

fn score_card(card: Card) -> i32 {
    let winning = card.1;
    let mine = card.2;

    let mut exp = -1;

    for num in mine {
        if winning.contains(&num) {
            exp += 1;
        }
    }

    if exp >= 0 {
        2_i32.pow(exp as u32)
    } else {
        0
    }
}

fn win_scratchcards(card: &Card) -> i32 {
    let winning = &card.1;
    let mine = &card.2;

    let mut cards_won = 0;

    for num in winning {
        if mine.contains(num) {
            cards_won += 1;
        }
    }

    cards_won
}

fn play_game(card_data: Vec<Card>) -> i32 {
    let mut cards_won_total = card_data.len() as i32;
    let mut hand: VecDeque<Card> =
        VecDeque::from(card_data.clone());

    while let Some(card) = hand.pop_front() {
        let cards_won = win_scratchcards(&card);

        // cards are 1-indexed, but vecs are 0-indexed, so this
        // gets the index of the next card
        let mut new_card = card.0;
        for _ in 0..cards_won {
            cards_won_total += 1;
            let card_n = card_data.get(new_card as usize).unwrap();
            
            hand.push_back(card_n.clone());
            new_card += 1;
        }
    }

    cards_won_total
}

pub fn run(filename: &str) -> (i32, i32) {
    let lines = get_input(filename);
    let cards = parse_lines(lines);
    let sum_scores: i32 =
        cards.clone().iter().map(|c| score_card(c.clone())).sum();
    let total_scratchcards: i32 = play_game(cards);

    (sum_scores, total_scratchcards)
}
