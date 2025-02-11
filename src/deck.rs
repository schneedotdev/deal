use rand::{rng, seq::SliceRandom};

use crate::{
    card::{Card, Suit, Value},
    error::DeckError,
};

pub enum State {
    Shuffled,
    Unshuffled,
}

pub enum Arrangement {
    BySuit,
    ByValue,
    Reverse,
}

pub struct Deck {
    playable: Vec<Card>,
    dealt: Vec<Card>,
    discarded: Vec<Card>,
    state: State,
}

impl Deck {
    const DECK_SIZE: usize = 52;

    pub fn new() -> Self {
        let mut deck = Deck {
            playable: Vec::with_capacity(Self::DECK_SIZE),
            dealt: Vec::with_capacity(Self::DECK_SIZE),
            discarded: Vec::with_capacity(Self::DECK_SIZE),
            state: State::Unshuffled,
        };

        deck.arrange(Arrangement::BySuit);
        deck
    }

    pub fn shuffle(&mut self) {
        self.playable.shuffle(&mut rng());
        self.state = State::Shuffled;
    }

    pub fn shuffle_discarded(&mut self) {
        self.discarded.shuffle(&mut rng());
    }

    pub fn arrange(&mut self, arrangement: Arrangement) {
        let mut cards = Vec::with_capacity(Self::DECK_SIZE);

        match arrangement {
            Arrangement::BySuit => {
                for suit in Suit::ALL {
                    for value in Value::ALL {
                        cards.push(Card::new(value, suit));
                    }
                }
            }
            Arrangement::ByValue => {
                for value in Value::ALL {
                    for suit in Suit::ALL {
                        cards.push(Card::new(value, suit));
                    }
                }
            }
            Arrangement::Reverse => {
                for value in Value::ALL.iter().rev() {
                    for suit in Suit::ALL {
                        cards.push(Card::new(*value, suit));
                    }
                }
            }
        }

        self.playable = cards;
        self.state = State::Unshuffled;
    }

    pub fn draw(&mut self) -> Option<Card> {
        if let Some(drawn_card) = self.playable.pop() {
            self.dealt.push(drawn_card.clone());
            return Some(drawn_card);
        }

        None
    }

    // TODO: update if deck size can ever be increased
    pub fn discard(&mut self, card: Card) -> Result<(), DeckError> {
        if self.playable.contains(&card) || self.discarded.contains(&card) {
            return Err(DeckError::IllegalDiscard);
        }

        self.discarded.push(card);
        Ok(())
    }

    // TODO: Discard a card that wasn't previously played
    pub fn discard_unplayed(&mut self, card: Card) {
        unimplemented!()
    }

    pub fn size() -> usize {
        Self::DECK_SIZE
    }
}
