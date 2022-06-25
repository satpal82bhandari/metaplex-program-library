use anchor_lang::{prelude::*, Discriminator};
use mpl_token_metadata::state::{MAX_CREATOR_LIMIT, MAX_SYMBOL_LENGTH};
use solana_program::{program::invoke, system_instruction};
use spl_token::state::Mint;

use crate::{
    assert_initialized, assert_owned_by, cmp_pubkeys,
    constants::{CONFIG_ARRAY_START, CONFIG_LINE_SIZE, DEFAULT_UUID, SWAP_REMOVE_FEATURE_INDEX},
    set_feature_flag, CandyError, CandyMachine, CandyMachineData,
};

/// Create a new candy machine.
#[derive(Accounts)]
#[instruction(data: CandyMachineData)]
pub struct InitializeCandyMachine<'info> {
    /// CHECK: account constraints checked in account trait
    #[account(zero, rent_exempt = skip, constraint = candy_machine.to_account_info().owner == program_id && candy_machine.to_account_info().data_len() >= get_space_for_candy(data)?)]
    candy_machine: UncheckedAccount<'info>,
    /// CHECK: wallet can be any account and is not written to or read
    wallet: UncheckedAccount<'info>,
    /// CHECK: authority can be any account and is not written to or read
    authority: UncheckedAccount<'info>,
    payer: Signer<'info>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

pub fn handle_initialize_candy_machine(
    ctx: Context<InitializeCandyMachine>,
    data: CandyMachineData,
) -> Result<()> {
    let candy_machine_account = &mut ctx.accounts.candy_machine;

    let mut candy_machine = CandyMachine {
        data,
        authority: ctx.accounts.authority.key(),
        wallet: ctx.accounts.wallet.key(),
        token_mint: None,
        items_redeemed: 0,
    };
    // default UUID for features flag
    candy_machine.data.uuid = String::from(DEFAULT_UUID);

    if !ctx.remaining_accounts.is_empty() {
        let token_mint_info = &ctx.remaining_accounts[0];
        let _token_mint: Mint = assert_initialized(token_mint_info)?;
        let token_account: spl_token::state::Account = assert_initialized(&ctx.accounts.wallet)?;

        assert_owned_by(token_mint_info, &spl_token::id())?;
        assert_owned_by(&ctx.accounts.wallet, &spl_token::id())?;

        if !cmp_pubkeys(&token_account.mint, &token_mint_info.key()) {
            return err!(CandyError::MintMismatch);
        }

        candy_machine.token_mint = Some(*token_mint_info.key);
    }

    let mut array_of_zeroes = vec![];
    while array_of_zeroes.len() < MAX_SYMBOL_LENGTH - candy_machine.data.symbol.len() {
        array_of_zeroes.push(0u8);
    }
    let new_symbol =
        candy_machine.data.symbol.clone() + std::str::from_utf8(&array_of_zeroes).unwrap();
    candy_machine.data.symbol = new_symbol;

    // - 1 because we are going to be a creator
    if candy_machine.data.creators.len() > MAX_CREATOR_LIMIT - 1 {
        return err!(CandyError::TooManyCreators);
    }

    // previous CLIs do not allocate the size needed to store the mint
    // index array for the swap_remove - during initialization we fund the
    // account to have rent for the increased size, which will be done
    // during the add_config_lines
    if candy_machine.data.hidden_settings.is_none() {
        let expected_size = CONFIG_ARRAY_START
            + 4
            + (candy_machine.data.items_available as usize) * CONFIG_LINE_SIZE
            + 4
            + ((candy_machine
                .data
                .items_available
                .checked_div(8)
                .ok_or(CandyError::NumericalOverflowError)?
                + 1) as usize)
            + 4
            + (candy_machine.data.items_available as usize) * 4;

        if (expected_size as u64) > system_instruction::MAX_PERMITTED_DATA_LENGTH {
            return err!(CandyError::CandyMachineExceedDataLimit);
        }

        let account = candy_machine_account.to_account_info();

        if account.data_len() < expected_size {
            let snapshot: u64 = account.lamports();
            // calculates the additional lamports needed
            let rent_delta = Rent::get()?
                .minimum_balance(expected_size)
                .saturating_sub(snapshot);

            msg!("Adding {} lamports for account realloc", rent_delta);

            invoke(
                &system_instruction::transfer(ctx.accounts.payer.key, account.key, rent_delta),
                &[
                    ctx.accounts.payer.to_account_info(),
                    account.clone(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
        }

        // new candy machines will be using swap_remove
        set_feature_flag(&mut candy_machine.data.uuid, SWAP_REMOVE_FEATURE_INDEX);
    }

    let mut new_data = CandyMachine::discriminator().try_to_vec().unwrap();
    new_data.append(&mut candy_machine.try_to_vec().unwrap());

    let mut data = candy_machine_account.data.borrow_mut();
    // god forgive me couldnt think of better way to deal with this
    for i in 0..new_data.len() {
        data[i] = new_data[i];
    }

    // only if we are not using hidden settings we will have space for
    // the config lines
    if candy_machine.data.hidden_settings.is_none() {
        let vec_start = CONFIG_ARRAY_START
            + 4
            + (candy_machine.data.items_available as usize) * CONFIG_LINE_SIZE;
        let as_bytes = (candy_machine
            .data
            .items_available
            .checked_div(8)
            .ok_or(CandyError::NumericalOverflowError)? as u32)
            .to_le_bytes();

        for i in 0..4 {
            data[vec_start + i] = as_bytes[i];
        }
    }

    Ok(())
}

fn get_space_for_candy(data: CandyMachineData) -> Result<usize> {
    let num = if data.hidden_settings.is_some() {
        CONFIG_ARRAY_START
    } else {
        // not enforcing the allocation of the mint index array to maintain
        // compatibility with previous CLIs (the size will be increased if needed
        // when adding config lines)
        CONFIG_ARRAY_START
            + 4
            + (data.items_available as usize) * CONFIG_LINE_SIZE
            + 8
            + 2 * ((data
                .items_available
                .checked_div(8)
                .ok_or(CandyError::NumericalOverflowError)?
                + 1) as usize)
    };

    Ok(num)
}
