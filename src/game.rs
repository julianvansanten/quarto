use crate::{board::Board, player::Player};

/// A QuartoGame has two players, an index for the current player, and the board.
pub struct QuartoGame {
    players: [Box<dyn Player>; 2],
    current: usize,
    board: Board,
}

#[derive(Debug, PartialEq, Eq)]
/// A result of a game of Quarto.
/// The game can result in a draw, a win for a given player number, or a game error occurring when a given player was playing.
pub enum GameResult {
    Draw,
    Error(usize),
    Win(usize),
}

impl QuartoGame {
    /// Build a new `QuartoGame`.
    /// There are two `Player` types, that both have the `Player` trait and a known size at runtime.
    pub fn new<P1, P2>(player1: P1, player2: P2) -> Self
    where
        P1: Player + 'static,
        P2: Player + 'static,
    {
        Self {
            players: [Box::new(player1), Box::new(player2)],
            current: 0,
            board: Board::new(),
        }
    }

    /// Advance the game to the next player.
    fn next_player(&mut self) {
        self.current = 1 - self.current;
    }

    /// Play the `QuartoGame` once, without asking players to call Quarto.
    /// Return the winner, `Draw` if it is a draw, and `Error` if the game ended pre-emptively due to an error.
    pub fn play_without_call(&mut self) -> GameResult {
        while !self.board.game_over() {
            let piece: u8 = match self.players[self.current].get_piece(&self.board) {
                Some(p) => p,
                None => return GameResult::Error(self.current),
            };
            self.next_player();
            let player_move = match self.players[self.current].get_move(&self.board, piece) {
                Some(m) => m,
                None => return GameResult::Error(self.current),
            };
            self.board.put_piece(piece, player_move);
        }
        if self.board.has_winner() {
            return GameResult::Win(self.current);
        }
        GameResult::Draw
    }

    /// Get the internal representation of the Board.
    pub fn board(&self) -> Board {
        self.board
    }

    /// Reset the game, keeping the same players.
    pub fn reset(&mut self) {
        self.board = Board::new();
        self.current = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::player::ComputerPlayer;
    use crate::strategy::{DeterministicStrategy, DumbStrategy};

    use super::*;

    #[test]
    fn test_new_game_empty_board() {
        let player1 = ComputerPlayer::new(DumbStrategy);
        let player2 = ComputerPlayer::new(DumbStrategy);
        let game = QuartoGame::new(player1, player2);
        assert!(game.board.is_empty());
        assert_eq!(game.current, 0);
    }

    #[test]
    fn test_play_game_without_call_with_dumb_bots() {
        let player1 = ComputerPlayer::new(DumbStrategy);
        let player2 = ComputerPlayer::new(DumbStrategy);
        let mut game = QuartoGame::new(player1, player2);
        let res = game.play_without_call();
        assert_ne!(res, GameResult::Error(0));
        assert_ne!(res, GameResult::Error(1));
    }

    #[test]
    fn test_play_game_without_call_with_deterministic_bots() {
        let player1 = ComputerPlayer::new(DeterministicStrategy);
        let player2 = ComputerPlayer::new(DeterministicStrategy);
        let mut game = QuartoGame::new(player1, player2);
        let res = game.play_without_call();
        assert_ne!(res, GameResult::Error(0));
        assert_ne!(res, GameResult::Error(1));
    }

    #[test]
    fn test_reset_game() {
        let player1 = ComputerPlayer::new(DeterministicStrategy);
        let player2 = ComputerPlayer::new(DeterministicStrategy);
        let mut game = QuartoGame::new(player1, player2);
        game.play_without_call();
        game.reset();
        assert!(game.board().is_empty());
    }
}
