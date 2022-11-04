use crate::engine::common::subscribe_checks;
use crate::error::PaymentProcessorError;
use crate::state::{Discriminator, IsClosed, Serdes, SubscriptionAccount, SubscriptionStatus};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
};

pub fn process_renew_subscription(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    quantity: i64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let signer_info = next_account_info(account_info_iter)?;
    let subscription_info = next_account_info(account_info_iter)?;
    let merchant_info = next_account_info(account_info_iter)?;
    let order_info = next_account_info(account_info_iter)?;

    // ensure subscription account is owned by this program
    if *subscription_info.owner != *program_id {
        msg!("Error: Wrong owner for subscription account");
        return Err(ProgramError::IncorrectProgramId);
    }
    // get the subscription account
    let mut subscription_account = SubscriptionAccount::unpack(&subscription_info.data.borrow())?;
    if !subscription_account.is_initialized() {
        return Err(ProgramError::UninitializedAccount);
    }
    if subscription_account.is_closed() {
        return Err(PaymentProcessorError::ClosedAccount.into());
    }
    if subscription_account.discriminator != Discriminator::Subscription as u8 {
        msg!("Error: Invalid subscription account");
        return Err(ProgramError::InvalidAccountData);
    }
    let (order_account, package) = subscribe_checks(
        program_id,
        signer_info,
        merchant_info,
        order_info,
        subscription_info,
        &subscription_account.name,
    )?;
    // ensure the amount paid is as expected
    let expected_amount = (quantity as u64) * package.price;
    if expected_amount > order_account.paid_amount {
        return Err(PaymentProcessorError::NotFullyPaid.into());
    }
    // update subscription account
    let timestamp = Clock::get()?.unix_timestamp;
    if timestamp > subscription_account.period_end {
        // had ended so we start a new period
        subscription_account.period_start = timestamp;
        subscription_account.period_end = timestamp + (package.duration * quantity);
    } else {
        // not yet ended so we add the time to the end of the current period
        subscription_account.period_end =
            subscription_account.period_end + (package.duration * quantity);
    }
    subscription_account.status = SubscriptionStatus::Initialized as u8;
    SubscriptionAccount::pack(
        &subscription_account,
        &mut subscription_info.data.borrow_mut(),
    );

    Ok(())
}
