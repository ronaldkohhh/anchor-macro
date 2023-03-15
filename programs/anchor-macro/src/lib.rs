use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod pda_example {
    use super::*;

    pub fn initialize_pda_account(ctx: Context<InitializePdaAccount>) -> ProgramResult {
        // Initialization logic goes here
    }
}

#[derive(Accounts)]
pub struct InitializePdaAccount<'info> {
    #[account(signer)]
    pub authority: Signer<'info>,
    #[account(init, associated_token::address)]
    pub pda_account: Box<Account<'info, TokenAccount>>,
    pub rent: Sysvar<'info, Rent>,
}
