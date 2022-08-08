use anchor_lang::prelude::*;
use anchor_lang::AccountsClose;

use crate::constants::COLLECTION;
use crate::{cmp_pubkeys, CandyError, CandyMachine, CollectionPDA};

/// Withdraw SOL from candy machine account.
#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(mut, close = authority, has_one = authority)]
    candy_machine: Account<'info, CandyMachine>,
    #[account(mut, address = candy_machine.authority)]
    authority: Signer<'info>,
    // > Only if collection
    // CollectionPDA account
}

pub fn handle_withdraw_funds<'info>(
    ctx: Context<'_, '_, '_, 'info, WithdrawFunds<'info>>,
) -> Result<()> {
    let authority = &ctx.accounts.authority;
    let candy_machine_info = &ctx.accounts.candy_machine.to_account_info();

    if !ctx.remaining_accounts.is_empty() {
        let seeds = [COLLECTION.as_bytes(), candy_machine_info.key.as_ref()];
        let collection_pda = &ctx.remaining_accounts[0];
        if !cmp_pubkeys(
            &collection_pda.key(),
            &Pubkey::find_program_address(&seeds, &crate::id()).0,
        ) {
            return err!(CandyError::MismatchedCollectionPDA);
        }
        let collection_pda: Account<CollectionPDA> =
            Account::try_from(&collection_pda.to_account_info())?;
        collection_pda.close(authority.to_account_info())?;
    }

    Ok(())
}
