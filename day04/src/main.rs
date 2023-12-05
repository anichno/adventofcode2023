use std::collections::{HashMap, HashSet};

struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>,
}

fn parse(input: &[&str]) -> Vec<Card> {
    let mut cards = Vec::new();
    for line in input {
        let (_, line) = line.split_once(' ').unwrap();
        let (id, line) = line.split_once(':').unwrap();
        let id = id.trim().parse().unwrap();
        let (winning_numbers_str, numbers_str) = line.split_once('|').unwrap();
        let mut winning_numbers = HashSet::new();
        for wnum in winning_numbers_str.trim().split_ascii_whitespace() {
            winning_numbers.insert(wnum.parse().unwrap());
        }

        let mut numbers = HashSet::new();
        for num in numbers_str.trim().split_ascii_whitespace() {
            numbers.insert(num.parse().unwrap());
        }

        cards.push(Card {
            id,
            winning_numbers,
            numbers,
        });
    }

    cards
}

fn solve1(input: &[&str]) -> u32 {
    let cards = parse(input);
    let mut sum = 0;

    for card in cards {
        let num_winning = card.winning_numbers.intersection(&card.numbers).count();
        if num_winning > 0 {
            sum += 2_u32.pow(num_winning as u32 - 1);
        }
    }

    sum
}

fn resolve_children(
    cur_card: &Card,
    cards: &HashMap<u32, Card>,
    num_children: &mut HashMap<u32, u32>,
) {
    let num_subcards = cur_card
        .winning_numbers
        .intersection(&cur_card.numbers)
        .count() as u32;

    let mut sum = num_subcards;
    for i in cur_card.id + 1..cur_card.id + 1 + num_subcards {
        if let Some(tot_below) = num_children.get(&i) {
            sum += tot_below;
        } else {
            resolve_children(cards.get(&i).unwrap(), cards, num_children);
            sum += num_children.get(&i).unwrap();
        }
    }

    num_children.insert(cur_card.id, sum);
}

fn solve2(input: &[&str]) -> u32 {
    let card_by_id: HashMap<u32, Card> =
        HashMap::from_iter(parse(input).into_iter().map(|c| (c.id, c)));
    let mut num_children: HashMap<u32, u32> = HashMap::new();

    for card in card_by_id.values() {
        resolve_children(card, &card_by_id, &mut num_children);
    }

    let mut sum = card_by_id.len();
    for (card_id, _) in card_by_id.iter() {
        sum += *num_children.get(card_id).unwrap_or(&0) as usize;
    }

    sum as u32
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 13)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 30)
    }
}
