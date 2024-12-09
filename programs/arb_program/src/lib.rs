use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token};
use crate::dex::raydium;

declare_id!("Rw9S416EA9rc6CA8UGazHSg4XZW3FvyoUE58akafcT9");

#[program]
pub mod arb_program {
    use super::*;

    pub fn swap(ctx: Context<Swap>, amount_in: u64) -> Result<()> {
        raydium::handle_swap(ctx, amount_in)
    }
}

#[derive(Accounts)]
pub struct Swap<'info> {
    /// CHECK: Validación en tiempo de ejecución
    pub amm_id: AccountInfo<'info>,
    /// CHECK: Validación en tiempo de ejecución
    pub amm_authority: AccountInfo<'info>,
    #[account(mut)]
    pub pool_coin_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_pc_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_source_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_destination_token_account: Account<'info, TokenAccount>,
    #[account(signer)]
    pub user_wallet: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
