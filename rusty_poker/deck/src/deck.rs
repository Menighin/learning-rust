use std::fmt;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub enum Card {
    Hearts(String),
    Spades(String),
    Clubs(String),
    Diamonds(String)
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Card::Clubs(v) => write!(f, "{: >2} ♣", v),
            Card::Diamonds(v) => write!(f, "{: >2} ♦", v),
            Card::Hearts(v) => write!(f, "{: >2} ♥", v),
            Card::Spades(v) => write!(f, "{: >2} ♠", v)
        }
    }
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {

    pub fn new() -> Self {

        let mut cards: Vec<Card> = Vec::new();
        
        for i in 2..11 {
            cards.push(Card::Clubs(i.to_string()));
            cards.push(Card::Diamonds(i.to_string()));
            cards.push(Card::Hearts(i.to_string()));
            cards.push(Card::Spades(i.to_string()));
        }

        cards.push(Card::Clubs(String::from("J")));
        cards.push(Card::Diamonds(String::from("J")));
        cards.push(Card::Hearts(String::from("J")));
        cards.push(Card::Spades(String::from("J")));

        cards.push(Card::Clubs(String::from("Q")));
        cards.push(Card::Diamonds(String::from("Q")));
        cards.push(Card::Hearts(String::from("Q")));
        cards.push(Card::Spades(String::from("Q")));

        cards.push(Card::Clubs(String::from("K")));
        cards.push(Card::Diamonds(String::from("K")));
        cards.push(Card::Hearts(String::from("K")));
        cards.push(Card::Spades(String::from("K")));

        cards.push(Card::Clubs(String::from("A")));
        cards.push(Card::Diamonds(String::from("A")));
        cards.push(Card::Hearts(String::from("A")));
        cards.push(Card::Spades(String::from("A")));

        Self { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn deal(&mut self) -> Option<Card> {
        return self.cards.pop()
    }

}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in &self.cards {
            write!(f, "{} ", card)?;
        }
        return Ok(())
    }
}

