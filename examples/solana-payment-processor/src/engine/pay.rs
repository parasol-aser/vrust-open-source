use crate::{
    engine::{
        common::create_program_owned_associated_token_account,
        constants::{DEFAULT_DATA, INITIAL, PAID, PROGRAM_OWNER, SPONSOR_FEE},
        json::{Item, OrderItems},
    },
    error::PaymentProcessorError,
    state::{Discriminator, IsClosed, MerchantAccount, OrderAccount, OrderStatus, Serdes},
    utils::{get_amounts, get_order_account_size},
};
use serde_json::{json, Error as JSONError, Value};
use solana_program::program_pack::Pack;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{clock::Clock, rent::Rent, Sysvar},
};
use spl_token::{self, state::Account as TokenAccount};
use std::collections::BTreeMap;
use std::str::FromStr;

/// Run checks for order processing
pub fn order_checks(
    program_id: &Pubkey,
    signer_info: &AccountInfo<'_>,
    merchant_info: &AccountInfo<'_>,
    buyer_token_info: &AccountInfo<'_>,
    mint_info: &AccountInfo<'_>,
    program_owner_info: &AccountInfo<'_>,
    sponsor_info: &AccountInfo<'_>,
) -> Result<MerchantAccount, ProgramError> {
    // ensure signer can sign
    if !signer_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    // ensure merchant account is owned by this program
    if *merchant_info.owner != *program_id {
        msg!("Error: Wrong owner for merchant account");
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
    // ensure buyer token account is owned by token program
    if *buyer_token_info.owner != spl_token::id() {
        msg!("Error: Buyer token account not owned by Token Program");
        return Err(ProgramError::IncorrectProgramId);
    }
    // Get mint details and verify that they match token account
    let buyer_token_data = TokenAccount::unpack(&buyer_token_info.data.borrow())?;
    if *mint_info.key != buyer_token_data.mint {
        return Err(PaymentProcessorError::MintNotEqual.into());
    }
    // check that provided program owner is correct
    if *program_owner_info.key != Pubkey::from_str(PROGRAM_OWNER).unwrap() {
        return Err(PaymentProcessorError::WrongProgramOwner.into());
    }
    // check that the provided sponsor is correct
    if *sponsor_info.key != Pubkey::new_from_array(merchant_account.sponsor) {
        msg!("Error: Sponsor account is incorrect");
        return Err(PaymentProcessorError::WrongSponsor.into());
    }

    Ok(merchant_account)
}

/// Verify chain checkout
///
/// Mainly ensure that the item(s) being paid for match the item(s) in the
/// merchant account and that the amount being paid is sufficient.
///
/// order_items is an object that looks like so:
/// {
///     id: quantity
/// }
/// e.g. {"item1", 1, "item2": 33}
pub fn chain_checkout_checks(
    merchant_account: &MerchantAccount,
    mint: &AccountInfo,
    order_items: &OrderItems,
    amount: u64,
) -> ProgramResult {
    if merchant_account.discriminator != Discriminator::MerchantChainCheckout as u8 {
        msg!("Error: Invalid merchant account");
        return Err(PaymentProcessorError::InvalidMerchantData.into());
    }

    let merchant_json_data: Result<BTreeMap<String, Item>, JSONError> =
        serde_json::from_str(&merchant_account.data);

    let registered_items = match merchant_json_data {
        Err(_error) => return Err(PaymentProcessorError::InvalidMerchantData.into()),
        Ok(data) => data,
    };

    let mut total_amount: u64 = 0;

    for (key, quantity) in order_items.iter() {
        let registered_item = match registered_items.get(key) {
            None => {
                msg!("Error: Invalid order item {:?}", key);
                return Err(PaymentProcessorError::InvalidOrderData.into());
            }
            Some(value) => value,
        };
        if registered_item.mint != mint.key.to_string() {
            msg!(
                "Error: Mint {:?} invalid for this order",
                mint.key.to_string()
            );
            return Err(PaymentProcessorError::WrongMint.into());
        }

        total_amount = total_amount + (registered_item.price * quantity);
    }

    if total_amount > amount {
        msg!("Error: Insufficient amount, should be {:?}", total_amount);
        return Err(ProgramError::InsufficientFunds);
    }

    Ok(())
}

/// process an order payment
pub fn process_order(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
    order_id: String,
    secret: String,
    maybe_data: Option<String>,
    checkout_items: Option<OrderItems>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let signer_info = next_account_info(account_info_iter)?;
    let order_info = next_account_info(account_info_iter)?;
    let merchant_info = next_account_info(account_info_iter)?;
    let seller_token_info = next_account_info(account_info_iter)?;
    let buyer_token_info = next_account_info(account_info_iter)?;
    let program_owner_info = next_account_info(account_info_iter)?;
    let sponsor_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let pda_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let rent_sysvar_info = next_account_info(account_info_iter)?;

    let rent = &Rent::from_account_info(rent_sysvar_info)?;
    let timestamp = Clock::get()?.unix_timestamp;

    let merchant_account = order_checks(
        program_id,
        signer_info,
        merchant_info,
        buyer_token_info,
        mint_info,
        program_owner_info,
        sponsor_info,
    )?;

    // get data
    let mut data = match maybe_data {
        None => String::from(DEFAULT_DATA),
        Some(value) => value,
    };

    let mut order_account_type = Discriminator::OrderExpressCheckout as u8;

    // process chain checkout
    if checkout_items.is_some() {
        order_account_type = Discriminator::OrderChainCheckout as u8;
        let order_items = checkout_items.unwrap();
        chain_checkout_checks(&merchant_account, &mint_info.clone(), &order_items, amount)?;
        if data == String::from(DEFAULT_DATA) {
            data = json!({ PAID: order_items }).to_string();
        } else {
            // let possible_json_data: Result<BTreeMap<&str, Value>, JSONError> = serde_json::from_str(&data);
            // let json_data = match possible_json_data {
            let json_data: Value = match serde_json::from_str(&data) {
                Err(_error) => return Err(PaymentProcessorError::InvalidOrderData.into()),
                Ok(data) => data,
            };
            data = json!({
                INITIAL: json_data,
                PAID: order_items
            })
            .to_string();
        }
    }

    // create order account
    let order_account_size = get_order_account_size(&order_id, &secret, &data);
    // the order account amount includes the fee in SOL
    let order_account_amount = Rent::default().minimum_balance(order_account_size);
    invoke(
        &system_instruction::create_account(
            signer_info.key,
            order_info.key,
            order_account_amount,
            order_account_size as u64,
            program_id,
        ),
        &[
            signer_info.clone(),
            order_info.clone(),
            system_program_info.clone(),
        ],
    )?;

    // next we are going to try and create a token account owned by the program
    // but whose address is derived from the order account
    // TODO: for subscriptions, should this use the subscription account as the base?
    create_program_owned_associated_token_account(
        program_id,
        &[
            signer_info.clone(),
            order_info.clone(),
            seller_token_info.clone(),
            mint_info.clone(),
            pda_info.clone(),
            token_program_info.clone(),
            system_program_info.clone(),
            rent_sysvar_info.clone(),
        ],
        rent,
    )?;

    // Transfer payment amount to associated seller token account...
    invoke(
        &spl_token::instruction::transfer(
            token_program_info.key,
            buyer_token_info.key,
            seller_token_info.key,
            signer_info.key,
            &[&signer_info.key],
            amount,
        )
        .unwrap(),
        &[
            buyer_token_info.clone(),
            seller_token_info.clone(),
            signer_info.clone(),
            token_program_info.clone(),
        ],
    )?;

    if Pubkey::new_from_array(merchant_account.sponsor) == Pubkey::from_str(PROGRAM_OWNER).unwrap()
    {
        // Transferring processing fee to the program owner...
        invoke(
            &system_instruction::transfer(
                &signer_info.key,
                program_owner_info.key,
                merchant_account.fee,
            ),
            &[
                signer_info.clone(),
                program_owner_info.clone(),
                system_program_info.clone(),
            ],
        )?;
    } else {
        // we need to pay both the program owner and the sponsor
        let (program_owner_fee, sponsor_fee) = get_amounts(merchant_account.fee, SPONSOR_FEE);
        // Transferring processing fee to the program owner and sponsor...
        invoke(
            &system_instruction::transfer(
                &signer_info.key,
                program_owner_info.key,
                program_owner_fee,
            ),
            &[
                signer_info.clone(),
                program_owner_info.clone(),
                system_program_info.clone(),
            ],
        )?;
        invoke(
            &system_instruction::transfer(&signer_info.key, sponsor_info.key, sponsor_fee),
            &[
                signer_info.clone(),
                sponsor_info.clone(),
                system_program_info.clone(),
            ],
        )?;
    }

    // get the order account
    // TODO: ensure this account is not already initialized
    let mut order_account_data = order_info.try_borrow_mut_data()?;
    // Saving order information...
    let order = OrderAccount {
        discriminator: order_account_type,
        status: OrderStatus::Paid as u8,
        created: timestamp,
        modified: timestamp,
        merchant: merchant_info.key.to_bytes(),
        mint: mint_info.key.to_bytes(),
        token: seller_token_info.key.to_bytes(),
        payer: signer_info.key.to_bytes(),
        expected_amount: amount,
        paid_amount: amount,
        order_id,
        secret,
        data,
    };

    order.pack(&mut order_account_data);

    // ensure order account is rent exempt
    if !rent.is_exempt(order_info.lamports(), order_account_size) {
        return Err(ProgramError::AccountNotRentExempt);
    }

    Ok(())
}

pub fn process_express_checkout(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
    order_id: String,
    secret: String,
    maybe_data: Option<String>,
) -> ProgramResult {
    process_order(
        program_id,
        accounts,
        amount,
        order_id,
        secret,
        maybe_data,
        Option::None,
    )?;
    Ok(())
}

pub fn process_chain_checkout(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
    order_items: OrderItems,
    maybe_data: Option<String>,
) -> ProgramResult {
    process_order(
        program_id,
        accounts,
        amount,
        format!("{timestamp}", timestamp = Clock::get()?.unix_timestamp),
        "".to_string(),
        maybe_data,
        Some(order_items),
    )?;
    Ok(())
}
