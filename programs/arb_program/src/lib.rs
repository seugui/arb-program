#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token};

mod dex; // Registra el folder dex como un módulo

declare_id!("3XaaffPKycGUe2yc5EC2yGwyFugDUWH6FY9X97YuG5Xc");

#[program]
pub mod arb_program {
    use super::*;

    pub fn swap(ctx: Context<Swap>, amount_in: u64) -> Result<()> {
        dex::raydium::handle_swap(ctx, amount_in)
    }
}

#[derive(Accounts)]
pub struct Swap<'info> {
    /// CHECK: Esta es la cuenta del AMM en Raydium, no necesita validación adicional.
    pub amm_id: AccountInfo<'info>,
    /// CHECK: La autoridad del AMM no necesita validación adicional porque es parte del contrato Raydium.
    pub amm_authority: AccountInfo<'info>,
    #[account(mut)]
    pub pool_coin_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_pc_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_source_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_destination_token_account: Account<'info, TokenAccount>,
    /// CHECK: Esta cuenta pertenece al usuario final y no requiere validación adicional.
    #[account(signer)]
    pub user_wallet: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}