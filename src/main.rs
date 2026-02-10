// Author: @julianvansanten
// Quarto game!

use crate::{
    game::QuartoGame,
    player::{ComputerPlayer, HumanPlayer},
    strategy::{DumbStrategy, NaiveStrategy},
    tui::interface::TextualInterface,
    ui::PlayerInterface,
};

pub mod board;
pub mod game;
pub mod player;
pub mod printable;
pub mod strategy;
pub mod tui;
pub mod ui;

fn main() {
    println!("Welcome to Quarto!");
    let player1 = HumanPlayer::new(TextualInterface);
    let player2 = ComputerPlayer::new(NaiveStrategy);
    let mut game = QuartoGame::new(player1, player2);
    for _ in 0..100000 {
        match game.play_without_call() {
            game::GameResult::Error => panic!("The game panicked!"),
            game::GameResult::Draw => println!("The game ended in a draw!"),
            game::GameResult::Win(p) => println!("Player {} has won this game!", p),
        }
        game.reset();
    }
}
