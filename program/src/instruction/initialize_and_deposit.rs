use crate::{
    constants::{LAMPORTS_PER_SOL, MAXIMUM_AGE, SOL_ID},
    state::VaultState,
    utils::{get_feed_id_from_hex, load_acc_mut_unchecked, load_ix_data, DataLen},
};
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    sysvars::{clock::Clock, rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::state::TokenAccount;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InitializeIxData {
    pub amount: u64,
    pub lock_duration: i64,
    pub bump: u8,
}

impl DataLen for InitializeIxData {
    const LEN: usize = core::mem::size_of::<InitializeIxData>();
}

pub fn process_initialize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [user, expiration, amount, vault_state, vault, _system_program, _token_program, _rest @ ..] =
        accounts
    else {
        return Err(ProgramError::InvalidAccountData);
    };
    if !user.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if !vault_state.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    let vault_acc = TokenAccount::from_account_info(vault)?;
    assert_eq!(vault_acc.owner(), vault_state.key());

    let min_lock_duration: i64 = 2_592_000;

    // Parse feed ID from hex
    let feed_id: [u8; 32] = get_feed_id_from_hex(SOL_ID)?;

    // Get the price update
    let price_data =
        self.price_update
            .get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE, &feed_id)?;

    // Access the price
    let price: i64 = price_data.price; // price is i64

    // Ensure price is non-negative before converting to u64
    let sol_price_at_initialization: u64 = price
        .try_into()
        .map_err(|_| MyProgramError::InvalidPriceConversion)?; // Handle possible conversion errors

    let rent = Rent::get()?;
    let ix_data = unsafe { load_ix_data::<InitializeIxData>(data)? };

    // Check if the lock duration is sufficient
    assert!(
        lock_duration >= min_lock_duration,
        MyProgramError::TimeTooShort
    );

    let bump_seed = [ix_data.bump];
    let vault_state_seeds = [
        Seed::from(b"vault_state"),
        Seed::from(maker.key().as_ref()),
        Seed::from(&bump_seed[..]),
    ];

    let seeds = Signer::from(&vault_state_seeds);

    (CreateAccount {
        from: user,
        to: vault_state,
        lamports: rent.minimum_balance(VaultState::LEN),
        space: VaultState::LEN as u64,
        owner: &crate::ID,
    })
    .invoke_signed(&[seeds])?;

    let vault_acc_state =
        (unsafe { load_acc_mut_unchecked::<VaultState>(vault_state.borrow_mut_data_unchecked()) })?;

    vault_acc_state.initialize(
        *user.key(),
        ix_data.lock_duration,
        ix_data.bump,
        Clock::get()?.unix_timestamp,
        *sol_price_at_initialization,
    );

    Ok(())
}

pub fn process_deposit(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [user, user_ata, amount, vault_state, vault, _system_program, _token_program, _rest @ ..] =
        accounts
    else {
        return Err(ProgramError::InvalidAccountData);
    };

    let min_deposit: u64 = LAMPORTS_PER_SOL; // Minimum deposit of 1 SOL

    let ix_data = unsafe { load_ix_data::<InitializeIxData>(data)? };

    let vault_acc = VaultState::from_account_info(vault_state);

    // Ensure the deposit amount is at least 1 SOL
    assert!(amount >= min_deposit, ProgramError::DepositTooSmall);

    assert_eq!(vault_acc.user, *user.key());

    assert!(
        TokenAccount::from_account_info_unchecked(vault)
            .unwrap()
            .owner()
            == vault_state.key()
    );

    pinocchio_token::instructions::Transfer {
        from: user_ata,
        to: vault,
        authority: user,
        amount: ix_data.amount,
    }
    .invoke()?;
    // update state
    vault_acc.amount = ix_data.amount;
    Ok(())
}
