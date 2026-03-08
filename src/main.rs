// Author: @julianvansanten
// Quarto game!

use crate::{
    quarto::{
        game::{self, QuartoGame},
        player::{ComputerPlayer, HumanPlayer},
        strategy::NaiveStrategy,
    },
    tui::interface::TextualInterface,
};

pub mod net;
pub mod quarto;
pub mod tui;
pub mod ui;

fn main() {
    println!("Welcome to Quarto!");
    let player1 = HumanPlayer::new(TextualInterface);
    let player2 = ComputerPlayer::new(NaiveStrategy);
    let mut game = QuartoGame::new(player1, player2);
    match game.play_without_call() {
        game::GameResult::Error(p) => {
            panic!("The game panicked when player {} was supposed to play!", p)
        }
        game::GameResult::Draw => println!("The game ended in a draw!"),
        game::GameResult::Win(p) => println!("Player {} has won this game!", p + 1),
    }
}
