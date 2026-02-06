// Author: @julianvansanten
// Players that can play the Quarto game.
// Uses the `Board` to determine the moves.

use crate::board::Board;

/// An abstraction of a `Player` that can play Quarto.
/// The implementation should at least be able to get the piece for the opponent, the move to make, and the call for Quarto.
pub trait Player {
    /// Get the piece for the opponent to play.
    fn get_piece(&self, board: &Board) -> Option<u8>;

    /// Decide the move of this player where to place the given piece.
    fn get_move(&self, board: &Board, piece: u8) -> Option<u8>;

    /// Ask the player if they wish to call Quarto.
    fn quarto(&self, board: &Board) -> bool;
}

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

// TODO: add a BufReader for a `HumanPlayer`.
pub struct HumanPlayer;
pub struct ComputerPlayer<T: Strategy> {
    /// A `ComputerPlayer` uses a `Strategy` to determine its decisions.
    strategy: T,
}

impl<T: Strategy> ComputerPlayer<T> {
    /// Create a new `ComputerPlayer` with a given `Strategy`.
    pub fn new(strategy: T) -> ComputerPlayer<T> {
        ComputerPlayer { strategy }
    }
}

impl Player for HumanPlayer {
    /// Ask the player for the piece to play.
    fn get_piece(&self, board: &Board) -> Option<u8> {
        todo!()
    }

    /// Ask the player for the move to make, based on a given piece.
    fn get_move(&self, board: &Board, piece: u8) -> Option<u8> {
        todo!()
    }

    fn quarto(&self, board: &Board) -> bool {
        todo!()
    }
}

pub struct DumbStrategy;
pub struct NaiveStrategy;
pub struct SmartStrategy;

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

/// Use the `Strategy` `T` to determine the moves.
impl<T: Strategy> Player for ComputerPlayer<T> {
    fn get_piece(&self, board: &Board) -> Option<u8> {
        self.strategy.get_piece(board)
    }

    fn get_move(&self, board: &Board, piece: u8) -> Option<u8> {
        self.strategy.get_move(board, piece)
    }

    fn quarto(&self, board: &Board) -> bool {
        self.strategy.quarto(board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_get_move_from_dumb_full_board() {
        let mut board: Board = Board::new();
        for i in 0..16 {
            board.put_piece(i, i);
        }
        let player = ComputerPlayer {
            strategy: DumbStrategy,
        };
        match player.get_move(&board, 0) {
            Some(n) => panic!(
                "Strategy came back with number {}, while there is no valid space!",
                n
            ),
            None => (),
        }
    }

    #[test]
    fn test_get_piece_from_dumb_full_board() {
        let mut board: Board = Board::new();
        for i in 0..16 {
            board.put_piece(i, i);
        }
        let player = ComputerPlayer {
            strategy: DumbStrategy,
        };
        match player.get_piece(&board) {
            Some(n) => panic!(
                "Strategy came back with number {}, while there is no valid space!",
                n
            ),
            None => (),
        }
    }

    #[test]
    fn test_get_move_from_naive_full_board() {
        let mut board: Board = Board::new();
        for i in 0..16 {
            board.put_piece(i, i);
        }
        let player = ComputerPlayer {
            strategy: NaiveStrategy,
        };
        match player.get_move(&board, 0) {
            Some(n) => panic!(
                "Strategy came back with number {}, while there is no valid space!",
                n
            ),
            None => (),
        }
    }

    #[test]
    fn test_get_piece_from_naive_full_board() {
        let mut board: Board = Board::new();
        for i in 0..16 {
            board.put_piece(i, i);
        }
        let player = ComputerPlayer {
            strategy: NaiveStrategy,
        };
        match player.get_piece(&board) {
            Some(n) => panic!(
                "Strategy came back with number {}, while there is no valid space!",
                n
            ),
            None => (),
        }
    }

    #[test]
    fn test_get_move_from_dumb_nearly_full_board() {
        let mut board: Board = Board::new();
        for i in 0..15 {
            board.put_piece(i, i);
        }
        let player = ComputerPlayer {
            strategy: DumbStrategy,
        };
        match player.get_move(&board, 0) {
            Some(n) => assert_eq!(n, 15),
            None => panic!("Strategy gave no move, but the board still has an empty space!"),
        }
    }

    #[test]
    fn test_get_piece_from_dumb_nearly_full_board() {
        let mut board: Board = Board::new();
        for i in 0..15 {
            board.put_piece(i, i);
        }
        let player = ComputerPlayer {
            strategy: DumbStrategy,
        };
        match player.get_piece(&board) {
            Some(n) => assert_eq!(n, 15),
            None => panic!("Strategy gave no piece, but the board still has an empty space!"),
        }
    }

    #[test]
    fn test_get_move_from_naive_nearly_full_board() {
        let mut board: Board = Board::new();
        for i in 0..15 {
            board.put_piece(i, i);
        }
        let player = ComputerPlayer {
            strategy: NaiveStrategy,
        };
        match player.get_move(&board, 0) {
            Some(n) => assert_eq!(n, 15),
            None => panic!("Strategy gave no move, but the board still has an empty space!"),
        }
    }

    #[test]
    fn test_get_piece_from_naive_nearly_full_board() {
        let mut board: Board = Board::new();
        for i in 0..15 {
            board.put_piece(i, i);
        }
        let player = ComputerPlayer {
            strategy: NaiveStrategy,
        };
        match player.get_piece(&board) {
            Some(n) => assert_eq!(n, 15),
            None => panic!("Strategy gave no piece, but the board still has an empty space!"),
        }
    }

    #[test]
    fn test_get_move_from_dumb_empty_board() {
        let board: Board = Board::new();
        let player = ComputerPlayer {
            strategy: DumbStrategy,
        };
        match player.get_move(&board, 0) {
            Some(m) => assert!(m < 16),
            None => panic!("Strategy gave no move, but the board still has an empty space!"),
        }
    }

    #[test]
    fn test_get_piece_from_dumb_empty_board() {
        let board: Board = Board::new();
        let player = ComputerPlayer {
            strategy: DumbStrategy,
        };
        match player.get_piece(&board) {
            Some(m) => assert!(m < 16),
            None => panic!("Strategy gave no move, but the board still has an empty space!"),
        }
    }

    #[test]
    fn test_get_move_from_naive_empty_board() {
        let board: Board = Board::new();
        let player = ComputerPlayer {
            strategy: NaiveStrategy,
        };
        match player.get_move(&board, 0) {
            Some(m) => assert!(m < 16),
            None => panic!("Strategy gave no move, but the board still has an empty space!"),
        }
    }

    #[test]
    fn test_get_piece_from_naive_empty_board() {
        let board: Board = Board::new();
        let player = ComputerPlayer {
            strategy: NaiveStrategy,
        };
        match player.get_piece(&board) {
            Some(m) => assert!(m < 16),
            None => panic!("Strategy gave no move, but the board still has an empty space!"),
        }
    }
}
