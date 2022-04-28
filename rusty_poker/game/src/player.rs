use deck::deck::Card;

pub struct Player {
    name: String,
    money: u16,
    cards: Vec<Card>
}

impl Player {

    pub fn new(name: String) -> Self {
        Player {
            name,
            money: 1000,
            cards: Vec::new()
        }
    }

}