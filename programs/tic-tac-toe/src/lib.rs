use anchor_lang::prelude::*;

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
