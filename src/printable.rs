// Author: @julianvansanten
// An easy to debug board with a list of pieces.
// This implementation uses a lot of memory (reads/writes), so there is only a way to go from this board to the bitboard.

use std::fmt::Display;

use crate::board::{Board, PIECE_SIZE};
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
        // Reverse the for loop to start at the first (leftmost) 8 bits
        for shift in (0..16).rev() {
            let u8piece: u8 = (board.items() >> (PIECE_SIZE * shift) & 255) as u8;
            items.push(Piece::from_u8(u8piece));
        }
        PrintableBoard { items }
    }

    /// Create a deep copy of the items in the board.
    pub fn items(&self) -> Vec<Option<Piece>> {
        let mut res: Vec<Option<Piece>> = Vec::new();
        for option in self.items.iter() {
            res.push(match option {
                // Must be a `clone` to create a copy of the piece!
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
    pub hole: bool,   // fill
    pub square: bool, // shape
    pub high: bool,   // size
    pub dark: bool,   // color
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = if self.high {
            "Large"
        } else {
            "Small"
        };
        let color = if self.dark {
            "dark"
        } else {
            "light"
        };
        let form = if self.square {
            "square"
        } else {
            "circle"
        };
        let hole = if self.hole {
            "with"
        } else {
            "without"
        };
        write!(f, "{}, {} {} {} hole", size, color, form, hole)
    }
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

    /// Create a Piece from a u8, if possible.
    pub fn from_u8(input: u8) -> Option<Self> {
        // If the existence bit is not set, return None.
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

    /// Convert the `Piece` to a number between 0 and (incl.) 15.
    /// This number can be used to place a piece on the board.
    pub fn to_number(&self) -> u8 {
        let mut res: u8 = 0;
        res += (self.hole as u8) << 3;
        res += (self.square as u8) << 2;
        res += (self.high as u8) << 1;
        res += self.dark as u8;
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
    fn test_pp_to_number() {
        let piece = match Piece::from_u8(1) {
            Some(piece) => piece,
            None => panic!("This is not supposed to happen, check Piece::from_u8()!"),
        };
        assert_eq!(piece.to_number(), 0)
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
