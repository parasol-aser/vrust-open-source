use crate::engine::common::subscribe_checks;
use crate::engine::constants::DEFAULT_DATA;
use crate::error::PaymentProcessorError;
use crate::state::{Discriminator, Serdes, SubscriptionAccount, SubscriptionStatus};
use crate::utils::get_subscription_account_size;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{clock::Clock, rent::Rent, Sysvar},
};

pub fn process_subscribe(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    maybe_data: Option<String>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let signer_info = next_account_info(account_info_iter)?;
    let subscription_info = next_account_info(account_info_iter)?;
    let merchant_info = next_account_info(account_info_iter)?;
    let order_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let rent_sysvar_info = next_account_info(account_info_iter)?;

    let (order_account, package) = subscribe_checks(
        program_id,
        signer_info,
        merchant_info,
        order_info,
        subscription_info,
        &name,
    )?;

    // ensure the amount paid is as expected
    if package.price > order_account.paid_amount {
        return Err(PaymentProcessorError::NotFullyPaid.into());
    }
    // get subscription account size
    let data = match maybe_data {
        None => String::from(DEFAULT_DATA),
        Some(value) => value,
    };
    let account_size = get_subscription_account_size(&name, &data);
    // the address of the subscription account is derived using the program id,
    // the signer address, the merchant address, and the subscription package name
    // thus ensuring a unique address for each signer + merchant + name
    let (_subscribe_account_address, bump_seed) = Pubkey::find_program_address(
        &[
            &signer_info.key.to_bytes(),
            &merchant_info.key.to_bytes(),
            &name.as_bytes(),
        ],
        program_id,
    );
    // get signer seeds
    let signer_seeds: &[&[_]] = &[
        &signer_info.key.to_bytes(),
        &merchant_info.key.to_bytes(),
        &name.as_bytes(),
        &[bump_seed],
    ];

    // Fund the subscription account with the minimum balance to be rent exempt
    invoke(
        &system_instruction::transfer(
            &signer_info.key,
            subscription_info.key,
            Rent::default().minimum_balance(account_size),
        ),
        &[
            signer_info.clone(),
            subscription_info.clone(),
            system_program_info.clone(),
        ],
    )?;
    // Allocate space for the subscription account
    invoke_signed(
        &system_instruction::allocate(subscription_info.key, account_size as u64),
        &[subscription_info.clone(), system_program_info.clone()],
        &[&signer_seeds],
    )?;
    // Assign the subscription account to the SolPayments program
    invoke_signed(
        &system_instruction::assign(subscription_info.key, &program_id),
        &[subscription_info.clone(), system_program_info.clone()],
        &[&signer_seeds],
    )?;

    let rent = &Rent::from_account_info(rent_sysvar_info)?;
    let timestamp = Clock::get()?.unix_timestamp;

    // get the trial period duration
    let trial_duration: i64 = match package.trial {
        None => 0,
        Some(value) => value,
    };
    // get the subscription account
    // TODO: ensure this account is not already initialized
    let mut subscription_data = subscription_info.try_borrow_mut_data()?;
    // Saving subscription information...
    let subscription = SubscriptionAccount {
        discriminator: Discriminator::Subscription as u8,
        status: SubscriptionStatus::Initialized as u8,
        owner: signer_info.key.to_bytes(),
        merchant: merchant_info.key.to_bytes(),
        name,
        joined: timestamp,
        period_start: timestamp,
        period_end: timestamp + trial_duration + package.duration,
        data,
    };
    subscription.pack(&mut subscription_data);

    // ensure subscription account is rent exempt
    if !rent.is_exempt(subscription_info.lamports(), account_size) {
        return Err(ProgramError::AccountNotRentExempt);
    }

    Ok(())
}
