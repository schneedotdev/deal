#[derive(Debug, Clone, Copy)]
pub struct Card {
    value: Value,
    suit: Suit,
    discarded: bool,
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Self {
        Card {
            value,
            suit,
            discarded: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Suit {
    pub const ALL: [Suit; 4] = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Value {
    Ace = 1,
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
}

impl Value {
    pub const ALL: [Value; 13] = [
        Value::Ace,
        Value::Two,
        Value::Three,
        Value::Four,
        Value::Five,
        Value::Six,
        Value::Seven,
        Value::Eight,
        Value::Nine,
        Value::Ten,
        Value::Jack,
        Value::Queen,
        Value::King,
    ];
}
