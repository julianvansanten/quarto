use crate::printable::PrintableBoard;

/// The bit size of a single piece.
const PIECE_SIZE: u8 = 8;
/// The bits set to check existence in the right-most column.
/// Left-shift `COLUMN` by PIECE per column.
const COLUMN: u128 = 0b1 + (0b1 << 4 * PIECE_SIZE) + (0b1 << 8 * PIECE_SIZE) + (0b1 << 12 * PIECE_SIZE);
/// The bits set to check existence in the lowest row.
/// Left-shift `ROW` by 4 * PIECE per row.
const ROW: u128 = 0b1 + (0b1 << PIECE_SIZE) + (0b1 << 2 * PIECE_SIZE) + (0b1 << 3 * PIECE_SIZE);
/// The bits set to check existence in the down diagonal. 
const DIAG_DOWN: u128 = 0b1 + (0b1 << 5 * PIECE_SIZE) + (0b1 << 10 * PIECE_SIZE) + (0b1 << 15 * PIECE_SIZE);
/// The bits set to check existence in the up diagonal.
const DIAG_UP: u128 = (0b1 << 3 * PIECE_SIZE) + (0b1 << 6 * PIECE_SIZE) + (0b1 << 9 * PIECE_SIZE) + (0b1 << 12 * PIECE_SIZE);

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
    pub fn from_u128(items: u128) -> Self {
        Board { items }
    }

    /// Create a `Board` from this `PrintableBoard`.
    pub fn from_printable(pboard: &PrintableBoard) -> Result<Self, &'static str> {
        let pboard_items = pboard.items();
        if pboard_items.len() != 16 {
            return Err("The PrintableBoard does not contain 16 elements!");
        }
        let mut items: u128 = 0;
        for (i, option) in pboard_items.iter().enumerate() {
            match option {
                Some(piece) => items += (piece.to_u8() as u128) << ((15 - i) * 8),
                None => continue,
            };
        }
        Ok(Board::from_u128(items))
    }

    /// Get a copy of the internal `u128` board structure.
    pub fn items(&self) -> u128 {
        self.items
    }
    
    /// Check if the index on the board is empty.
    pub fn is_empty(&self, index: u8) -> bool {
        if index > 15 {
            return false
        }
        let pos_mask: u128 = 0b1 << ((15 - index) * PIECE_SIZE);
        self.items & pos_mask == 0
    }

    /// Check if a row on the board is full.
    /// The `row` value must lie between 0 and (incl.) 3.
    fn row(&self, row: u8) -> bool {
        if row > 3 {
            return false
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
    
    /// Check if the diagonal is full.
    fn diagonal(&self) -> bool {
        self.items & DIAG_DOWN == DIAG_DOWN || self.items & DIAG_UP == DIAG_UP
    }
    
    /// Check if a row on the board is full and has blocks with at least one characteristic.
    /// The `row` value must lie between 0 and (incl.) 3.
    pub fn full_row(&self, row: u8) -> bool {
        if !self.row(row) {
            return false
        }
        for t in 4..8 {
            let row_mask = ROW << (4 * PIECE_SIZE * (3 - row) + t);
            if self.items & row_mask == row_mask || self.items & row_mask == 0 {
                return true
            }
        }
        false
    }
    
    /// Check if a column on the board is full and has blocks with at least one characteristic.
    /// The `column` value must lie between 0 and (incl.) 3.
    pub fn full_column(&self, column: u8) -> bool {
        if !self.column(column) {
            return false
        }
        for t in 4..8 {
            let col_mask = COLUMN << (PIECE_SIZE * (3 - column) + t);
            if self.items & col_mask == col_mask || self.items & col_mask == 0 {
                return true
            }
        }
        false
    }
    
    pub fn full_diagonal(&self) -> bool {
        if !self.diagonal() {
            return false
        }
        todo!("Implement diagonal check for existing items")
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
        let num: u128 = (1 << (127 - 31)) + (1 << (127 - 23)) + (1 << (127 - 15)) + (1 << (127 - 7));
        let board = Board::from_u128(num);
        assert!(board.row(0));
        for row in 1..4 {
            assert!(!board.row(row))
        }
    }
    
    #[test]
    fn test_column_first() {
        let num: u128 = (1 << (127 - 7)) + (1 << (127 - 39)) + (1 << (127 - 71)) + (1 << (127 - 103));
        let board = Board::from_u128(num);
        assert!(board.column(0));
        for column in 1..4 {
            assert!(!board.column(column))
        }
    }
    
    #[test]
    fn test_diagonals() {
        let board: Board = Board::new();
        assert!(!board.diagonal());
        let num: u128 = 1 + (1 << (127 - 87)) + (1 << (127 - 47)) + (1 << (127 - 7));
        let board: Board = Board::from_u128(num);
        assert!(board.diagonal());
        let num: u128 = (1 << (127 - 103)) + (1 << (127 - 79)) + (1 << (127 - 55)) + (1 << (127 - 31));
        let board: Board = Board::from_u128(num);
        assert!(board.diagonal());
    }
    
    #[test]
    fn test_full_row_empty_board() {
        let board: Board = Board::new();
        for x in 0..4 {
            assert!(!board.full_row(x))
        }
    }
    
    #[test]
    fn test_full_column_empty_board() {
        let board: Board = Board::new();
        for x in 0..4 {
            assert!(!board.full_column(x))
        }
    }
    
    #[test]
    fn test_full_row_winning_row() {
        let mut pboard_items: Vec<Option<Piece>> = Vec::new();
        pboard_items.push(Some(Piece{
            hole: true,
            square: false,
            high: false,
            dark: false,
        }));
        pboard_items.push(Some(Piece{
            hole: true,
            square: true,
            high: false,
            dark: false,
        }));
        pboard_items.push(Some(Piece{
            hole: true,
            square: false,
            high: true,
            dark: false,
        }));
        pboard_items.push(Some(Piece{
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
        assert!(board.full_row(0));
        for i in 1..4 {
            assert!(!board.full_row(i));
        }
    }
    
    #[test]
    fn test_full_row_non_winning_row() {
        let mut pboard_items: Vec<Option<Piece>> = Vec::new();
        pboard_items.push(Some(Piece{
            hole: true,
            square: false,
            high: false,
            dark: false,
        }));
        pboard_items.push(Some(Piece{
            hole: false,
            square: true,
            high: false,
            dark: false,
        }));
        pboard_items.push(Some(Piece{
            hole: false,
            square: false,
            high: true,
            dark: false,
        }));
        pboard_items.push(Some(Piece{
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
            assert!(!board.full_row(i));
        }
    }
    
    #[test]
    fn test_full_column_winning_column() {
        let mut pboard_items: Vec<Option<Piece>> = Vec::new();
        pboard_items.push(Some(Piece{
            hole: true,
            square: false,
            high: false,
            dark: false,
        }));
        for _ in 0..3 {
            pboard_items.push(None);
        }
        pboard_items.push(Some(Piece{
            hole: true,
            square: true,
            high: false,
            dark: false,
        }));
        for _ in 0..3 {
            pboard_items.push(None);
        }
        pboard_items.push(Some(Piece{
            hole: true,
            square: false,
            high: true,
            dark: false,
        }));
        for _ in 0..3 {
            pboard_items.push(None);
        }
        pboard_items.push(Some(Piece{
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
        assert!(board.full_column(0));
        for i in 1..4 {
            assert!(!board.full_column(i));
        }
    }
    
    #[test]
    fn test_full_column_non_winning_column() {
        let mut pboard_items: Vec<Option<Piece>> = Vec::new();
        pboard_items.push(Some(Piece{
            hole: true,
            square: false,
            high: false,
            dark: false,
        }));
        for _ in 0..3 {
            pboard_items.push(None);
        }
        pboard_items.push(Some(Piece{
            hole: false,
            square: true,
            high: false,
            dark: false,
        }));
        for _ in 0..3 {
            pboard_items.push(None);
        }
        pboard_items.push(Some(Piece{
            hole: false,
            square: false,
            high: true,
            dark: false,
        }));
        for _ in 0..3 {
            pboard_items.push(None);
        }
        pboard_items.push(Some(Piece{
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
            assert!(!board.full_column(i));
        }
    }
    
    #[test]
    fn test_full_diagonal() {
        todo!("This must be implemented still!")
    }
}
