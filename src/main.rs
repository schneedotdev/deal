use deal::deck::Deck;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    println!("Drawing card...");
    if let Some(card) = deck.draw() {
        println!("Drew the {:?}.", card);

        println!("Discarding the {card:?}...");

        if deck.discard(card).is_ok() {
            println!("Card was discarded.");
        } else {
            println!("Failed to discard card.");
        }
    } else {
        println!("Deck is empty.");
    }
}
