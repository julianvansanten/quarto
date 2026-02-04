use crate::board::Board;

/// An abstraction of a `Player` that can play Quarto.
/// The implementation should at least be able to get the piece for the opponent, the move to make, and the call for Quarto.
pub trait Player {
    /// Get the piece for the opponent to play.
    fn get_piece(&self, board: &Board) -> u8;
    
    /// Decide the move of this player where to place the given piece.
    fn get_move(&self, board: &Board, piece: u8) -> u8;
    
    /// Ask the player if they wish to call Quarto.
    fn quarto(&self, board: &Board) -> bool;
}

/// A `Strategy` determines how the `ComputerPlayer` determines thw piece for the opponents, and its own moves.
/// It also allows a different implementation for calling Quarto.
pub trait Strategy {
    
    /// Calculate which piece the opponent should use.
    fn get_piece(board: &Board) -> u8;
    
    /// Calculate the next move on the board.
    fn get_move(board: &Board) -> u8;
    
    /// Calculate the decision to make for calling Quarto.
    /// Can be implemented smart (always and only call Quarto on first win), or naive (e.g. 1/10 chance the `Strategy` forgets to call Quarto).
    fn quarto(board: &Board) -> bool;
}

pub struct HumanPlayer;
pub struct ComputerPlayer<T: Strategy> {
    /// A `ComputerPlayer` uses a `Strategy` to determine its decisions.
    strategy: T,
}

impl Player for HumanPlayer {
    
    /// Ask the player for the piece to play.
    fn get_piece(&self, board: &Board) -> u8 {
        todo!()
    }

    /// Ask the player for the move to make, based on a given piece.
    fn get_move(&self, board: &Board, piece: u8) -> u8 {
        todo!()
    }
    
    fn quarto(&self, board: &Board) -> bool {
        todo!()
    }
}

impl<T: Strategy> Player for ComputerPlayer<T> {
    
    fn get_piece(&self, board: &Board) -> u8 {
        todo!()
    }

    fn get_move(&self, board: &Board, piece: u8) -> u8 {
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
    fn get_piece(board: &Board) -> u8 {
        todo!()
    }

    fn get_move(board: &Board) -> u8 {
        todo!()
    }

    fn quarto(board: &Board) -> bool {
        todo!()
    }
}