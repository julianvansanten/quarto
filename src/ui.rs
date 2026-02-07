use crate::{board::Board, printable::Piece};

pub enum Warning {
    IncorrectPiece(Piece),
    IncorrectIndex(u8)
}

/// Any interface for the `HumanPlayer` should implement these functions.
pub trait PlayerInterface {
    /// Get the piece to play from the interface.
    /// This function **must** return a number.
    fn prompt_for_piece(&self, board: &Board) -> u8;
    /// Get the move from the interface.
    /// This function **must** return a number.
    fn prompt_for_move(&self, board: &Board, piece: u8) -> u8;
    /// Ask if the player wants to call Quarto via the interface.
    fn ask_quarto(&self, board: &Board) -> bool;
    
    fn warn_player(&self, warning: Warning);
}

pub struct TextualInterface;

impl PlayerInterface for TextualInterface {
    fn prompt_for_piece(&self, board: &Board) -> u8 {
        todo!()
    }

    fn prompt_for_move(&self, board: &Board, piece: u8) -> u8 {
        todo!()
    }

    fn ask_quarto(&self, board: &Board) -> bool {
        todo!()
    }

    fn warn_player(&self, warning: Warning) {
        match warning {
            Warning::IncorrectPiece(p) => println!("{} is not a valid piece!", p),
            Warning::IncorrectIndex(i) => println!("{} is not a valid place to put the piece!", i),
        }
    }
}