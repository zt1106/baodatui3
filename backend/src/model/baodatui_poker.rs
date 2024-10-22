use crate::model::poker::Suit;

fn score_of_numeric_number(num: u32) -> u32 {
    if num % 5 == 0 {
        return num;
    }
    if num == 13 {
        return 10;
    }
    return 0;
}

fn raw_power_of_numeric_number(num: u32) -> i32 {
    return if num == 1 { 14 } else { num as i32 };
}

pub struct PrimeOrSub {
    pub prime_suit: Option<Suit>,
    sub_suit: Option<Suit>,
}

impl PrimeOrSub {
    pub fn of_prime_suit(prime_suit: Option<Suit>) -> Self {
        Self {
            prime_suit,
            sub_suit: None,
        }
    }

    pub fn of_sub_suit(sub_suit: Suit) -> Self {
        Self {
            prime_suit: None,
            sub_suit: Some(sub_suit),
        }
    }

    pub fn is_prime(&self) -> bool {
        return match self.sub_suit {
            Some(_) => false,
            None => true,
        };
    }

    pub fn is_sub(&self) -> bool {
        !self.is_prime()
    }
}
