use crate::board::Board;
/// Representation for the board that is easier to print.
/// Uses `Some(Piece)`s to store each piece, is easier to print but way slower to operate on.
/// If there is no Piece on a location, we store a `None`.
#[derive(Debug, PartialEq, Eq)]
pub struct PrintableBoard {
    items: Vec<Option<Piece>>,
}

impl PrintableBoard {
    /// Wrap a `Vec<Option`Piece>>` in a new PrintableBoard.
    /// Returns `Some(PrintableBoard)` if the list contains exactly 16 entries, otherwise None.
    pub fn from_list(items: Vec<Option<Piece>>) -> Option<Self> {
        // Boards must have 16 spaces.
        if items.len() != 16 {
            return None;
        }
        Some(PrintableBoard { items })
    }

    /// Create a `PrintableBoard` from a `Board`.
    /// Take the `u128`, shift over the bits, AND it with 255 and turn it into a `u8`.
    pub fn from_board(board: Board) -> Self {
        let mut items: Vec<Option<Piece>> = Vec::new();
        // Reverse the for loop to start at the first 8 bits
        for shift in (0..16).rev() {
            let u8piece: u8 = (board.items() >> (8 * shift) & 255) as u8;
            items.push(Piece::from_u8(u8piece));
        }
        PrintableBoard { items }
    }
    
    /// Create a deep copy of the items in the board.
    pub fn items(&self) -> Vec<Option<Piece>> {
        let mut res: Vec<Option<Piece>> = Vec::new();
        for option in self.items.iter() {
            res.push(match option {
                Some(p) => Some(p.clone()),
                None => None,
            });
        }
        res
    }
}

/// A Piece on the board that can be printed, but is not necessarily used in the Board structure (slow).
/// There are 16 Pieces in Quarto, with each piece having a hole/no hole, being square/round, being high/low, and dark/light.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Piece {
    // This order is coherent with the order of the networking protocol.
    hole: bool,   // fill
    square: bool, // shape
    high: bool,   // size
    dark: bool,   // color
}

impl Piece {
    /// Create a Piece from the four properties.
    pub fn new(hole: bool, square: bool, high: bool, dark: bool) -> Self {
        Piece {
            hole,
            square,
            high,
            dark,
        }
    }

    /// Create a Piece from a u8, if possible
    pub fn from_u8(input: u8) -> Option<Self> {
        // If the existence bit is not set, return None
        if input & 1u8 == 0 {
            return None;
        }
        Some(Piece {
            hole: input & (1 << 7) == (1 << 7),
            square: input & (1 << 6) == (1 << 6),
            high: input & (1 << 5) == (1 << 5),
            dark: input & (1 << 4) == (1 << 4),
        })
    }

    /// Create a u8 from this Piece, for debugging only.
    pub fn to_u8(&self) -> u8 {
        let mut res: u8 = 1;
        res += (self.hole as u8) << 7;
        res += (self.square as u8) << 6;
        res += (self.high as u8) << 5;
        res += (self.dark as u8) << 4;
        res
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use super::*;

    #[test]
    fn test_pp_exists() {
        // When the existence bit is not set, there is no piece
        for x in 1u8..8u8 {
            assert_eq!(Piece::from_u8(1u8 << x), None);
        }
        // When the existence bit is set, there should be a piece
        for x in 1u8..8u8 {
            assert_ne!(Piece::from_u8((1u8 << x) + 1), None);
        }
    }

    #[test]
    fn pp_from_u8() {
        // When only the existence bit is set, all items should be empty.
        assert_eq!(
            Piece::from_u8(1),
            Some(Piece {
                hole: false,
                square: false,
                high: false,
                dark: false
            })
        );
        assert_eq!(
            Piece::from_u8(129),
            Some(Piece {
                hole: true,
                square: false,
                high: false,
                dark: false
            })
        );
    }

    #[test]
    fn test_pp_to_u8() {
        let piece = match Piece::from_u8(1) {
            Some(piece) => piece,
            None => panic!("This is not supposed to happen, check Piece::from_u8()!"),
        };
        assert_eq!(piece.to_u8(), 1)
    }

    #[test]
    fn test_board_conversion_arb_length() {
        assert_eq!(PrintableBoard::from_list(Vec::new()), None);
    }

    #[test]
    fn test_board_conversion_correct_list() {
        let mut pieces: Vec<Option<Piece>> = Vec::new();
        for i in 0..16 {
            let piece: Option<Piece> = Piece::from_u8((i << 4) + 1);
            pieces.push(piece);
        }
        
        let pboard: PrintableBoard = match PrintableBoard::from_list(pieces) {
            Some(pboard) => pboard,
            None => panic!("PrintableBoard not correctly initialized!"),
        };
        
        let board: Board = match Board::from_printable(&pboard) {
            Ok(b) => b,
            Err(_) => panic!("Board conversion failed!"),
        };
        
        match Board::from_printable(&PrintableBoard::from_board(board)) {
            Ok(board2) => assert_eq!(board, board2),
            Err(_) => panic!("Double conversion failed!"),
        };
    }
}
