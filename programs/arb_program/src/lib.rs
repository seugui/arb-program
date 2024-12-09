use anchor_lang::prelude::*;
use crate::dex::raydium;

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

#[error_code]
pub enum ErrorCode {
    #[msg("El DEX especificado no está soportado.")]
    DexNotSupported,
