use deck::deck::Deck;
use std::io;
pub mod player;
pub mod table;
pub mod poker;
pub mod display;


fn main() {

    println!("Please type your name: ");

    let mut player_name = String::new();
    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read line");


    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let poker_game = poker::PokerGame::new(player_name);


    // while !poker_game.has_ended() {
        poker_game.game_step();
    // }

}
