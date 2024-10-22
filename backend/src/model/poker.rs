use enum_iterator::{all, Sequence};
use std::sync::OnceLock;

static CARDS: OnceLock<Cards> = OnceLock::new();

pub fn cards() -> &'static Cards {
    CARDS.get_or_init(Cards::default)
}

#[test]
fn test_cards() {
    let cards = cards();
    assert_eq!(cards.cards.len(), 54);
}

#[derive(Debug, Copy, Clone, Sequence)]
pub enum Color {
    RED,
    BLACK,
}

#[derive(Debug, Sequence, Copy, Clone, PartialEq, Eq)]
pub enum Suit {
    DIAMONDS,
    CLUBS,
    HEARTS,
    SPADES,
}

impl Suit {
    pub fn color(&self) -> Color {
        return match self {
            Suit::DIAMONDS => Color::RED,
            Suit::CLUBS => Color::BLACK,
            Suit::HEARTS => Color::RED,
            Suit::SPADES => Color::BLACK,
        };
    }
}

pub struct Cards {
    pub cards: [Card; 54],
}

impl Default for Cards {
    fn default() -> Self {
        let mut cards_vec: Vec<Card> = vec![];
        let mut intrinsic_id = 0;
        for suit in all::<Suit>() {
            for numeric_num in 1..14 {
                cards_vec.push(Card::of_numeric_num_and_suit(
                    suit,
                    numeric_num,
                    intrinsic_id,
                ));
                intrinsic_id += 1;
            }
        }
        cards_vec.push(Card::of_black_joker());
        cards_vec.push(Card::of_red_joker());
        let cards: [Card; 54] = cards_vec.try_into().unwrap();
        Self { cards }
    }
}

impl Cards {
    pub fn by_id(&self, id: u32) -> &Card {
        &self.cards[(id as usize) % 54]
    }
}

#[derive(Debug)]
pub struct Card {
    pub intrinsic_id: u32,
    pub suit: Option<Suit>,
    pub color: Color,
    // pub score: u32,
    // pub raw_power: i32,
    pub numeric_card_num: Option<u32>,
}

impl Card {
    fn of_numeric_num_and_suit(suit: Suit, numeric_card_num: u32, intrinsic_id: u32) -> Self {
        Self {
            intrinsic_id,
            suit: Some(suit),
            color: suit.color(),
            // score: score_of_numeric_number(numeric_card_num),
            // raw_power: raw_power_of_numeric_number(numeric_card_num),
            numeric_card_num: Some(numeric_card_num),
        }
    }

    fn of_red_joker() -> Self {
        Self {
            intrinsic_id: 53,
            suit: None,
            color: Color::RED,
            // score: 0,
            // raw_power: 16,
            numeric_card_num: None,
        }
    }

    fn of_black_joker() -> Self {
        Self {
            intrinsic_id: 52,
            suit: None,
            color: Color::BLACK,
            // score: 0,
            // raw_power: 15,
            numeric_card_num: None,
        }
    }

    pub fn is_joker(&self) -> bool {
        self.intrinsic_id == 52 || self.intrinsic_id == 53
    }

    // pub fn is_prime(&self, prime_suit: Option<Suit>) -> bool {
    //     if self.is_flex_prime() {
    //         return true;
    //     }
    //     return match prime_suit {
    //         Some(p) => p == self.suit.unwrap(),
    //         None => false,
    //     };
    // }

    // pub fn is_prime_suit(&self, prime_suit: Option<Suit>) -> bool {
    //     self.suit == prime_suit
    // }
    //
    // pub fn is_flex_prime(&self) -> bool {
    //     if self.is_joker() {
    //         return true;
    //     }
    //     let num = self.numeric_card_num.unwrap();
    //     num == 2 || num == 3 || num == 5
    // }

    // pub fn is_prime_five(&self, prime_suit: Option<Suit>) -> bool {
    //     if self.is_joker() {
    //         return false;
    //     }
    //     if self.numeric_card_num.unwrap() != 5 {
    //         return false;
    //     }
    //     return match prime_suit {
    //         Some(s) => s == self.suit.unwrap(),
    //         None => false,
    //     };
    // }
    //
    // pub fn real_power(&self, prime_suit: Option<Suit>) -> i32 {
    //     if self.is_prime_five(prime_suit) {
    //         return 10000000;
    //     }
    //     let mut pow = self.raw_power;
    //     if self.is_prime(prime_suit) {
    //         if self.is_flex_prime() {
    //             pow *= 1000;
    //             if self.is_prime_suit(prime_suit) {
    //                 pow += 1;
    //             }
    //         } else {
    //             pow *= 100;
    //         }
    //     }
    //     pow
    // }
}
