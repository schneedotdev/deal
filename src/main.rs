use deck::Deck;

mod card;
mod deck;
mod error;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    let drawn = deck.draw();
    match drawn {
        Some(card) => println!("{card:?}"),
        None => println!("Deck is empty."),
    }
}
