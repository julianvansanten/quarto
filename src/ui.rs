use crate::board::Board;

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
}