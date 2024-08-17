use anchor_lang::prelude::*;

use num_derive::*;
use num_traits::*;

declare_id!("2VoQBkLfKDU3S7iqfo1Ub7NqBinC3fFBL8CDLqcvXz2J");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct Game {
    players: [Pubkey; 2],           // (32 * 2)
    turn: u8,                       // 1
    board: [[Option<Sign>; 3]; 33], // 9 * (1 + 1) = 18
    state: GameState,               // 32 + 1
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Pubkey },
}

#[derive(
    AnchorSerialize, AnchorDeserialize, FromPrimitive, ToPrimitive, Copy, Clone, PartialEq, Eq,
)]
pub enum Sign {
    X,
    O,
}
