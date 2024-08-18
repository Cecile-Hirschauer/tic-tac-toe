use crate::errors::TicTacToeError;
use anchor_lang::prelude::*; // Importing the core functionality of Anchor.
use num_derive::*; // Importing macros for deriving numerical enums.
use num_traits::*; // Importing traits for handling numerical enums.

// Account structure for storing the game's state.
#[account]
pub struct Game {
    players: [Pubkey; 2],          // (32 * 2) - Public keys of the two players.
    turn: u8,                      // 1 - Current turn.
    board: [[Option<Sign>; 3]; 3], // 9 * (1 + 1) = 18 - 3x3 board with optional signs.
    state: GameState,              // 32 + 1 - Current state of the game.
}

// Implementation of methods associated with the Game struct.
impl Game {
    // Maximum size of the Game struct in bytes.
    pub const MAXIMUM_SIZE: usize = (32 * 2) + 1 + (9 * (1 + 1) + (32 + 1));

    // Starts a new game by initializing the players.
    pub fn start(&mut self, players: [Pubkey; 2]) -> Result<()> {
        require_eq!(self.turn, 0, TicTacToeError::GameAlreadyStarted); // Ensure the game hasn't already started.
        self.players = players; // Assign the players.
        self.turn = 1; // Start the first turn.
        Ok(())
    }

    // Checks if the game is currently active.
    pub fn is_active(&mut self) -> bool {
        self.state == GameState::Active
    }

    // Calculates the index of the current player (0 or 1).
    fn current_player_index(&mut self) -> usize {
        ((self.turn - 1) % 2) as usize
    }

    // Returns the public key of the current player.
    pub fn current_player(&mut self) -> Pubkey {
        self.players[self.current_player_index()]
    }

    // Executes a move by placing the current player's sign on the specified tile.
    pub fn play(&mut self, tile: &Tile) -> Result<()> {
        // Ensure the game is still active.
        require!(self.is_active(), TicTacToeError::GameAlreadyOver);

        // Validate the tile and ensure it's not already occupied, then place the player's sign.
        match tile {
            tile @ Tile {
                row: 0..=2,
                column: 0..=2,
            } => match self.board[tile.row as usize][tile.column as usize] {
                Some(_) => return Err(TicTacToeError::TileAlreadySet.into()), // The tile is already occupied.
                None => {
                    self.board[tile.row as usize][tile.column as usize] =
                        Some(Sign::from_usize(self.current_player_index()).unwrap())
                    // Place the player's sign.
                }
            },
            _ => return Err(TicTacToeError::TileOutOfBounds.into()), // The tile is out of bounds.
        }

        // Update the game state after the move.
        self.update_state();

        // If the game is still active, increment the turn.
        if GameState::Active == self.state {
            self.turn += 1;
        }

        Ok(())
    }

    // Checks if a trio of tiles forms a winning combination.
    fn is_winning_trio(&self, trio: [(usize, usize); 3]) -> bool {
        let [first, second, third] = trio;
        self.board[first.0][first.1].is_some()
            && self.board[first.0][first.1] == self.board[second.0][second.1]
            && self.board[first.0][first.1] == self.board[third.0][third.1]
    }

    // Updates the game state based on the current board.
    fn update_state(&mut self) {
        for i in 0..=2 {
            // Three of the same in one row
            if self.is_winning_trio([(i, 0), (i, 1), (i, 2)]) {
                self.state = GameState::Won {
                    winner: self.current_player(),
                };
                return;
            }

            // Three of the same in one column
            if self.is_winning_trio([(0, i), (1, i), (2, i)]) {
                self.state = GameState::Won {
                    winner: self.current_player(),
                };
                return;
            }
        }

        // Three of the same in a diagonal
        if self.is_winning_trio([(0, 0), (1, 1), (2, 2)])
            || self.is_winning_trio([(0, 2), (1, 1), (2, 0)])
        {
            self.state = GameState::Won {
                winner: self.current_player(),
            };
            return;
        }

        // Reaching this point means the game hasn't been won yet,
        // so if there are any unfilled tiles left, the game is still active.
        for row in 0..=2 {
            for column in 0..=2 {
                if self.board[row][column].is_none() {
                    return;
                }
            }
        }

        // If all tiles are filled and there's no winner, the game ends in a tie.
        self.state = GameState::Tie;
    }
}

// Struct representing a tile on the board.
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Tile {
    row: u8,
    column: u8,
}

// Enum representing the different possible states of the game.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Pubkey },
}

// Enum representing the possible signs (X or O) in the game.
#[derive(
    AnchorSerialize, AnchorDeserialize, FromPrimitive, ToPrimitive, Copy, Clone, PartialEq, Eq,
)]
pub enum Sign {
    X,
    O,
}
