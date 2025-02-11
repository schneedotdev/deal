// TODO: This file represents the default card format for a deck of playing cards.
// Build deck so that it can work with any Card type a user creates, so long as it
// implements necessary traits.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    value: Value,
    suit: Suit,
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Self {
        Card { value, suit }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Suit {
    pub const ALL: [Suit; 4] = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Ace,
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
