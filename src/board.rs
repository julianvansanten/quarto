/// A Quarto board is stored as a `u128`.
/// Each cell is 8 bits, so the entire board is 8 * 16 = 128.
/// Each 8 bits represent a state of the cell: the leftmost 4 bits symbolize the 4 categories, the rightmost bit signals the existence of a piece.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Board {
    items: u128
}

impl Board {
    /// Create a new empty board.
    pub fn new() -> Self {
        Board { items: 0 }
    }
    
    /// Create a `Board` from a number.
    /// This method does not validate the correctness of the board!
    pub fn from(items: u128) -> Self {
        Board { items }
    }
    
    /// Get a copy of the internal `u128` board structure.
    pub fn items(&self) -> u128 {
        self.items
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
}