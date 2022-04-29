use crate::table::Table;
use crate::display;
use std::io;

#[derive(Eq, PartialEq)]
pub enum PlayerAction {
    Pass,
    Fold,
    Bet(u16),
    Raise(u16)
}

#[derive(Eq, PartialEq)]
pub enum GameState {
    GameStart,
    DealCards,
    RoundStart,
    PlayerTurn(u8, PlayerAction),
    Flip,
    CheckWinner,
    RoundEnd,
    CheckPlayers,
    EndGame
}

pub struct PokerGame {
    table: Table,
    state: GameState
}

impl PokerGame {

    pub fn new(player_name: String) -> Self {
        PokerGame {
            table: Table::new(player_name),
            state: GameState::GameStart
        }
    }

    pub fn has_ended(&self) -> bool {
        self.state == GameState::EndGame
    }

    pub fn game_step(&self) {
        match self.state {
            GameState::GameStart => self.game_start(),
            GameState::DealCards => self.game_start(),
            GameState::RoundStart => self.game_start(),
            GameState::PlayerTurn(player, ref action) => self.game_start(),
            GameState::Flip => self.game_start(),
            GameState::CheckWinner => self.game_start(),
            GameState::RoundEnd => self.game_start(),
            GameState::CheckPlayers => self.game_start(),
            GameState::EndGame => self.game_start(),
        }
    }

    fn game_start(&self) {
        display::display_table(&self.table, String::from("This is a test message"))
    }

    // fn print_table(&self) {
    //     println!("{}", self.table.dealer.deck);

    //     let mut discard = String::new();
    //     io::stdin().read_line(&mut discard).expect("Failed to read line");

    //     print!("\x1B[2J\x1B[1;1H");

    //     println!("It worked D:");
    //     io::stdin().read_line(&mut discard).expect("Failed to read line");

    // }
    
}