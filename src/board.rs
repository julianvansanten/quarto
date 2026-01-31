/// A Quarto board is stored as a `u128`.
/// Each cell is 8 bits, so the entire board is 8 * 16 = 128.
/// Each 8 bits represent a state of the cell: the leftmost 4 bits symbolize the 4 categories, the rightmost bit signals the existence of a piece.
pub struct Board {
    items: u128
}

impl Board {
    /// Create a new empty board
    fn new() -> Self {
        Board { items: 0 }
    }
}

/// Representation for the board that is printable.
/// Uses Some(Piece)s to store each piece, is easier to print but way slower to operate on.
/// If there is no Piece on a location, we store a None.
pub struct PrintableBoard {
    items: Vec<Option<Piece>>
}



/// A Piece on the board that can be printed, but is not necessarily used in the Board structure (slow)
#[derive(Debug, PartialEq, Eq)]
pub struct Piece {
    light: bool,
    hole: bool,
    rectangle: bool,
    high: bool,
}

impl Piece {
    
    /// Create a PrintablePiece
    fn new(light: bool, hole: bool, rectangle: bool, high: bool) -> Self {
        Piece { light, hole, rectangle, high }
    }
    
    /// Create a PrintablePiece from a u8, if possible
    fn from_u8(input: u8) -> Option<Self> {
        // If the existence bit is not set, return None
        if input & 1u8 == 0 {
            return None;
        }
        Some(Piece {
            light: input & 128u8 == 128u8,
            hole: input & 64u8 == 64u8,
            rectangle: input & 32u8 == 32u8,
            high: input & 16u8 == 16u8,
        })
    }
    
    /// Create a u8 from this Piece, for debugging only.
    fn to_u8(&self) -> u8 {
        let mut res: u8 = 1;
        if self.light {
            res += 128;
        }
        if self.hole {
            res += 64;
        }
        if self.rectangle {
            res += 32;
        }
        if self.high {
            res += 16;
        }
        res
    }
}

#[cfg(test)]
mod tests {
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
                light: false,
                hole: false,
                rectangle: false,
                high: false
            })
        );
        assert_eq!(
            Piece::from_u8(129),
            Some(Piece {
                light: true,
                hole: false,
                rectangle: false,
                high: false
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
}
