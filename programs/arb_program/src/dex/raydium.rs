use anchor_lang::prelude::*;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::program::invoke;

pub fn handle_swap(ctx: Context<crate::Swap>, amount_in: u64) -> Result<()> {
    let accounts = vec![
        AccountMeta::new_readonly(*ctx.accounts.amm_id.key, false),
        AccountMeta::new_readonly(*ctx.accounts.amm_authority.key, false),
        AccountMeta::new(*ctx.accounts.pool_coin_token_account.to_account_info().key, false),
        AccountMeta::new(*ctx.accounts.pool_pc_token_account.to_account_info().key, false),
        AccountMeta::new(*ctx.accounts.user_source_token_account.to_account_info().key, false),
        AccountMeta::new(*ctx.accounts.user_destination_token_account.to_account_info().key, false),
        AccountMeta::new_readonly(*ctx.accounts.user_wallet.key, true),
        AccountMeta::new_readonly(*ctx.accounts.token_program.key, false),
    ];

    let swap_instruction = Instruction {
        program_id: ctx.accounts.amm_id.key(),
        accounts,
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
