use crate::state::game::*;
use anchor_lang::prelude::*;

// Function to initialize a new tic-tac-toe game.
// This function sets up a game account with two players.
pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> Result<()> {
    ctx.accounts
        .game
        .start([ctx.accounts.player_one.key(), player_two])
}

// Context for setting up the game.
// Contains the accounts necessary to initialize a new game.
#[derive(Accounts)]
pub struct SetupGame<'info> {
    #[account(
        init, // Initializes a new game account.
        payer = player_one, // Player one pays for the account initialization.
        space = 8 + Game::MAXIMUM_SIZE) // Allocates enough space on the blockchain for the game account.
        ]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player_one: Signer<'info>, // The signer who is setting up the game.
    pub system_program: Program<'info, System>, // System program for interacting with Solana.
}
