use crate::board::Board;

/// A `Strategy` determines how the `ComputerPlayer` determines thw piece for the opponents, and its own moves.
/// It also allows a different implementation for calling Quarto.
pub trait Strategy {
    /// Calculate which piece the opponent should use.
    fn get_piece(&self, board: &Board) -> Option<u8>;

    /// Calculate the next move on the board.
    fn get_move(&self, board: &Board, piece: u8) -> Option<u8>;

    /// Calculate the decision to make for calling Quarto.
    /// Can be implemented smart (always and only call Quarto on first win), or naive (e.g. 1/10 chance the `Strategy` forgets to call Quarto).
    fn quarto(&self, board: &Board) -> bool;
}


pub struct DumbStrategy;
pub struct NaiveStrategy;
pub struct SmartStrategy;
pub struct DeterministicStrategy;

impl Strategy for DumbStrategy {
    /// Select a random piece for the opponent.
    fn get_piece(&self, board: &Board) -> Option<u8> {
        let valid_pieces = board.valid_pieces();
        if valid_pieces.is_empty() {
            return None;
        }
        let i = fastrand::usize(..valid_pieces.len());
        Some(valid_pieces[i])
    }

    /// Select a random place to put the piece on.
    /// This implementation just ignores what piece to place now.
    fn get_move(&self, board: &Board, _: u8) -> Option<u8> {
        let empty_spaces = board.empty_spaces();
        if empty_spaces.is_empty() {
            return None;
        }
        let i = fastrand::usize(..empty_spaces.len());
        Some(empty_spaces[i])
    }

    /// Be dumb and do not call Quarto on 1/10 of the winning moments.
    fn quarto(&self, board: &Board) -> bool {
        if board.has_winner() && fastrand::usize(0..10) != 0 {
            return true;
        }
        false
    }
}

impl Strategy for NaiveStrategy {
    /// Select a random piece for the opponent.
    fn get_piece(&self, board: &Board) -> Option<u8> {
        let valid_pieces = board.valid_pieces();
        if valid_pieces.is_empty() {
            return None;
        }
        let i = fastrand::usize(..valid_pieces.len());
        Some(valid_pieces[i])
    }

    /// Select a random place to put the piece on.
    /// This implementation just ignores what piece to place now.
    fn get_move(&self, board: &Board, _: u8) -> Option<u8> {
        let empty_spaces = board.empty_spaces();
        if empty_spaces.is_empty() {
            return None;
        }
        let i = fastrand::usize(..empty_spaces.len());
        Some(empty_spaces[i])
    }

    /// Always call Quarto when the board has a winner.
    fn quarto(&self, board: &Board) -> bool {
        board.has_winner()
    }
}

impl Strategy for SmartStrategy {
    fn get_piece(&self, board: &Board) -> Option<u8> {
        todo!("SmartStrategy not yet implemented!")
    }

    fn get_move(&self, board: &Board, piece: u8) -> Option<u8> {
        todo!("SmartStrategy not yet implemented!")
    }

    fn quarto(&self, board: &Board) -> bool {
        todo!("SmartStrategy not yet implemented!")
    }
}

impl Strategy for DeterministicStrategy {
    /// Select a random piece for the opponent.
    fn get_piece(&self, board: &Board) -> Option<u8> {
        let valid_pieces = board.valid_pieces();
        if valid_pieces.is_empty() {
            return None;
        }
        Some(valid_pieces[0])
    }

    /// Select a random place to put the piece on.
    /// This implementation just ignores what piece to place now.
    fn get_move(&self, board: &Board, _: u8) -> Option<u8> {
        let empty_spaces = board.empty_spaces();
        if empty_spaces.is_empty() {
            return None;
        }
        Some(empty_spaces[0])
    }

    /// Always call Quarto when the board has a winner.
    fn quarto(&self, board: &Board) -> bool {
        board.has_winner()
    }
}