use std::fmt::Debug;
use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;
pub fn bubble_sort<T: Debug + PartialOrd>(v: &mut Vec<T>) {
    for i in 1..v.len() - 1 {
        for j in 0..v.len() - i {
            if v[j] > v[j+1] {
                v.swap(j,j+1);
            }
        }
    }
}

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Rank>,
    bid: i64,
    hand_type: HandType,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfaKind,
    FullHouse,
    FourOfaKind,
    FiveOfaKind,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

impl From<char> for Rank {
    fn from(c: char) -> Self {
        match c {
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' => Rank::Ten,
            'J' => Rank::J,
            'Q' => Rank::Q,
            'K' => Rank::K,
            'A' => Rank::A,
            _ => unreachable!(),
        }
    }
}

use HandType::*;
impl Hand {

    fn new(cards: &str, bid: i64) -> Self {
        Self {
            cards: Self::get_cards(cards),
            bid,
            hand_type: Self::calculate_type(cards)
        }
    }

    fn get_cards(cards: & str) -> Vec<Rank> {
        let mut cards_vec = Vec::new();
        for char in cards.chars(){
            cards_vec.push(char.into());
        }
        cards_vec
    }

    fn calculate_type(s: & str) -> HandType {
        let mut hm = HashMap::new();
        for c in s.chars() {
            let count = hm.entry(c).or_insert(0);
            *count += 1;
        }
        if hm.len() == 1 { return FiveOfaKind }
        else if hm.len() == 2 {
            for (_, v) in hm.iter() {
                if *v == 4 {
                    return FourOfaKind;
                }
            }
            return FullHouse;
        }
        else if hm.len() == 3 {
            for (_, v) in hm.iter() {
                if *v == 3 {
                    return ThreeOfaKind;
                }
            }
            return TwoPair;
        }
        else if hm.len() == 4 { return OnePair;}
        else if hm.len() == 5 { return HighCard;}
        unreachable!()
    }
}


impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type == other.hand_type {
             for i in 0..=4 {
                 if self.cards[i] < other.cards[i] {
                     return Some(Ordering::Less);
                 }
                 if self.cards[i] > other.cards[i] {
                     return Some(Ordering::Greater);
                 }
             }
            return Some(Ordering::Equal);
        } else {
            self.hand_type.partial_cmp(&other.hand_type)
        }
    }
}

pub fn process_part1(input: &str) -> i64 {
    let mut hands = Vec::new();
    input
        .split("\n")
        .for_each(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<i64>().unwrap();
            let hand = Hand::new(cards, bid);
            hands.push(hand);
        });
    bubble_sort(&mut hands);
    let mut winnings = 0;

    for (idx, hand) in hands.iter().enumerate()   {
            let score = ((idx + 1) as i64) * hand.bid;
            // print!("{:?}", hand);
            // println!(" ----> {} * {} = {}", idx + 1, hand.bid, score);
            winnings += score;
    }
    winnings
}

////////////////////////////////////////


#[derive(Debug)]
pub struct Hand2 {
    cards: Vec<Rank2>,
    bid: i64,
    hand_type: HandType,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Rank2 {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Q,
    K,
    A,
}

impl From<char> for Rank2 {
    fn from(c: char) -> Self {
        match c {
            'J' => Rank2::J,
            '2' => Rank2::Two,
            '3' => Rank2::Three,
            '4' => Rank2::Four,
            '5' => Rank2::Five,
            '6' => Rank2::Six,
            '7' => Rank2::Seven,
            '8' => Rank2::Eight,
            '9' => Rank2::Nine,
            'T' => Rank2::Ten,
            'Q' => Rank2::Q,
            'K' => Rank2::K,
            'A' => Rank2::A,
            _ => unreachable!(),
        }
    }
}

impl Hand2 {

    fn new(cards: &str, bid: i64) -> Self {
        Self {
            cards: Self::get_cards(cards),
            bid,
            hand_type: Self::calculate_joker_type(cards)
        }
    }

    fn get_cards(cards: & str) -> Vec<Rank2> {
        let mut cards_vec = Vec::new();
        for char in cards.chars(){
            cards_vec.push(char.into());
        }
        cards_vec
    }

    fn calculate_type(s: & str) -> (HandType, HashMap<char,i32>) {
        let mut hm = HashMap::new();
        for c in s.chars() {
            let count = hm.entry(c).or_insert(0);
            *count += 1;
        }
        if hm.len() == 1 { return (FiveOfaKind,hm) }
        else if hm.len() == 2 {
            for (_, v) in hm.iter() {
                if *v == 4 {
                    return (FourOfaKind, hm);
                }
            }
            return (FullHouse, hm);
        }
        else if hm.len() == 3 {
            for (_, v) in hm.iter() {
                if *v == 3 {
                    return (ThreeOfaKind, hm);
                }
            }
            return (TwoPair, hm);
        }
        else if hm.len() == 4 { return (OnePair, hm);}
        else if hm.len() == 5 { return (HighCard, hm);}
        unreachable!()
    }

    fn calculate_joker_type(s: & str) -> HandType {
        let (hand, hm) = Hand2::calculate_type(s);
        let match_hand = match hm.get(&'J') {
            Some(&1) => {
                match hand {
                    HighCard => OnePair,
                    OnePair => ThreeOfaKind,
                    TwoPair => FullHouse,
                    ThreeOfaKind => FourOfaKind,
                    FullHouse => unreachable!("1, FullHouse"),
                    FourOfaKind => FiveOfaKind,
                    FiveOfaKind => unreachable!("1, FiveOfaKind"),
                }
            },
            Some(&2) => {
                match hand {
                    HighCard => unreachable!("2, HighCard"),
                    OnePair => ThreeOfaKind,
                    TwoPair => FourOfaKind,
                    ThreeOfaKind => unreachable!("2, ThreeOfaKind"),
                    FullHouse => FiveOfaKind,
                    FourOfaKind => unreachable!("2, FourOfaKind"),
                    FiveOfaKind => unreachable!("2, FiveOfaKind"),
                }
            }
            Some(&3) => {
                match hand {
                    HighCard => unreachable!("3, HighCard"),
                    OnePair => unreachable!("3, OnePair"),
                    TwoPair => unreachable!("3, TwoPair"),
                    ThreeOfaKind => FourOfaKind,
                    FullHouse => FiveOfaKind,
                    FourOfaKind => unreachable!("3, FourOfaKind"),
                    FiveOfaKind => unreachable!("3, FiveOfaKind"),
                }
            }
            Some(&4) => {
                match hand {
                    HighCard => unreachable!("4, HighCard"),
                    OnePair => unreachable!("4, OnePair"),
                    TwoPair => unreachable!("4, TwoPair"),
                    ThreeOfaKind => unreachable!("4, ThreeOfaKind"),
                    FullHouse => unreachable!("4, FourOfaKind"),
                    FourOfaKind => FiveOfaKind,
                    FiveOfaKind => unreachable!("4, FiveOfaKind"),
                }
            }
            Some(&5) => {
                match hand {
                    HighCard => unreachable!("5, HighCard"),
                    OnePair => unreachable!("5, OnePair"),
                    TwoPair => unreachable!("5, TwoPair"),
                    ThreeOfaKind => unreachable!("5, ThreeOfaKind"),
                    FullHouse => unreachable!("5, FourOfaKind"),
                    FourOfaKind => unreachable!("5, FourOfaKind"),
                    FiveOfaKind => FiveOfaKind,
                }
            }
            _ => hand
        };
        match_hand
    }
}


impl PartialEq for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}
impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type == other.hand_type {
            for i in 0..=4 {
                if self.cards[i] < other.cards[i] {
                    return Some(Ordering::Less);
                }
                if self.cards[i] > other.cards[i] {
                    return Some(Ordering::Greater);
                }
            }
            return Some(Ordering::Equal);
        } else {
            self.hand_type.partial_cmp(&other.hand_type)
        }
    }
}




pub fn process_part2(input: &str) -> i64 {
    let mut hands = Vec::new();
    input
        .split("\n")
        .for_each(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<i64>().unwrap();
            let hand2 = Hand2::new(cards, bid);
            hands.push(hand2);
        });
    bubble_sort(&mut hands);
    let mut winnings = 0;

    for (idx, hand) in hands.iter().enumerate()   {
        let score = ((idx + 1) as i64) * hand.bid;
        // print!("{:?}", hand);
        // println!(" ----> {} * {} = {}", idx + 1, hand.bid, score);
        winnings += score;
    }
    winnings
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = process_part1(&input);
        assert_eq!(result, 6440);
    }

    fn test_second() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = process_part2(&input);
        assert_eq!(result, 5905);
    }

}