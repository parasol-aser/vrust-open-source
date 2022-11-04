use crate::{
    engine::json::{OrderSubscription, Package, Packages},
    error::PaymentProcessorError,
    state::{Discriminator, IsClosed, MerchantAccount, OrderAccount, OrderStatus, Serdes},
};
use serde_json::Error as JSONError;
use solana_program::program_pack::Pack;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    system_instruction,
    sysvar::rent::Rent,
};

/// ensure the order is for the subscription
pub fn verify_subscription_order(
    subscription_info: &AccountInfo<'_>,
    order_account: &OrderAccount,
) -> ProgramResult {
    let order_json_data: Result<OrderSubscription, JSONError> =
        serde_json::from_str(&order_account.data);
    let expected_subscription = match order_json_data {
        Err(_error) => return Err(PaymentProcessorError::InvalidSubscriptionData.into()),
        Ok(data) => data.subscription,
    };
    if expected_subscription != subscription_info.key.to_string() {
        return Err(PaymentProcessorError::WrongOrderAccount.into());
    }
    Ok(())
}

/// Get subscription package
pub fn get_subscription_package(
    subscription_package_name: &str,
    merchant_account: &MerchantAccount,
) -> Result<Package, ProgramError> {
    // ensure the merchant has a subscription by this name
    let merchant_json_data: Result<Packages, JSONError> =
        serde_json::from_str(&merchant_account.data);
    let packages = match merchant_json_data {
        Err(_error) => return Err(PaymentProcessorError::InvalidSubscriptionData.into()),
        Ok(data) => data.packages,
    };
    // NB: if the are duplicates, take the first one --> verified in a test
    let package = packages
        .into_iter()
        .find(|package| package.name == subscription_package_name);
    match package {
        None => return Err(PaymentProcessorError::InvalidSubscriptionPackage.into()),
        Some(value) => Ok(value),
    }
}

/// run checks for subscription processing
pub fn subscribe_checks(
    program_id: &Pubkey,
    signer_info: &AccountInfo<'_>,
    merchant_info: &AccountInfo<'_>,
    order_info: &AccountInfo<'_>,
    subscription_info: &AccountInfo<'_>,
    subscription_name: &str,
) -> Result<(OrderAccount, Package), ProgramError> {
    // ensure signer can sign
    if !signer_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    // ensure merchant & order accounts are owned by this program
    if *merchant_info.owner != *program_id {
        msg!("Error: Wrong owner for merchant account");
        return Err(ProgramError::IncorrectProgramId);
    }
    if *order_info.owner != *program_id {
        msg!("Error: Wrong owner for order account");
        return Err(ProgramError::IncorrectProgramId);
    }
    // get the merchant account
    let merchant_account = MerchantAccount::unpack(&merchant_info.data.borrow())?;
    if merchant_account.is_closed() {
        return Err(PaymentProcessorError::ClosedAccount.into());
    }
    if !merchant_account.is_initialized() {
        return Err(ProgramError::UninitializedAccount);
    }
    let allowed_merchant_account_types = vec![
        Discriminator::MerchantSubscription as u8,
        Discriminator::MerchantSubscriptionWithTrial as u8,
    ];
    if !allowed_merchant_account_types.contains(&merchant_account.discriminator) {
        msg!("Error: Invalid merchant account");
        return Err(ProgramError::InvalidAccountData);
    }
    // get the order account
    let order_account = OrderAccount::unpack(&order_info.data.borrow())?;
    if order_account.is_closed() {
        return Err(PaymentProcessorError::ClosedAccount.into());
    }
    if !order_account.is_initialized() {
        return Err(ProgramError::UninitializedAccount);
    }
    if order_account.discriminator != Discriminator::OrderExpressCheckout as u8 {
        msg!("Error: Invalid order account");
        return Err(ProgramError::InvalidAccountData);
    }
    // ensure this order is for this subscription
    verify_subscription_order(subscription_info, &order_account)?;
    // ensure we have the right payer
    if signer_info.key.to_bytes() != order_account.payer {
        return Err(PaymentProcessorError::WrongPayer.into());
    }
    // ensure order account is paid
    if order_account.status != (OrderStatus::Paid as u8) {
        return Err(PaymentProcessorError::NotPaid.into());
    }
    // ensure the order account belongs to this merchant
    if merchant_info.key.to_bytes() != order_account.merchant {
        return Err(ProgramError::InvalidAccountData);
    }
    // get the package
    let package = get_subscription_package(subscription_name, &merchant_account)?;
    if package.mint != Pubkey::new_from_array(order_account.mint).to_string() {
        return Err(PaymentProcessorError::WrongMint.into());
    }
    Ok((order_account, package))
}

/// Create associated token account
///
/// Creates an associated token account that is owned by a custom program.
/// This is similar to spl_associated_token_account::create_associated_token_account
/// which would fail for creating token accounts not owned by the token program
pub fn create_program_owned_associated_token_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo; 8],
    rent: &Rent,
) -> ProgramResult {
    let signer_info = &accounts[0];
    let base_account_info = &accounts[1];
    let new_account_info = &accounts[2];
    let mint_info = &accounts[3];
    let pda_info = &accounts[4];
    let token_program_info = &accounts[5];
    let system_program_info = &accounts[6];
    let rent_sysvar_info = &accounts[7];

    let (associated_token_address, bump_seed) = Pubkey::find_program_address(
        &[
            &base_account_info.key.to_bytes(),
            &spl_token::id().to_bytes(),
            &mint_info.key.to_bytes(),
        ],
        program_id,
    );
    // assert that the derived address matches the one supplied
    if associated_token_address != *new_account_info.key {
        msg!("Error: Associated address does not match seed derivation");
        return Err(ProgramError::InvalidSeeds);
    }
    // get signer seeds
    let associated_token_account_signer_seeds: &[&[_]] = &[
        &base_account_info.key.to_bytes(),
        &spl_token::id().to_bytes(),
        &mint_info.key.to_bytes(),
        &[bump_seed],
    ];
    // Fund the associated seller token account with the minimum balance to be rent exempt
    let required_lamports = rent
        .minimum_balance(spl_token::state::Account::LEN)
        .max(1)
        .saturating_sub(new_account_info.lamports());
    if required_lamports > 0 {
        // Transfer lamports to the associated seller token account
        invoke(
            &system_instruction::transfer(
                &signer_info.key,
                new_account_info.key,
                required_lamports,
            ),
            &[
                signer_info.clone(),
                new_account_info.clone(),
                system_program_info.clone(),
            ],
        )?;
    }
    // Allocate space for the associated seller token account
    invoke_signed(
        &system_instruction::allocate(new_account_info.key, spl_token::state::Account::LEN as u64),
        &[new_account_info.clone(), system_program_info.clone()],
        &[&associated_token_account_signer_seeds],
    )?;
    // Assign the associated seller token account to the SPL Token program
    invoke_signed(
        &system_instruction::assign(new_account_info.key, &spl_token::id()),
        &[new_account_info.clone(), system_program_info.clone()],
        &[&associated_token_account_signer_seeds],
    )?;
    // Initialize the associated seller token account
    invoke(
        &spl_token::instruction::initialize_account(
            &spl_token::id(),
            new_account_info.key,
            mint_info.key,
            pda_info.key,
        )?,
        &[
            new_account_info.clone(),
            mint_info.clone(),
            pda_info.clone(),
            rent_sysvar_info.clone(),
            token_program_info.clone(),
        ],
    )?;

    Ok(())
}

/// Transfer SOL from one account to another
/// Used for accounts not owned by the system program
pub fn transfer_sol(
    sol_origin_info: AccountInfo,
    sol_destination_info: AccountInfo,
    amount: u64,
) -> ProgramResult {
    // Transfer tokens from the account to the sol_destination.
    let dest_starting_lamports = sol_destination_info.lamports();
    let origin_starting_lamports = sol_origin_info.lamports();

    **sol_destination_info.lamports.borrow_mut() =
        dest_starting_lamports.checked_add(amount).unwrap();
    **sol_origin_info.lamports.borrow_mut() = origin_starting_lamports.checked_sub(amount).unwrap();
    Ok(())
}
