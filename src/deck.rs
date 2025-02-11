use crate::card::{Card, Suit, Value};

const DECK_SIZE: usize = 52;

struct Deck {
    playable: Vec<Card>,
    discarded: Vec<Card>,
}

impl Deck {
    fn new(self) -> Self {
        Deck {
            playable: Deck::build(),
            discarded: Vec::with_capacity(DECK_SIZE),
        }
    }

    fn build() -> Vec<Card> {
        let mut cards = Vec::with_capacity(52);

        for suit in Suit::ALL {
            for value in Value::ALL {
                cards.push(Card::new(value, suit));
            }
        }

        cards
    }

    pub fn draw(&mut self) -> Option<Card> {
        if let Some(drawn_card) = self.playable.pop() {
            self.discarded.push(drawn_card.clone());
            return Some(drawn_card);
        }

        None
    }
}
