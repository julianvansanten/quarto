use std::io::{self, BufRead};

use crate::{
    board::Board,
    printable::{Piece, PrintableBoard},
};

pub enum Warning {
    IncorrectPiece(u8),
    IncorrectIndex(u8),
}

/// Any interface for the `HumanPlayer` should implement these functions.
pub trait PlayerInterface {
    /// Show the game board to the player.
    fn show_game_board(&self, board: &Board);
    /// Get the piece to play from the interface.
    /// This function **must** return a number.
    fn prompt_for_piece(&self) -> u8;
    /// Get the move from the interface.
    /// This function **must** return a number.
    fn prompt_for_move(&self, piece: u8) -> u8;
    /// Ask if the player wants to call Quarto via the interface.
    fn ask_quarto(&self) -> bool;
    /// Warn the player they made a mistake at game logic level.
    fn warn_player(&self, warning: Warning);
}
