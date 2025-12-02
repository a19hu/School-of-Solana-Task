//-------------------------------------------------------------------------------
///
/// TASK: Implement the withdraw functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the vault is not locked
/// - Verify that the vault has enough balance to withdraw
/// - Transfer lamports from vault to vault authority
/// - Emit a withdraw event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::WithdrawEvent;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    // TODO: Add required accounts and constraints
    #[account(mut)]
    pub placeholder: Signer<'info>,
    #[account(mut)]
    pub vault : Account<'info,Vault>,

    pub system_program : Program<'info,System>
}

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // TODO: Implement withdraw functionality

    let vault = &mut ctx.accounts.vault;
    let user = &mut ctx.accounts.placeholder;

    if vault.locked {
        return Err(VaultError::VaultLocked.into());
    }

    if amount > **vault.to_account_info().lamports.borrow() {
        return Err(VaultError::InsufficientBalance.into());
    }

    **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    **user.to_account_info().try_borrow_mut_lamports()? += amount;

    emit!(WithdrawEvent{
        amount,
        vault_authority: user.key(),
        vault : vault.key()
    });

    Ok(())
    // todo!()
}