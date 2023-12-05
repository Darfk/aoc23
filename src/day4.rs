use std::collections::BTreeSet;

type Set<T> = BTreeSet<T>;

#[derive(Debug)]
struct Card {
    number: u32,
    picks: Set<u32>,
    winners: Set<u32>,
}

impl Card {
    fn from_input_line(line: &str) -> Self {
        let (card_details, card_numbers) = line.split_once(':').unwrap();
        let card_number: u32 = card_details
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let (winners, picks) = card_numbers.split_once('|').unwrap();
        let winners = winners
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Set<u32>>();
        let picks = picks
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Set<u32>>();
        Card {
            number: card_number,
            picks,
            winners,
        }
    }

    fn num_winners(&self) -> u32 {
        self.picks.intersection(&self.winners).count() as u32
    }
}

pub fn part1(input: String) -> u32 {
    let cards = input.lines().map(Card::from_input_line).collect::<Vec<_>>();

    let mut score = 0u32;

    for card in cards {
        let exponent = card.winners.iter().fold(0u32, |a, w| {
            a + card
                .picks
                .iter()
                .fold(0u32, |a, p| a + if p == w { 1 } else { 0 })
        });

        score += if exponent == 0 { 0 } else { 1 << exponent - 1 };
    }

    return score;
}

pub fn part2(input: String) -> u32 {
    let cards = input
        .lines()
        .map(Card::from_input_line)
        .map(|card| card.num_winners())
        .collect::<Vec<u32>>();

    let mut copies = cards.iter().map(|card| (*card, 1u32)).collect::<Vec<_>>();

    for index in 0..copies.len() {
        let (card, amount) = copies[index];
        for copy_index in index + 1..copies.len().min(index + 1 + card as usize) {
            copies[copy_index].1 += amount;
        }

        
    }

    dbg!(&copies);

    return copies.iter().map(|copy| copy.1).sum();
}

#[cfg(test)]
mod test {
    #[test]
    fn test_if_can_push_into_itered_vec() {
        let v = vec![1, 2, 3];
        let q = &v[0..5];
        dbg!(q);
    }

    #[test]
    fn test_some_other_thing() {
        let range = 1..1 + 1;
        for i in range {
            dbg!(i);
        }
    }
}
