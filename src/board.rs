// Author: @julianvansanten
// A bitboard to store the Quarto board.

use crate::printable::PrintableBoard;

/// The bit size of a single piece.
pub const PIECE_SIZE: u8 = 8;
/// The bits set to check existence in the right-most column.
/// Left-shift `COLUMN` by PIECE per column.
const COLUMN: u128 =
    0b1 + (0b1 << 4 * PIECE_SIZE) + (0b1 << 8 * PIECE_SIZE) + (0b1 << 12 * PIECE_SIZE);
/// The bits set to check existence in the lowest row.
/// Left-shift `ROW` by 4 * PIECE per row.
const ROW: u128 = 0b1 + (0b1 << PIECE_SIZE) + (0b1 << 2 * PIECE_SIZE) + (0b1 << 3 * PIECE_SIZE);
/// The bits set to check existence on the whole board.
const BOARD_MASK: u128 =
    COLUMN + (COLUMN << PIECE_SIZE) + (COLUMN << PIECE_SIZE * 2) + (COLUMN << PIECE_SIZE * 3);
/// The bits set to check existence in the down diagonal.
const DIAG_DOWN: u128 =
    0b1 + (0b1 << 5 * PIECE_SIZE) + (0b1 << 10 * PIECE_SIZE) + (0b1 << 15 * PIECE_SIZE);
/// The bits set to check existence in the up diagonal.
const DIAG_UP: u128 = (0b1 << 3 * PIECE_SIZE)
    + (0b1 << 6 * PIECE_SIZE)
    + (0b1 << 9 * PIECE_SIZE)
    + (0b1 << 12 * PIECE_SIZE);

/// A Quarto board is stored as a `u128`.
/// Each cell is 8 bits, so the entire board is 8 * 16 = 128.
/// Each 8 bits represent a state of the cell: the leftmost 4 bits symbolize the 4 categories, the rightmost bit signals the existence of a piece.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Board {
    items: u128,
}

impl Board {
    /// Create a new empty board.
    pub fn new() -> Self {
        Board { items: 0 }
    }

    /// Create a `Board` from a number.
    /// This method does not validate the correctness of the board!
    fn from_u128(items: u128) -> Self {
        Board { items }
    }

    /// Create a `Board` from a `PrintableBoard`.
    pub fn from_printable(pboard: &PrintableBoard) -> Result<Self, &'static str> {
        let pboard_items = pboard.items();
        if pboard_items.len() != 16 {
            return Err("The PrintableBoard does not contain 16 elements!");
        }
        let mut board: Board = Board::new();
        for (i, option) in pboard_items.iter().enumerate() {
            match option {
                // Safely place the items on the board, return an `Err` if there is a duplicate.
                Some(piece) => if !board.put_piece(piece.to_u8(), i as u8) {
                    // TODO: add formatted string that tells why it failed.
                    return Err("Unable to put item on board! Perhaps it already exists?");
                }
                None => continue,
            };
        }
        Ok(board)
    }

    /// Get a copy of the internal `u128` board structure.
    pub fn items(&self) -> u128 {
        self.items
    }

    /// Check if the index on the board is empty.
    pub fn is_empty(&self, index: u8) -> bool {
        if index > 15 {
            return false;
        }
        let pos_mask: u128 = 0b1 << ((15 - index) * PIECE_SIZE);
        self.items & pos_mask == 0
    }

    /// Check if a row on the board is full.
    /// The `row` value must lie between 0 and (incl.) 3.
    fn row(&self, row: u8) -> bool {
        if row > 3 {
            return false;
        }
        let row_mask = ROW << (4 * PIECE_SIZE * (3 - row));
        self.items & row_mask == row_mask
    }

    /// Check if a column on the board is full.
    /// The `column` value must lie between 0 and (incl.) 3.
    fn column(&self, column: u8) -> bool {
        if column > 3 {
            return false;
        }
        let col_mask = COLUMN << (PIECE_SIZE * (3 - column));
        self.items & col_mask == col_mask
    }

    /// Check if a row on the board is full and has blocks with one common characteristic.
    /// The `row` value must lie between 0 and (incl.) 3.
    pub fn winning_row(&self, row: u8) -> bool {
        if !self.row(row) {
            return false;
        }
        for t in 4..8 {
            let row_mask = ROW << (4 * PIECE_SIZE * (3 - row) + t);
            if self.items & row_mask == row_mask || self.items & row_mask == 0 {
                return true;
            }
        }
        false
    }

    /// Check if a column on the board is full and has blocks with one common characteristic.
    /// The `column` value must lie between 0 and (incl.) 3.
    pub fn winning_column(&self, column: u8) -> bool {
        if !self.column(column) {
            return false;
        }
        for t in 4..8 {
            let col_mask = COLUMN << (PIECE_SIZE * (3 - column) + t);
            if self.items & col_mask == col_mask || self.items & col_mask == 0 {
                return true;
            }
        }
        false
    }

    /// Check if a diagonal on the board is full and has blocks with one common characteristic.
    pub fn winning_diagonal(&self) -> bool {
        for t in 4..8 {
            let diag_up_mask = DIAG_UP << t;
            let diag_down_mask = DIAG_DOWN << t;
            let up_and = self.items & diag_up_mask;
            let down_and = self.items & diag_down_mask;
            if self.items & DIAG_UP == DIAG_UP && (up_and == diag_up_mask || up_and == 0) {
                return true;
            }
            if self.items & DIAG_DOWN == DIAG_DOWN && (down_and == diag_down_mask || down_and == 0)
            {
                return true;
            }
        }
        false
    }

    /// Check if the board has a winner.
    /// Return true if there is a row/column/diagonal that is full with winning pieces.
    pub fn has_winner(&self) -> bool {
        // Check all rows and columns first
        for i in 0..4 {
            if self.winning_row(i) || self.winning_column(i) {
                return true;
            }
        }
        // Finally, assume the result depends on the diagonals
        self.winning_diagonal()
    }

    /// Check if the board is full with pieces.
    /// The board is full if all existence bits are set on the entire board.
    pub fn board_full(&self) -> bool {
        // Build a bit mask from the COLUMN
        self.items & BOARD_MASK == BOARD_MASK
    }

    /// Check if the game is over.
    /// The game is over when there is a winning combination, or when the board is full.
    pub fn game_over(&self) -> bool {
        self.has_winner() || self.board_full()
    }

    /// Put a piece (given as a number from 0 to (incl.) 15) on the board at a given index.
    /// Returns true if the piece was placed, false otherwise.
    pub fn put_piece(&mut self, piece: u8, index: u8) -> bool {
        // Cannot put a nonexisting piece on the board, or with an invalid index.
        if index > 15 || !self.valid_piece(piece) {
            return false;
        }
        let bit_index = 15 - index;
        // Cannot put a piece in an existing place.
        if (1 << PIECE_SIZE * bit_index + 1) & self.items != 0 {
            return false;
        }
        // Shift left the existence bit, then shift left the piece type (extra offset of 4 from the existence bit).
        // Finally, add it to the board.
        self.items +=
            (1 << (PIECE_SIZE * bit_index)) + ((piece as u128) << (PIECE_SIZE * bit_index) + 4);
        true
    }

    /// Check if a piece is valid to place on the board.
    /// Loop over the pieces, if a piece exists, check if the values align with that of the piece number.
    pub fn valid_piece(&self, piece: u8) -> bool {
        // Pieces larger than 15 do not exist.
        if piece > 15 {
            return false;
        }
        for p in 0..16 {
            let piece_mask = (piece as u128) << (PIECE_SIZE * p + 4);
            if self.items & (1 << PIECE_SIZE * p) != 0 && self.items & piece_mask == piece_mask {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use crate::printable::Piece;

    use super::*;

    #[test]
    fn test_new_board() {
        let board = Board::new();
        assert_eq!(board.items, 0);
    }

    #[test]
    fn test_items() {
        let board = Board::new();
        assert_eq!(board.items(), 0);
    }

    #[test]
    fn test_row_empty_board() {
        let board = Board::new();
        // Assert that a new board has no full rows.
        for r in 0..4 {
            assert!(!board.row(r));
        }
    }

    #[test]
    fn test_column_empty_board() {
        let board = Board::new();
        // Assert that a new board has no full rows.
        for c in 0..4 {
            assert!(!board.column(c));
        }
    }

    #[test]
    fn test_is_empty_new_board() {
        let board: Board = Board::new();
        for x in 0..16 {
            assert!(board.is_empty(x));
        }
    }

    #[test]
    fn test_is_empty_non_empty_board() {
        let board: Board = Board::from_u128(1);
        assert!(!board.is_empty(15));
        for x in 0..15 {
            assert!(board.is_empty(x));
        }
    }

    #[test]
    fn test_row_first() {
        let num: u128 =
            (1 << (127 - 31)) + (1 << (127 - 23)) + (1 << (127 - 15)) + (1 << (127 - 7));
        let board = Board::from_u128(num);
        assert!(board.row(0));
        for row in 1..4 {
            assert!(!board.row(row))
        }
    }

    #[test]
    fn test_column_first() {
        let num: u128 =
            (1 << (127 - 7)) + (1 << (127 - 39)) + (1 << (127 - 71)) + (1 << (127 - 103));
        let board = Board::from_u128(num);
        assert!(board.column(0));
        for column in 1..4 {
            assert!(!board.column(column))
        }
    }

    #[test]
    fn test_winning_row_empty_board() {
        let board: Board = Board::new();
        for x in 0..4 {
            assert!(!board.winning_row(x))
        }
    }

    #[test]
    fn test_winning_column_empty_board() {
        let board: Board = Board::new();
        for x in 0..4 {
            assert!(!board.winning_column(x))
        }
    }

    #[test]
    fn test_winning_row_winning_row() {
        let mut pboard_items: Vec<Option<Piece>> = Vec::new();
        pboard_items.push(Some(Piece {
            hole: true,
            square: false,
            high: false,
            dark: false,
        }));
        pboard_items.push(Some(Piece {
            hole: true,
            square: true,
            high: false,
            dark: false,
        }));
        pboard_items.push(Some(Piece {
            hole: true,
            square: false,
            high: true,
            dark: false,
        }));
        pboard_items.push(Some(Piece {
            hole: true,
            square: false,
            high: false,
            dark: true,
        }));
        for _ in 0..12 {
            pboard_items.push(None);
        }
        let pboard: PrintableBoard = match PrintableBoard::from_list(pboard_items) {
            Some(board) => board,
            None => panic!("Unable to construct printable board!"),
        };
        let board: Board = match Board::from_printable(&pboard) {
            Ok(b) => b,
            Err(e) => panic!("Failed to construct board! {}", e),
        };
        assert!(board.winning_row(0));
        for i in 1..4 {
            assert!(!board.winning_row(i));
        }
    }

    #[test]
    fn test_winning_row_non_winning_row() {
        let mut pboard_items: Vec<Option<Piece>> = Vec::new();
        pboard_items.push(Some(Piece {
            hole: true,
            square: false,
            high: false,
            dark: false,
        }));
        pboard_items.push(Some(Piece {
            hole: false,
            square: true,
            high: false,
            dark: false,
        }));
        pboard_items.push(Some(Piece {
            hole: false,
            square: false,
            high: true,
            dark: false,
        }));
        pboard_items.push(Some(Piece {
            hole: false,
            square: false,
            high: false,
            dark: true,
        }));
        for _ in 0..12 {
            pboard_items.push(None);
        }
        let pboard: PrintableBoard = match PrintableBoard::from_list(pboard_items) {
            Some(board) => board,
            None => panic!("Unable to construct printable board!"),
        };
        let board: Board = match Board::from_printable(&pboard) {
            Ok(b) => b,
            Err(e) => panic!("Failed to construct board! {}", e),
        };
        for i in 0..4 {
            assert!(!board.winning_row(i));
        }
    }

    #[test]
    fn test_winning_column_winning_column() {
        let mut pboard_items: Vec<Option<Piece>> = Vec::new();
        pboard_items.push(Some(Piece {
            hole: true,
            square: false,
            high: false,
            dark: false,
        }));
        for _ in 0..3 {
            pboard_items.push(None);
        }
        pboard_items.push(Some(Piece {
            hole: true,
            square: true,
            high: false,
            dark: false,
        }));
        for _ in 0..3 {
            pboard_items.push(None);
        }
        pboard_items.push(Some(Piece {
            hole: true,
            square: false,
            high: true,
            dark: false,
        }));
        for _ in 0..3 {
            pboard_items.push(None);
        }
        pboard_items.push(Some(Piece {
            hole: true,
            square: false,
            high: false,
            dark: true,
        }));
        for _ in 0..3 {
            pboard_items.push(None);
        }
        let pboard: PrintableBoard = match PrintableBoard::from_list(pboard_items) {
            Some(board) => board,
            None => panic!("Unable to construct printable board!"),
        };
        let board: Board = match Board::from_printable(&pboard) {
            Ok(b) => b,
            Err(e) => panic!("Failed to construct board! {}", e),
        };
        assert!(board.winning_column(0));
        for i in 1..4 {
            assert!(!board.winning_column(i));
        }
    }

    #[test]
    fn test_winning_column_non_winning_column() {
        let mut pboard_items: Vec<Option<Piece>> = Vec::new();
        pboard_items.push(Some(Piece {
            hole: true,
            square: false,
            high: false,
            dark: false,
        }));
        for _ in 0..3 {
            pboard_items.push(None);
        }
        pboard_items.push(Some(Piece {
            hole: false,
            square: true,
            high: false,
            dark: false,
        }));
        for _ in 0..3 {
            pboard_items.push(None);
        }
        pboard_items.push(Some(Piece {
            hole: false,
            square: false,
            high: true,
            dark: false,
        }));
        for _ in 0..3 {
            pboard_items.push(None);
        }
        pboard_items.push(Some(Piece {
            hole: false,
            square: false,
            high: false,
            dark: true,
        }));
        for _ in 0..3 {
            pboard_items.push(None);
        }
        let pboard: PrintableBoard = match PrintableBoard::from_list(pboard_items) {
            Some(board) => board,
            None => panic!("Unable to construct printable board!"),
        };
        let board: Board = match Board::from_printable(&pboard) {
            Ok(b) => b,
            Err(e) => panic!("Failed to construct board! {}", e),
        };
        for i in 0..4 {
            assert!(!board.winning_column(i));
        }
    }

    #[test]
    fn test_winning_diagonal_empty_board() {
        let board: Board = Board::new();
        assert!(!board.winning_diagonal());
    }

    #[test]
    fn test_winning_diagonal_non_winning() {
        let mut items: Vec<Option<Piece>> = Vec::new();
        items.push(Some(Piece {
            hole: true,
            square: false,
            high: false,
            dark: false,
        }));
        for _ in 0..4 {
            items.push(None);
        }
        items.push(Some(Piece {
            hole: false,
            square: true,
            high: false,
            dark: false,
        }));
        for _ in 0..4 {
            items.push(None);
        }
        items.push(Some(Piece {
            hole: false,
            square: false,
            high: true,
            dark: false,
        }));
        for _ in 0..4 {
            items.push(None);
        }
        items.push(Some(Piece {
            hole: false,
            square: false,
            high: false,
            dark: true,
        }));
        let pboard: PrintableBoard = match PrintableBoard::from_list(items) {
            Some(pb) => pb,
            None => panic!("Unable to create the diagonal board!"),
        };
        let board: Board = match Board::from_printable(&pboard) {
            Ok(b) => b,
            Err(e) => panic!("Unable to construct the board from printable! {}", e),
        };
        assert!(!board.winning_diagonal())
    }

    #[test]
    fn test_winning_diagonal_winning() {
        let mut items: Vec<Option<Piece>> = Vec::new();
        items.push(Some(Piece {
            hole: true,
            square: false,
            high: false,
            dark: false,
        }));
        for _ in 0..4 {
            items.push(None);
        }
        items.push(Some(Piece {
            hole: true,
            square: true,
            high: false,
            dark: false,
        }));
        for _ in 0..4 {
            items.push(None);
        }
        items.push(Some(Piece {
            hole: true,
            square: false,
            high: true,
            dark: false,
        }));
        for _ in 0..4 {
            items.push(None);
        }
        items.push(Some(Piece {
            hole: true,
            square: false,
            high: false,
            dark: true,
        }));
        let pboard: PrintableBoard = match PrintableBoard::from_list(items) {
            Some(pb) => pb,
            None => panic!("Unable to create the diagonal board!"),
        };
        let board: Board = match Board::from_printable(&pboard) {
            Ok(b) => b,
            Err(e) => panic!("Unable to construct the board from printable! {}", e),
        };
        assert!(board.winning_diagonal())
    }

    #[test]
    fn test_put_invalid_piece() {
        let mut board: Board = Board::new();
        assert!(!board.put_piece(16, 0));
        assert_eq!(board.items(), 0);
        assert!(!board.put_piece(0, 16));
        assert_eq!(board.items(), 0);
    }

    #[test]
    fn test_put_duplicate_piece() {
        let mut board: Board = Board::new();
        // First attempt to put piece 0 on the board.
        assert!(board.put_piece(0, 0));
        // Then try to put piece 0 again, but now in a different spot.
        assert!(!board.put_piece(0, 1));
    }

    #[test]
    fn test_put_valid_piece() {
        let mut board: Board = Board::new();
        assert!(board.put_piece(1, 0));
        assert_ne!(board.items(), 0);
        let pboard: PrintableBoard = PrintableBoard::from_board(board);
        let items = pboard.items();
        match items.first() {
            Some(option) => match option {
                Some(piece) => assert_eq!(
                    *piece,
                    Piece {
                        hole: false,
                        square: false,
                        high: false,
                        dark: true
                    }
                ),
                None => panic!("There is no piece in the first spot!"),
            },
            None => panic!("Unable to get first item from the printable board!"),
        }
    }

    #[test]
    fn test_board_full_empty_board() {
        let board: Board = Board::new();
        assert!(!board.board_full());
    }

    #[test]
    fn test_board_full() {
        let mut items: u128 = 0;
        for i in 0..16 {
            items += 1 << (i * PIECE_SIZE);
        }
        let board: Board = Board::from_u128(items);
        assert!(board.board_full());
    }

    #[test]
    fn test_board_full_almost_full() {
        let mut items: u128 = 0;
        // Lets only put 10 pieces on the board.
        for i in 0..10 {
            items += 1 << (i * PIECE_SIZE);
        }
        let board: Board = Board::from_u128(items);
        assert!(!board.board_full());
    }

    #[test]
    fn test_has_winner_new_board() {
        let board: Board = Board::new();
        assert!(!board.has_winner());
    }

    #[test]
    fn test_has_winner_actual_winning() {
        let mut items: Vec<Option<Piece>> = Vec::new();
        // Add 4 items in a row that have a hole and nothing else in common.
        items.push(Some(Piece {
            hole: true,
            square: false,
            high: false,
            dark: false,
        }));
        items.push(Some(Piece {
            hole: true,
            square: true,
            high: false,
            dark: false,
        }));
        items.push(Some(Piece {
            hole: true,
            square: false,
            high: true,
            dark: false,
        }));
        items.push(Some(Piece {
            hole: true,
            square: false,
            high: false,
            dark: true,
        }));
        // Add empty spaces.
        for _ in 0..12 {
            items.push(None);
        }
        let pboard: PrintableBoard = match PrintableBoard::from_list(items) {
            Some(pb) => pb,
            None => panic!("Unable to create printable board!"),
        };
        let board: Board = match Board::from_printable(&pboard) {
            Ok(b) => b,
            Err(e) => panic!("Unable to create board from printable! {}", e),
        };
        assert!(board.has_winner())
    }
}
