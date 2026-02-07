// Author: @julianvansanten
// Players that can play the Quarto game.
// Uses the `Board` to determine the moves.

use crate::printable::Piece;
use crate::ui::Warning::{IncorrectIndex, IncorrectPiece};
use crate::{board::Board, strategy::Strategy, ui::PlayerInterface};

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

pub struct HumanPlayer<I: PlayerInterface> {
    // A `HumanPlayer` needs an interface that can ask questions and get responses.
    interface: I
}
pub struct ComputerPlayer<T: Strategy> {
    /// A `ComputerPlayer` uses a `Strategy` to determine its decisions.
    strategy: T,
}

impl<I: PlayerInterface> HumanPlayer<I> {
    /// Create a new HumanPlayer with a given interface.
    pub fn new(interface: I) -> Self {
        HumanPlayer { interface }
    }
}

impl<T: Strategy> ComputerPlayer<T> {
    /// Create a new `ComputerPlayer` with a given `Strategy`.
    pub fn new(strategy: T) -> ComputerPlayer<T> {
        ComputerPlayer { strategy }
    }
}

impl<I: PlayerInterface> Player for HumanPlayer<I> {
    
    /// Ask the player for the piece to play.
    /// Validate the piece and ask (via the interface) for a new piece if it is wrong.
    fn get_piece(&self, board: &Board) -> Option<u8> {
        let valid_pieces = board.valid_pieces();
        if valid_pieces.is_empty() {
            return None
        }
        let mut piece = self.interface.prompt_for_piece(board);
        while !board.valid_piece(piece) {
            // TODO: fix this question
            // self.interface.warn_player(IncorrectPiece(Piece::from_u8(piece)));
            piece = self.interface.prompt_for_piece(board);
        }
        Some(piece)
    }

    /// Ask the player for the move to make, based on a given piece.
    /// Validate the move and ask (via the interface) for a new move if it is wrong.
    fn get_move(&self, board: &Board, piece: u8) -> Option<u8> {
        let empty_spaces = board.empty_spaces();
        if empty_spaces.is_empty() {
            return None
        }
        let mut get_move = self.interface.prompt_for_move(board, piece);
        while !board.empty_index(get_move) {
            self.interface.warn_player(IncorrectIndex(get_move));
            get_move = self.interface.prompt_for_move(board, piece);
        }
        Some(self.interface.prompt_for_move(board, piece))
    }

    /// Ask the user via the interface if they wish to call Quarto.
    fn quarto(&self, board: &Board) -> bool {
        self.interface.ask_quarto(board)
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
    use crate::strategy::{DumbStrategy, NaiveStrategy};

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
