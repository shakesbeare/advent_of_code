use crate::get_input;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning_nums: HashSet<i32>,
    owned_nums: HashSet<i32>,
}

// type Card = (i32, Vec<i32>, Vec<i32>);

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
            .parse::<usize>()
            .unwrap();

        // get winning nums
        let winning_nums = winning_num_str
            .trim()
            .split(' ')
            .filter(|s| *s != " " && !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<HashSet<i32>>();

        // get my nums
        let owned_nums = my_num_str
            .trim()
            .split(' ')
            .filter(|s| *s != " " && !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<HashSet<i32>>();

        cards.push(Card { id: card_id, winning_nums, owned_nums });
    }

    cards
}

fn score_card(card: Card) -> i32 {
    let winning = card.winning_nums;
    let mine = card.owned_nums;

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
    let winning = &card.winning_nums;
    let mine = &card.owned_nums;

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
    let mut appearances: Vec<i32> = vec![1;218];

    card_data.iter().for_each(|c| {
        let cards_won =  win_scratchcards(c);
        let self_appearances = *(appearances.get(c.id - 1).unwrap());
        
        for offset in 0..cards_won {
            appearances[c.id + (offset as usize)] += self_appearances;
        }

        cards_won_total += cards_won * self_appearances;
    });

    cards_won_total
}

pub fn run(filename: &str) -> (i32, i32) {
    let lines = get_input(filename);
    let cards = parse_lines(lines);
    let sum_scores: i32 =
    cards.clone().iter().map(|c| score_card(c.clone())).sum();
    let total_scratchcards: i32 = play_game(cards);

    assert_eq!(total_scratchcards, 14427616);

    (sum_scores, total_scratchcards)
}
