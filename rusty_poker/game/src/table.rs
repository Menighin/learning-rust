use crate::player::Player;
use deck::deck::Card;
use dealer::dealer::Dealer;

pub struct Table {
    pub players: Vec<Player>,
    pub dealer: Dealer,
    cards: Vec<Card>,
    cards_flipped: u8,
    player_start: u8,
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
            cards_flipped: 0,
            player_start: 0,
            money_sum: 0
        }

    }

}