use crate::{board::Board, player::Player};

pub struct QuartoGame<P: Player> {
    players: [P; 2],
    current: usize,
    board: Board,
}

pub enum GameResult {
    Error,
    Draw,
    Win(usize)
}

impl<P: Player> QuartoGame<P> {
    /// Build a new `QuartoGame`.
    pub fn new(player1: P, player2: P) -> Self {
        let players = [player1, player2];
        QuartoGame {
            players,
            current: 0,
            board: Board::new(),
        }
    }
    
    /// Advance the game to the next player.
    fn next_player(&mut self) {
        self.current = 1 - self.current;
    }

    /// Play the `QuartoGame` once. Return the winner, `Draw` if it is a draw, and `Error` if the game ended pre-emptively due to an error.
    pub fn play(&mut self) -> GameResult {
        let mut piece: u8 = 16;
        while !self.board.game_over() {
            if !self.board.is_empty() {
                let player_move = match self.players[self.current].get_move(&self.board, piece) {
                    Some(m) => m,
                    None => return GameResult::Error,
                };
                self.board.put_piece(piece, player_move);
            }
            piece = match self.players[self.current].get_piece(&self.board) {
                Some(p) => p,
                None => return GameResult::Error,
            };
            self.next_player();
        }
        if self.board.has_winner() {
            return GameResult::Win(self.current);
        }
        GameResult::Draw
    }
}

#[cfg(test)]
mod tests {
    use crate::player::{ComputerPlayer, DumbStrategy};

    use super::*;

    #[test]
    fn test_new_game_empty_board() {
        let player1 = ComputerPlayer::new(DumbStrategy);
        let player2 = ComputerPlayer::new(DumbStrategy);
        let game = QuartoGame::new(player1, player2);
        assert!(game.board.is_empty());
        assert_eq!(game.current, 0)
    }
    
    // TODO: add play game tests, with mocking.
}
