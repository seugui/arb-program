#![allow(unused_imports)]
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::dex::raydium; // Importa el submódulo Raydium

// Declaración del ID del programa
declare_id!("Fq217QUgpVEeaAnptT4rrpLc4oZ51hgbWTTxHW1DRyEJ");

#[program]
pub mod dex_swap_example {
    use super::*;

    pub fn swap(ctx: Context<Swap>, dex_name: String, amount_in: u64) -> Result<()> {
        match dex_name.as_str() {
            "raydium" => raydium::handle_swap(ctx, amount_in),
            _ => Err(ErrorCode::DexNotSupported.into()),
        }
    }
}

#[derive(Accounts)]
pub struct Swap<'info> {
    /// CHECK: Se valida en tiempo de ejecución
    pub amm_id: AccountInfo<'info>,
    /// CHECK: Se valida en tiempo de ejecución
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

#[error_code]
pub enum ErrorCode {
    #[msg("The specified DEX is not supported.")]
    DexNotSupported,
}
