use itertools::Itertools;
use std::cmp::Ordering;
use std::ops::Sub;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel  {
    pub hands: Vec<Hand>
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands :Vec<Hand> = s.lines()
            .filter_map(|line| Hand::from_str(line))
            .collect();
        Ok(InputModel{hands})
    }
}


#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
/// the possible values of the cards
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

impl Card {
    /// the value of a card can be determined from the first
    /// character of the notation for it
    fn from_char(c: char) -> Option<Card> {
        match c {
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'J' => Some(Card::Jack),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            'A' => Some(Card::Ace),
            _ => None
        }
    }

}

impl Sub for Card {
    // it is useful to know the difference between values
    // e.g. for determining straights
    type Output = i32;

    fn sub(self, other: Card) -> i32 {
        let a = self as i32;
        let b = other as i32;
        a - b
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
/// the possible poker card combinations ranked from least
/// to most valuable
enum Rank {
    HighCard(Card),
    Pair(Card),
    TwoPairs(Card, Card),
    ThreeOfAKind(Card),
    FullHouse(Card, Card),
    FourOfAKind(Card),
    FiveOfAKind(Card),
}

#[derive(Debug, Eq, Clone)]
/// a poker hand is a collection of combinations or ranks
pub struct Hand {
    /// sorted vector of ranks, most valuable first
    ranks: Vec<Rank>,

    /// original hand dealt
    dealt: String,

    /// the bid
    pub bid: u64
}

/// Override Ord implementation to ignore the hand dealt
/// as all info is in the ranks vector
impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        self.ranks.cmp(&other.ranks)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        let ord = self.ranks.cmp(&other.ranks);
        if ord == Ordering::Equal {
            Some(self.dealt.cmp(&other.dealt))
        } else {
            Some(ord)
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

/// Map groups of consecutive cards with the same value
fn map_card_groups<'a>(group: impl Iterator<Item=Card>) -> Rank {
    let cards:Vec<Card> = group.collect();
    match cards.len() {
        1 => Rank::HighCard(cards[0]),
        2 => Rank::Pair(cards[0]),
        3 => Rank::ThreeOfAKind(cards[0]),
        4 => Rank::FourOfAKind(cards[0]),
        5 => Rank::FiveOfAKind(cards[0]),
        _ => panic!("Invalid number of cards in group")
    }
}

/// match patterns in initially sorted and grouped hand
///
/// The ranks must already been grouped in pairs, three of a kinds, four of a
/// kinds and sorted in descending value. That ensures canonical combinations
/// which vastly simplifies matching for other combinations.
fn map_combinations<'a>(ranks: &Vec<Rank>) -> Vec<Rank> {
    match ranks.as_slice() {
        // two pairs
        [Rank::Pair(a), Rank::Pair(b), c]
            => {
                vec!(Rank::TwoPairs(*a, *b),c.clone())
            },
        // check for full house
        [Rank::ThreeOfAKind(a), Rank::Pair(b)]
            => vec!(Rank::FullHouse(*a,*b)),
        x => x.to_vec()
    }
}

impl Hand {

    pub fn from_str(hand: &str) -> Option<Hand> {
        let parts = hand.split(" ").collect::<Vec<&str>>();
        let dealt = parts[0].to_string();
        let bid = parts[1].parse::<u64>().unwrap();

        let mut ranks : Vec<Rank> = dealt.chars()
            .filter_map(|c| Card::from_char(c))
            .sorted_by(Card::cmp)
            .group_by(|card| card.clone())
            .into_iter()
            .map(|(_, group)| map_card_groups(group))
            .collect();
        ranks.sort_unstable();
        ranks.reverse();
        let ranks = map_combinations(&ranks);

        Some(Hand{dealt, ranks, bid})
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str ="32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    fn input_data() -> InputModel{
        let hands :Vec<Hand> = TEST_INPUT.lines()
            .filter_map(|line| Hand::from_str(line))
            .collect();
        InputModel{hands}
    }
    
    #[test]
    fn test_card_from_char() {
        assert_eq!(Card::from_char('2'), Some(Card::Two));
        assert_eq!(Card::from_char('3'), Some(Card::Three));
        assert_eq!(Card::from_char('4'), Some(Card::Four));
        assert_eq!(Card::from_char('5'), Some(Card::Five));
        assert_eq!(Card::from_char('6'), Some(Card::Six));
        assert_eq!(Card::from_char('7'), Some(Card::Seven));
        assert_eq!(Card::from_char('8'), Some(Card::Eight));
        assert_eq!(Card::from_char('9'), Some(Card::Nine));
        assert_eq!(Card::from_char('T'), Some(Card::Ten));
        assert_eq!(Card::from_char('J'), Some(Card::Jack));
        assert_eq!(Card::from_char('Q'), Some(Card::Queen));
        assert_eq!(Card::from_char('K'), Some(Card::King));
        assert_eq!(Card::from_char('A'), Some(Card::Ace));
    }

    #[test]
    fn test_hand_from_str() {
        assert_eq!(Hand::from_str("2359K 123"),
                   Some(Hand{
                       dealt: "2359K".to_string(),
                       ranks: vec!(
                           Rank::HighCard(Card::King),
                           Rank::HighCard(Card::Nine),
                           Rank::HighCard(Card::Five),
                           Rank::HighCard(Card::Three),
                           Rank::HighCard(Card::Two)
                       ),
                       bid: 123}));
        assert_eq!(Hand::from_str("T55J5 684"),
                   Some(Hand{dealt: "T55J5".to_string(),
                             ranks: vec!(Rank::ThreeOfAKind(Card::Five),
                                         Rank::HighCard(Card::Jack),
                                         Rank::HighCard(Card::Ten)),
                             bid: 684
                   }));
    }

    #[test]
    fn test_sort_order() {
        let mut hands = input_data().hands;
        hands.sort();
        assert_eq!(hands[0].dealt, "32T3K");
        assert_eq!(hands[1].dealt, "KTJJT");
        assert_eq!(hands[2].dealt, "KK677");
        assert_eq!(hands[3].dealt, "T55J5");
        assert_eq!(hands[4].dealt, "QQQJA");
    }

}
