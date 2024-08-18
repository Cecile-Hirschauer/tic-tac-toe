use anchor_lang::error_code;

// Custom error codes for the Tic-Tac-Toe game.
#[error_code]
pub enum TicTacToeError {
    TileOutOfBounds,
    TileAlreadySet,
    GameAlreadyStarted,
    GameAlreadyOver,
    NotPlayersTurn,
}
