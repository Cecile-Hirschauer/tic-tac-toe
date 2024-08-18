use crate::errors::TicTacToeError;
use crate::state::game::*;
use anchor_lang::prelude::*;

// Function to make a move in the game.
// The current player must be the one who is allowed to play, and a move is made on the specified tile.
pub fn play(ctx: Context<Play>, tile: Tile) -> Result<()> {
    let game = &mut ctx.accounts.game;

    // Ensure that the current player is the one allowed to play.
    require_keys_eq!(
        game.current_player(),
        ctx.accounts.player.key(),
        TicTacToeError::NotPlayersTurn,
    );

    // Execute the move on the specified tile.
    game.play(&tile)
}

// Context for making a move.
// Contains the accounts necessary to make a move in the game.
#[derive(Accounts)]
pub struct Play<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>, // Game account where the move will be made.
    pub player: Signer<'info>, // The signer who is making the move.
}
