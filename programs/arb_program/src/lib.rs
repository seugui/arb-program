use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token};
use crate::dex::raydium;

declare_id!("8QZSmYMQZUscLBXWczUBBkzLKWzLQZ2S5f8Tg1L1N6bH");

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
