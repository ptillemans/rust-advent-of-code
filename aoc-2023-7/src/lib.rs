use itertools::Itertools;
use std::cmp::Ordering;
use std::ops::Sub;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct InputModel {
    pub hands: Vec<Hand>,
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}

impl FromStr for InputModel {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands: Vec<Hand> = s.lines().filter_map(|line| line.parse().ok()).collect();
        Ok(InputModel { hands })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
/// the possible values of the cards
pub enum Card {
    Joker,
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
    Ace,
}

impl Card {
    /// the value of a card can be determined from the first
    /// character of the notation for it
    fn from_char(c: char) -> Option<Card> {
        match c {
            '_' => Some(Card::Joker),
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
            _ => None,
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
    HighCard,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, Clone)]
/// a poker hand is a collection of combinations or ranks
pub struct Hand {
    /// sorted vector of ranks, most valuable first
    ranks: Vec<Rank>,

    /// original hand dealt
    dealt: Vec<Card>,

    /// the bid
    pub bid: u64,
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
        let mut ord = self.ranks.cmp(&other.ranks);
        if ord == Ordering::Equal {
            ord = self.dealt.cmp(&other.dealt);
        }
        Some(ord)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

/// Map groups of consecutive cards with the same value
fn map_card_groups<'a>(group: impl Iterator<Item = Card>) -> Rank {
    let cards: Vec<Card> = group.collect();
    match cards.len() {
        1 => Rank::HighCard,
        2 => Rank::Pair,
        3 => Rank::ThreeOfAKind,
        4 => Rank::FourOfAKind,
        5 => Rank::FiveOfAKind,
        _ => panic!("Invalid number of cards in group"),
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
        [Rank::Pair, Rank::Pair, c] => {
            vec![Rank::TwoPairs, c.clone()]
        }
        // check for full house
        [Rank::ThreeOfAKind, Rank::Pair] => vec![Rank::FullHouse],
        x => x.to_vec(),
    }
}

fn to_cards(s: &str) -> Vec<Card> {
    s.chars()
        .map(|c| Card::from_char(c))
        .collect::<Option<Vec<Card>>>()
        .unwrap()
}

fn to_cards_with_joker(s: &str) -> Vec<Card> {
    s.chars()
        .filter_map(|c| match Card::from_char(c) {
            Some(Card::Jack) => Some(Card::Joker),
            Some(card) => Some(card),
            None => None,
        })
        .collect::<Vec<Card>>()
}

impl FromStr for Hand {
    type Err = AocError;

    fn from_str(hand: &str) -> Result<Hand, AocError> {
        let parts = hand.split(" ").collect::<Vec<&str>>();
        let dealt = to_cards(parts[0]);
        let bid = parts[1].parse::<u64>().expect("Invalid bid");

        let mut ranks: Vec<Rank> = calc_ranks(&dealt);

        Ok(Hand { dealt, ranks, bid })
    }
}

fn calc_ranks(cards: &Vec<Card>) -> Vec<Rank> {
    let ranks: Vec<Rank> = cards
        .iter()
        .sorted()
        .cloned()
        .group_by(|card| card.clone())
        .into_iter()
        .map(|(_, group)| map_card_groups(group))
        .sorted()
        .rev()
        .collect();
    let ranks = map_combinations(&ranks);
    ranks
}

impl Hand {
    pub fn with_cards(&self, dealt: Vec<Card>) -> Hand {
        let mut ranks: Vec<Rank> = dealt
            .iter()
            .sorted()
            .cloned()
            .group_by(|card| card.clone())
            .into_iter()
            .map(|(_, group)| map_card_groups(group))
            .collect();
        ranks.sort_unstable();
        ranks.reverse();
        let ranks = map_combinations(&ranks);
        let bid = self.bid;

        Hand { dealt, ranks, bid }
    }
}

pub fn compare_card_with_jokers(a: &Card, b: &Card) -> Ordering {
    match (a, b) {
        (Card::Jack, Card::Jack) => Ordering::Equal,
        (Card::Jack, _) => Ordering::Less,
        (_, Card::Jack) => Ordering::Greater,
        (a, b) => a.cmp(&b),
    }
}

const JOKER_CARDS: [Card; 12] = [
    Card::Two,
    Card::Three,
    Card::Four,
    Card::Five,
    Card::Six,
    Card::Seven,
    Card::Eight,
    Card::Nine,
    Card::Ten,
    Card::Queen,
    Card::King,
    Card::Ace,
];


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Hand2 {
    cards: Vec<Card>,

    pub bid: u64,
    strongest: Vec<Card>,
    ranks: Vec<Rank>,
}

    
fn strongest_hand(hand: &[Card]) -> Vec<Card> {
    let strongest = (0..hand.len())
        .fold(hand.to_vec(), |cards, i| match cards[i] {
        Card::Joker => JOKER_CARDS
            .iter()
            .map(|card| {
                let mut new_cards = cards.clone();
                new_cards[i] = *card;
                strongest_hand(&new_cards)
            })
                .max_by(|a, b| {
                    let a_ranks = calc_ranks(a);
                    let b_ranks = calc_ranks(b);
                    let ord = a_ranks.cmp(&b_ranks);
                    if ord == Ordering::Equal {
                        a.cmp(b)
                    } else {
                       ord 
                    }
                })
                .unwrap(),
        _ => cards
    });

    strongest
}

        
impl From<Hand> for Hand2 {

    fn from(hand: Hand) -> Hand2 {
        let cards = hand.dealt.iter()
            .map(|card| match card {
                Card::Jack => Card::Joker,
                _ => *card,
            })
            .collect::<Vec<Card>>();
        let strongest = strongest_hand(&cards);
        let ranks = calc_ranks(&strongest);
        let hand2 = Hand2 {
            cards,
            strongest,
            ranks,
            bid: hand.bid,
        };
        hand2

    }
}

impl FromStr for Hand2 {
    type Err = AocError;

    fn from_str(hand: &str) -> Result<Hand2, AocError> {
        hand.parse::<Hand>().map(Hand2::from)
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut ord = self.ranks.cmp(&other.ranks);
        if ord == Ordering::Equal {
            ord = self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(a, b)| compare_card_with_jokers(a, b))
                .fold(Ordering::Equal, |acc, ord| {
                    if acc == Ordering::Equal {
                        ord
                    } else {
                        acc
                    }
                });
        };
        Some(ord)
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    fn input_data() -> InputModel {
        let hands: Vec<Hand> = TEST_INPUT
            .lines()
            .filter_map(|line| line.parse::<Hand>().ok())
            .collect();
        InputModel { hands }
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
        assert_eq!(
            "2359K 123".parse::<Hand>().unwrap(),
            Hand {
                dealt: to_cards("2359K"),
                ranks: vec!(
                    Rank::HighCard,
                    Rank::HighCard,
                    Rank::HighCard,
                    Rank::HighCard,
                    Rank::HighCard
                ),
                bid: 123
            }
        );
        assert_eq!(
            "T55J5 684".parse::<Hand>().unwrap(),
            Hand {
                dealt: to_cards("T55J5"),
                ranks: vec!(Rank::ThreeOfAKind, Rank::HighCard, Rank::HighCard),
                bid: 684
            }
        );
    }

    #[test]
    fn test_sort_order() {
        let mut hands = input_data().hands;
        hands.sort();
        assert_eq!(hands[0].dealt, to_cards("32T3K"));
        assert_eq!(hands[1].dealt, to_cards("KTJJT"));
        assert_eq!(hands[2].dealt, to_cards("KK677"));
        assert_eq!(hands[3].dealt, to_cards("T55J5"));
        assert_eq!(hands[4].dealt, to_cards("QQQJA"));
    }

    #[test]
    fn test_strongest_hand() {
        let hands = input_data().hands.into_iter()
            .map(Hand2::from)
            .collect::<Vec<Hand2>>();
        let expected: Vec<Hand2> = vec![
            "32T3K 765".parse::<Hand2>().unwrap(),
            "T5555 674".parse::<Hand2>().unwrap(),
            "KK677 28".parse::<Hand2>().unwrap(),
            "KTTTT 220".parse::<Hand2>().unwrap(),
            "QQQQA 483".parse::<Hand2>().unwrap(),
        ];
        hands
            .iter()
            .zip(expected.iter())
            .for_each(|(hand, expected)| {
                println!("hand: {:?}", hand);
                let actual = hand.strongest.clone();
                assert_eq!(actual, *expected.cards);
            });
    }

    #[test]
    fn test_compare_with_jokers() {
        let a = "KTJJT 123".parse::<Hand2>().unwrap();
        let b = "KK677 123".parse::<Hand2>().unwrap();
        assert_eq!(a.cmp(&b), Ordering::Greater);
    }

    #[test]
    fn test_sort_order_2() {
        let hands = input_data().hands;
        let mut hands2 = hands.into_iter().map(Hand2::from).collect::<Vec<Hand2>>();
        hands2.sort();
        assert_eq!(hands2[0].cards, to_cards_with_joker("32T3K"));
        assert_eq!(hands2[1].cards, to_cards_with_joker("KK677"));
        assert_eq!(hands2[2].cards, to_cards_with_joker("T55J5"));
        assert_eq!(hands2[3].cards, to_cards_with_joker("QQQJA"));
        assert_eq!(hands2[4].cards, to_cards_with_joker("KTJJT"));
    }
}
