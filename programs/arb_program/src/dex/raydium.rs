use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token};
use solana_program::instruction::Instruction;
use solana_program::program::invoke;
use crate::Swap;

pub fn handle_swap(ctx: Context<Swap>, amount_in: u64) -> Result<()> {
    let swap_instruction = Instruction {
        program_id: ctx.accounts.amm_id.key(),
        accounts: vec![
            ctx.accounts.amm_id.to_account_info(),
            ctx.accounts.amm_authority.to_account_info(),
            ctx.accounts.pool_coin_token_account.to_account_info(),
            ctx.accounts.pool_pc_token_account.to_account_info(),
            ctx.accounts.user_source_token_account.to_account_info(),
            ctx.accounts.user_destination_token_account.to_account_info(),
            ctx.accounts.user_wallet.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ],
        data: build_swap_data(amount_in),
    };

    invoke(
        &swap_instruction,
        &[
            ctx.accounts.amm_id.to_account_info(),
            ctx.accounts.amm_authority.to_account_info(),
            ctx.accounts.pool_coin_token_account.to_account_info(),
            ctx.accounts.pool_pc_token_account.to_account_info(),
            ctx.accounts.user_source_token_account.to_account_info(),
            ctx.accounts.user_destination_token_account.to_account_info(),
            ctx.accounts.user_wallet.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ],
    )?;

    Ok(())
}

fn build_swap_data(amount_in: u64) -> Vec<u8> {
    let mut data = vec![0u8; 8];
    data[..8].copy_from_slice(&amount_in.to_le_bytes());
    data
}
