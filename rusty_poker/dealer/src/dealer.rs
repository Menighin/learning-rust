use deck::deck;

pub fn deal_cards() {

    let mut deck = deck::Deck::new();

    println!("Before shuffle: {}", deck);

    deck.shuffle();

    println!("After shuffle: {}", deck);

    println!("Dealt: {}", deck.deal().expect("Wat"));

    println!("After deal: {}", deck);

}