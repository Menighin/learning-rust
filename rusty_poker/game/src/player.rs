use deck::deck::Card;

pub struct Player {
    pub name: String,
    pub is_playing: bool,
    pub money: u16,
    cards: Vec<Card>
}

impl Player {

    pub fn new(name: String) -> Self {
        Player {
            name,
            is_playing: true,
            money: 1000,
            cards: Vec::new()
        }
    }

}