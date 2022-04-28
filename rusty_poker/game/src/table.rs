use crate::player::Player;
use deck::deck::Card;
use dealer::dealer::Dealer;

pub struct Table {
    players: Vec<Player>,
    pub dealer: Dealer,
    cards: Vec<Card>,
    money_sum: u16
}

impl Table {

    pub fn new(player_name: String) -> Self {

        let players = vec![
            Player::new(player_name),
            Player::new(String::from("John Starr")),
            Player::new(String::from("Andrew McDoody")),
            Player::new(String::from("Peter Savage")),
        ];

        Table {
            players,
            dealer: Dealer::new(),
            cards: Vec::new(),
            money_sum: 0
        }

    }

}