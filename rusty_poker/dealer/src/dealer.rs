use deck::deck::Deck;

#[derive(Debug)]
pub struct Dealer {
    pub deck: Deck
}

impl Dealer {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();

        Dealer {
            deck
        }
    }
}