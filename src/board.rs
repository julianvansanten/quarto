use crate::printable::PrintableBoard;

/// The bit size of a single piece.
const PIECE: u8 = 8;
/// The bits set to check existence in the right-most column.
/// Left-shift `COLUMN` by PIECE per column.
const COLUMN: u128 = 0b1 + (0b1 << 4 * PIECE) + (0b1 << 8 * PIECE) + (0b1 << 12 * PIECE);
/// The bits set to check existence in the lowest row.
/// Left-shift `ROW` by 4 * PIECE per row.
const ROW: u128 = 0b1 + (0b1 << PIECE) + (0b1 << 2 * PIECE) + (0b1 << 3 * PIECE);
/// The bits set to check existence in the down diagonal. 
const DIAG_DOWN: u128 = 0b1 + (0b1 << 5 * PIECE) + (0b1 << 10 * PIECE) + (0b1 << 15 * PIECE);
/// The bits set to check existence in the up diagonal.
const DIAG_UP: u128 = (0b1 << 3 * PIECE) + (0b1 << 6 * PIECE) + (0b1 << 9 * PIECE) + (0b1 << 12 * PIECE);

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

    /// Check if a row on the board is full.
    /// The `row` value must lie between 0 and (incl.) 3.
    fn row(&self, row: u8) -> bool {
        if row > 3 {
            return false;
        }
        let row_mask = ROW << (4 * PIECE * (3 - row));
        self.items & row_mask == row_mask
    }

    /// Check if a column on the board is full.
    /// The `column` value must lie between 0 and (incl.) 3.
    fn column(&self, column: u8) -> bool {
        if column > 3 {
            return false;
        }
        let col_mask = COLUMN << (PIECE * (3 - column));
        self.items & col_mask == col_mask
    }
    
    /// Check if the diagonal is full.
    fn diagonal(&self) -> bool {
        self.items & DIAG_DOWN == DIAG_DOWN || self.items & DIAG_UP == DIAG_UP
    }
}

#[cfg(test)]
mod tests {
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
    fn test_full_row_empty_board() {
        let board = Board::new();
        // Assert that a new board has no full rows.
        for r in 0..4 {
            assert!(!board.row(r));
        }
    }

    #[test]
    fn test_full_column_empty_board() {
        let board = Board::new();
        // Assert that a new board has no full rows.
        for c in 0..4 {
            assert!(!board.column(c));
        }
    }
    
    #[test]
    fn test_full_row_first() {
        let num: u128 = (1 << (127 - 31)) + (1 << (127 - 23)) + (1 << (127 - 15)) + (1 << (127 - 7));
        let board = Board::from_u128(num);
        assert!(board.row(0));
        for row in 1..4 {
            assert!(!board.row(row))
        }
    }
    
    #[test]
    fn test_full_column_first() {
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
}
