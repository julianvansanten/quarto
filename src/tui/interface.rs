use std::io::{self, BufRead};

use crate::{
    board::Board,
    printable::{Piece, PrintableBoard},
    ui::{PlayerInterface, Warning},
};

pub struct TextualInterface;

impl TextualInterface {
    /// Private function to ask the user for a u8 input.
    fn ask_for_number(&self) -> u8 {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let mut input = String::new();
        loop {
            match handle.read_line(&mut input).ok() {
                Some(_) => match input.trim().parse::<u8>().ok() {
                    Some(num) => return num - 1,
                    None => println!("\nThat is not a number, please try again."),
                },
                None => println!("\nThat didn't work, please try again."),
            }
        }
    }
}

impl PlayerInterface for TextualInterface {
    /// Ask the user via stdin for the number of the piece.
    fn prompt_for_piece(&self) -> u8 {
        print!("Enter a piece [1-16]: ");
        loop {
            let res = self.ask_for_number();
            if res >= 1 && res <= 16 {
                return res;
            }
            println!("{} is not a valid piece, please try again.", res)
        }
    }

    /// Ask the user via stdin for the move for a given piece.
    fn prompt_for_move(&self, piece: u8) -> u8 {
        print!("Enter a place on the board to put piece {} [1-16]: ", piece);
        loop {
            let res = self.ask_for_number();
            if res >= 1 && res <= 16 {
                return res;
            }
            println!(
                "{} is not a valid place on the board, please try again.",
                res
            )
        }
    }

    /// Ask via stdin if the user wants to call Quarto or not.
    fn ask_quarto(&self) -> bool {
        print!("Call Quarto? [Y/N] ");
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let mut input = String::new();
        loop {
            match handle.read_line(&mut input).ok() {
                Some(_) => match input.trim() {
                    "Y" | "y" => return true,
                    "N" | "n" => return false,
                    _ => println!("Invalid answer, try again"),
                },
                None => println!("\nThat didn't work, please try again."),
            }
        }
    }

    /// Warn the player with a given `Warning`.
    fn warn_player(&self, warning: Warning) {
        match warning {
            Warning::IncorrectPiece(p) => match Piece::from_u8(p) {
                Some(piece) => println!("{} is not a valid piece!", piece),
                None => println!("{} is not a valid number for a piece!", p),
            },
            Warning::IncorrectIndex(i) => println!("{} is not a valid place to put the piece!", i),
        }
    }

    /// Show the game board to the player.
    fn show_game_board(&self, board: &Board) {
        println!("\n{}", PrintableBoard::from_board(*board).string())
    }
}
