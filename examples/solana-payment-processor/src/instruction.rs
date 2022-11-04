use crate::engine::json::OrderItems;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};
use spl_token::{self};
use std::collections::BTreeMap;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum PaymentProcessorInstruction {
    /// Register for a merchant account.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the merchant account
    /// 1. `[writable]` The merchant account.  Owned by this program
    /// 2. `[]` System program
    /// 3. `[]` The rent sysvar
    /// 4. `[optional]` The sponsor account
    RegisterMerchant {
        /// the seed used when creating the account
        #[allow(dead_code)] // not dead code..
        seed: Option<String>,
        /// the amount (in SOL lamports) that will be charged as a fee
        #[allow(dead_code)] // not dead code..
        fee: Option<u64>,
        /// arbitrary merchant data (maybe as a JSON string)
        #[allow(dead_code)] // not dead code..
        data: Option<String>,
    },
    /// Express Checkout
    ///
    /// Meant to be used to process payments initialized by systems that reside off-chain
    /// such as traditional e-commerce software.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the transaction
    /// 1. `[writable]` The order account.  Owned by this program
    /// 2. `[]` The merchant account.  Owned by this program
    /// 3. `[writable]` The seller token account - this is where the amount paid will go. Owned by this program
    /// 4. `[writable]` The buyer token account
    /// 5. `[writable]` The program owner account (where we will send program owner fee)
    /// 6. `[writable]` The sponsor account (where we will send sponsor fee)
    /// 7. `[]` The token mint account - represents the 'currency' being used
    /// 8. `[]` This program's derived address
    /// 9. `[]` The token program
    /// 10. `[]` The System program
    /// 11. `[]` The rent sysvar
    ExpressCheckout {
        #[allow(dead_code)] // not dead code..
        amount: u64,
        /// the external order id (as in issued by the merchant)
        #[allow(dead_code)] // not dead code..
        order_id: String,
        // An extra field that can store an encrypted (ot not encrypted) string
        // that the merchant can use to assert if a transaction is authentic
        #[allow(dead_code)] // not dead code..
        secret: String,
        /// arbitrary merchant data (maybe as a JSON string)
        #[allow(dead_code)] // not dead code..
        data: Option<String>,
    },
    /// Chain Checkout
    ///
    /// Meant to process payments that are validated completely on chain.  That is,
    /// all the information required to check that the payment is valid is available
    /// on-chain.
    ///
    /// This is made possible by relying on a merchant account that has certain defined
    /// items for which payment can be made.  See the engine::json::Item struct as well
    /// as the chain checkout tests for more on how this works.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the transaction
    /// 1. `[writable]` The order account.  Owned by this program
    /// 2. `[]` The merchant account.  Owned by this program
    /// 3. `[writable]` The seller token account - this is where the amount paid will go. Owned by this program
    /// 4. `[writable]` The buyer token account
    /// 5. `[writable]` The program owner account (where we will send program owner fee)
    /// 6. `[writable]` The sponsor account (where we will send sponsor fee)
    /// 7. `[]` The token mint account - represents the 'currency' being used
    /// 8. `[]` This program's derived address
    /// 9. `[]` The token program
    /// 10. `[]` The System program
    /// 11. `[]` The rent sysvar
    ChainCheckout {
        #[allow(dead_code)] // not dead code..
        amount: u64,
        /// the external order id (as in issued by the merchant)
        #[allow(dead_code)] // not dead code..
        order_items: BTreeMap<String, u64>, // use this instead of OrderItems for readability in API
        /// arbitrary merchant data (maybe as a JSON string)
        #[allow(dead_code)] // not dead code..
        data: Option<String>,
    },
    /// Withdraw funds for a particular order
    ///
    /// Note that a payment cannot be withdrawn for an order made for a subscription
    /// payment that is still within the subscription trial period.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the transaction
    /// 1. `[writable]` The order account.  Owned by this program
    /// 2. `[]` The merchant account.  Owned by this program
    /// 3. `[writable]` The order token account (where the money was put during payment)
    /// 4. `[writable]` The merchant token account (where we will withdraw to)
    /// 5. `[writable]` This account receives the refunded SOL after closing order token account
    /// 6. `[]` This program's derived address
    /// 7. `[]` The token program
    Withdraw {
        /// should we close the order account?
        /// can be sent as 0 for false; 1 for true from a dApp
        #[allow(dead_code)] // not dead code..
        close_order_account: bool,
    },
    /// Initialize a subscription
    ///
    /// A subscription is possible by relying on a merchant account that was created with
    /// the correct subscription package information.  See engine::json::Packages as well as
    /// subscription tests for more on this.
    ///
    /// A complete subscription transaction includes an ExpressCheckout instruction followed
    /// by a Subscribe instruction.  The actual payment is made in the ExpressCheckout instruction
    /// and subsequently thr subscription is activated in the Subscribe instruction.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the transaction
    /// 1. `[writable]` The subscription account.  Owned by this program
    /// 2. `[]` The merchant account.  Owned by this program
    /// 3. `[]` The order account.  Owned by this program
    /// 4. `[]` The System program
    /// 5. `[]` The rent sysvar
    Subscribe {
        /// the subscription package name
        #[allow(dead_code)] // not dead code..
        name: String,
        /// arbitrary merchant data (maybe as a JSON string)
        #[allow(dead_code)] // not dead code..
        data: Option<String>,
    },
    /// Renew a subscription
    ///
    /// A complete RenewSubscription transaction includes an ExpressCheckout instruction
    /// followed by a RenewSubscription instruction.  The actual payment is made in the
    /// ExpressCheckout instruction and subsequently thr subscription is activated in the
    /// RenewSubscription instruction.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the transaction
    /// 1. `[writable]` The subscription account.  Owned by this program
    /// 2. `[]` The merchant account.  Owned by this program
    /// 3. `[]` The order account.  Owned by this program
    RenewSubscription {
        /// the number of periods to renew e.g. if the subscription period is a year
        /// you can choose to renew for 1 year, 2 years, n years, etc
        #[allow(dead_code)] // not dead code..
        quantity: i64,
    },
    /// Cancel a subscription
    ///
    /// If a CancelSubscription instruction is sent during the trial period of a
    /// subscription, the amount initially paid for the subscription will be refunded in
    /// full.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the transaction
    /// 1. `[writable]` The subscription account.  Owned by this program
    /// 2. `[]` The merchant account.  Owned by this program
    /// 3. `[writable]` The order account.  Owned by this program
    /// 4. `[writable]` The order token account - this is where the amount was paid into. Owned by this program
    /// 5. `[writable]` The refund token account - this is where the refund will go
    /// 6. `[writable]` This account receives the refunded SOL after closing order token account
    /// 7. `[]` This program's derived address
    /// 8. `[]` The token program
    CancelSubscription,
}

/// Creates an 'RegisterMerchant' instruction.
pub fn register_merchant(
    program_id: Pubkey,
    signer: Pubkey,
    merchant: Pubkey,
    seed: Option<String>,
    fee: Option<u64>,
    data: Option<String>,
    sponsor: Option<&Pubkey>,
) -> Instruction {
    let mut account_metas = vec![
        AccountMeta::new(signer, true),
        AccountMeta::new(merchant, false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    if let Some(sponsor) = sponsor {
        account_metas.push(AccountMeta::new_readonly(*sponsor, false));
    }

    Instruction {
        program_id,
        accounts: account_metas,
        data: PaymentProcessorInstruction::RegisterMerchant { seed, fee, data }
            .try_to_vec()
            .unwrap(),
    }
}

/// Creates an 'ExpressCheckout' instruction.
pub fn express_checkout(
    program_id: Pubkey,
    signer: Pubkey,
    order: Pubkey,
    merchant: Pubkey,
    seller_token: Pubkey,
    buyer_token: Pubkey,
    mint: Pubkey,
    program_owner: Pubkey,
    sponsor: Pubkey,
    pda: Pubkey,
    amount: u64,
    order_id: String,
    secret: String,
    data: Option<String>,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(order, true),
            AccountMeta::new_readonly(merchant, false),
            AccountMeta::new(seller_token, false),
            AccountMeta::new(buyer_token, false),
            AccountMeta::new(program_owner, false),
            AccountMeta::new(sponsor, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(pda, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: PaymentProcessorInstruction::ExpressCheckout {
            amount,
            order_id,
            secret,
            data,
        }
        .try_to_vec()
        .unwrap(),
    }
}

/// Creates an 'ChainCheckout' instruction.
pub fn chain_checkout(
    program_id: Pubkey,
    signer: Pubkey,
    order: Pubkey,
    merchant: Pubkey,
    seller_token: Pubkey,
    buyer_token: Pubkey,
    mint: Pubkey,
    program_owner: Pubkey,
    sponsor: Pubkey,
    pda: Pubkey,
    amount: u64,
    order_items: OrderItems,
    data: Option<String>,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(order, true),
            AccountMeta::new_readonly(merchant, false),
            AccountMeta::new(seller_token, false),
            AccountMeta::new(buyer_token, false),
            AccountMeta::new(program_owner, false),
            AccountMeta::new(sponsor, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(pda, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: PaymentProcessorInstruction::ChainCheckout {
            amount,
            order_items,
            data,
        }
        .try_to_vec()
        .unwrap(),
    }
}

/// Creates an 'Withdraw' instruction.
pub fn withdraw(
    program_id: Pubkey,
    signer: Pubkey,
    order: Pubkey,
    merchant: Pubkey,
    order_payment_token: Pubkey,
    merchant_token: Pubkey,
    account_to_receive_sol_refund: Pubkey,
    pda: Pubkey,
    subscription: Option<Pubkey>,
    close_order_account: bool,
) -> Instruction {
    let mut account_metas = vec![
        AccountMeta::new(signer, true),
        AccountMeta::new(order, false),
        AccountMeta::new_readonly(merchant, false),
        AccountMeta::new(order_payment_token, false),
        AccountMeta::new(merchant_token, false),
        AccountMeta::new(account_to_receive_sol_refund, false),
        AccountMeta::new_readonly(pda, false),
        AccountMeta::new_readonly(spl_token::id(), false),
    ];

    if let Some(subscription) = subscription {
        account_metas.push(AccountMeta::new_readonly(subscription, false));
    }

    Instruction {
        program_id,
        accounts: account_metas,
        data: PaymentProcessorInstruction::Withdraw {
            close_order_account,
        }
        .try_to_vec()
        .unwrap(),
    }
}

/// creates a 'Subscribe' instruction
pub fn subscribe(
    program_id: Pubkey,
    signer: Pubkey,
    subscription: Pubkey,
    merchant: Pubkey,
    order: Pubkey,
    name: String,
    data: Option<String>,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(subscription, false),
            AccountMeta::new_readonly(merchant, false),
            AccountMeta::new_readonly(order, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: PaymentProcessorInstruction::Subscribe { name, data }
            .try_to_vec()
            .unwrap(),
    }
}

/// creates a 'RenewSubscription' instruction
pub fn renew_subscription(
    program_id: Pubkey,
    signer: Pubkey,
    subscription: Pubkey,
    merchant: Pubkey,
    order: Pubkey,
    quantity: i64,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(subscription, false),
            AccountMeta::new_readonly(merchant, false),
            AccountMeta::new_readonly(order, false),
        ],
        data: PaymentProcessorInstruction::RenewSubscription { quantity }
            .try_to_vec()
            .unwrap(),
    }
}

/// creates a 'CancelSubscription' instruction
pub fn cancel_subscription(
    program_id: Pubkey,
    signer: Pubkey,
    subscription: Pubkey,
    merchant: Pubkey,
    order: Pubkey,
    order_token: Pubkey,
    refund_token: Pubkey,
    account_to_receive_sol_refund: Pubkey,
    pda: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(subscription, false),
            AccountMeta::new_readonly(merchant, false),
            AccountMeta::new(order, false),
            AccountMeta::new(order_token, false),
            AccountMeta::new(refund_token, false),
            AccountMeta::new(account_to_receive_sol_refund, false),
            AccountMeta::new_readonly(pda, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: PaymentProcessorInstruction::CancelSubscription
            .try_to_vec()
            .unwrap(),
    }
}

#[cfg(test)]
mod test {
    use {
        super::*,
        crate::engine::constants::{
            DEFAULT_FEE_IN_LAMPORTS, INITIAL, MERCHANT, MIN_FEE_IN_LAMPORTS, PAID, PDA_SEED,
            PROGRAM_OWNER, SPONSOR_FEE,
        },
        crate::error::PaymentProcessorError,
        crate::instruction::PaymentProcessorInstruction,
        crate::state::{
            MerchantAccount, OrderAccount, OrderStatus, Serdes, SubscriptionAccount,
            SubscriptionStatus,
        },
        crate::utils::{get_amounts, get_order_account_size},
        assert_matches::*,
        serde_json::{json, Value},
        solana_program::{
            hash::Hash,
            program_pack::{IsInitialized, Pack},
            rent::Rent,
            system_instruction,
        },
        solana_program_test::*,
        solana_sdk::{
            instruction::InstructionError,
            signature::{Keypair, Signer},
            transaction::{Transaction, TransactionError},
            transport::TransportError,
        },
        spl_token::{
            instruction::{initialize_account, initialize_mint, mint_to},
            state::{Account as TokenAccount, Mint},
        },
        std::str::FromStr,
    };

    type MerchantResult = (Pubkey, Pubkey, BanksClient, Keypair, Hash);

    fn create_mint_transaction(
        payer: &Keypair,
        mint: &Keypair,
        mint_authority: &Keypair,
        recent_blockhash: Hash,
    ) -> Transaction {
        let instructions = [
            system_instruction::create_account(
                &payer.pubkey(),
                &mint.pubkey(),
                Rent::default().minimum_balance(Mint::LEN),
                Mint::LEN as u64,
                &spl_token::id(),
            ),
            initialize_mint(
                &spl_token::id(),
                &mint.pubkey(),
                &mint_authority.pubkey(),
                None,
                0,
            )
            .unwrap(),
        ];
        let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
        transaction.partial_sign(&[payer, mint], recent_blockhash);
        transaction
    }

    fn create_token_account_transaction(
        payer: &Keypair,
        mint: &Keypair,
        recent_blockhash: Hash,
        token_account: &Keypair,
        token_account_owner: &Pubkey,
        amount: u64,
    ) -> Transaction {
        let instructions = [
            system_instruction::create_account(
                &payer.pubkey(),
                &token_account.pubkey(),
                Rent::default().minimum_balance(TokenAccount::LEN),
                TokenAccount::LEN as u64,
                &spl_token::id(),
            ),
            initialize_account(
                &spl_token::id(),
                &token_account.pubkey(),
                &mint.pubkey(),
                token_account_owner,
            )
            .unwrap(),
            mint_to(
                &spl_token::id(),
                &mint.pubkey(),
                &token_account.pubkey(),
                token_account_owner,
                &[&payer.pubkey()],
                amount,
            )
            .unwrap(),
        ];
        let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
        transaction.partial_sign(&[payer, token_account], recent_blockhash);
        transaction
    }

    async fn create_merchant_account(
        seed: Option<String>,
        fee: Option<u64>,
        sponsor: Option<&Pubkey>,
        data: Option<String>,
    ) -> MerchantResult {
        let program_id = Pubkey::from_str(&"mosh111111111111111111111111111111111111111").unwrap();

        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "sol_payment_processor",
            program_id,
            processor!(PaymentProcessorInstruction::process),
        )
        .start()
        .await;

        let real_seed = match &seed {
            None => MERCHANT,
            Some(value) => &value,
        };

        // first we create a public key for the merchant account
        let merchant_acc_pubkey =
            Pubkey::create_with_seed(&payer.pubkey(), real_seed, &program_id).unwrap();

        // then call register merchant ix
        let mut transaction = Transaction::new_with_payer(
            &[register_merchant(
                program_id,
                payer.pubkey(),
                merchant_acc_pubkey,
                Some(real_seed.to_string()),
                fee,
                data,
                sponsor,
            )],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
        return (
            program_id,
            merchant_acc_pubkey,
            banks_client,
            payer,
            recent_blockhash,
        );
    }

    async fn prepare_order(
        program_id: &Pubkey,
        merchant: &Pubkey,
        mint: &Pubkey,
        banks_client: &mut BanksClient,
    ) -> (Keypair, Pubkey, Pubkey, MerchantAccount) {
        let order_acc_keypair = Keypair::new();

        let (pda, _bump_seed) = Pubkey::find_program_address(&[PDA_SEED], &program_id);

        let (seller_token, _bump_seed) = Pubkey::find_program_address(
            &[
                &order_acc_keypair.pubkey().to_bytes(),
                &spl_token::id().to_bytes(),
                &mint.to_bytes(),
            ],
            program_id,
        );

        let merchant_account = banks_client.get_account(*merchant).await;
        let merchant_data = match merchant_account {
            Ok(data) => match data {
                None => panic!("Oo"),
                Some(value) => match MerchantAccount::unpack(&value.data) {
                    Ok(data) => data,
                    Err(error) => panic!("Problem: {:?}", error),
                },
            },
            Err(error) => panic!("Problem: {:?}", error),
        };

        (order_acc_keypair, seller_token, pda, merchant_data)
    }

    async fn create_token_account(
        amount: u64,
        mint_keypair: &Keypair,
        merchant_result: &mut MerchantResult,
    ) -> Keypair {
        // next create token account for test
        let buyer_token_keypair = Keypair::new();

        // create and initialize mint
        assert_matches!(
            merchant_result
                .2
                .process_transaction(create_mint_transaction(
                    &merchant_result.3,
                    &mint_keypair,
                    &merchant_result.3,
                    merchant_result.4
                ))
                .await,
            Ok(())
        );
        // create and initialize buyer token account
        assert_matches!(
            merchant_result
                .2
                .process_transaction(create_token_account_transaction(
                    &merchant_result.3,
                    &mint_keypair,
                    merchant_result.4,
                    &buyer_token_keypair,
                    &merchant_result.3.pubkey(),
                    amount + 2000000,
                ))
                .await,
            Ok(())
        );

        buyer_token_keypair
    }

    async fn create_order_express_checkout(
        amount: u64,
        order_id: &String,
        secret: &String,
        data: Option<String>,
        merchant_result: &mut MerchantResult,
        mint_keypair: &Keypair,
    ) -> (Pubkey, Pubkey) {
        let buyer_token_keypair = create_token_account(amount, mint_keypair, merchant_result).await;
        let (order_acc_keypair, seller_token, pda, merchant_data) = prepare_order(
            &merchant_result.0,
            &merchant_result.1,
            &mint_keypair.pubkey(),
            &mut merchant_result.2,
        )
        .await;

        // call express checkout ix
        let mut transaction = Transaction::new_with_payer(
            &[express_checkout(
                merchant_result.0,
                merchant_result.3.pubkey(),
                order_acc_keypair.pubkey(),
                merchant_result.1,
                seller_token,
                buyer_token_keypair.pubkey(),
                mint_keypair.pubkey(),
                Pubkey::from_str(PROGRAM_OWNER).unwrap(),
                Pubkey::new_from_array(merchant_data.sponsor),
                pda,
                amount,
                (&order_id).to_string(),
                (&secret).to_string(),
                data,
            )],
            Some(&merchant_result.3.pubkey()),
        );
        transaction.sign(&[&merchant_result.3, &order_acc_keypair], merchant_result.4);
        assert_matches!(
            &mut merchant_result.2.process_transaction(transaction).await,
            Ok(())
        );

        (order_acc_keypair.pubkey(), seller_token)
    }

    async fn create_chain_checkout_transaction(
        amount: u64,
        order_items: &OrderItems,
        data: Option<String>,
        merchant_result: &mut MerchantResult,
        mint_keypair: &Keypair,
    ) -> Result<(Pubkey, Pubkey), TransportError> {
        let buyer_token_keypair = create_token_account(amount, mint_keypair, merchant_result).await;
        let (order_acc_keypair, seller_token, pda, merchant_data) = prepare_order(
            &merchant_result.0,
            &merchant_result.1,
            &mint_keypair.pubkey(),
            &mut merchant_result.2,
        )
        .await;
        let order_items = order_items.clone();

        // call chain checkout ix
        let mut transaction = Transaction::new_with_payer(
            &[chain_checkout(
                merchant_result.0,
                merchant_result.3.pubkey(),
                order_acc_keypair.pubkey(),
                merchant_result.1,
                seller_token,
                buyer_token_keypair.pubkey(),
                mint_keypair.pubkey(),
                Pubkey::from_str(PROGRAM_OWNER).unwrap(),
                Pubkey::new_from_array(merchant_data.sponsor),
                pda,
                amount,
                order_items,
                data,
            )],
            Some(&merchant_result.3.pubkey()),
        );
        transaction.sign(&[&merchant_result.3, &order_acc_keypair], merchant_result.4);
        let _result = merchant_result.2.process_transaction(transaction).await?;
        Ok((order_acc_keypair.pubkey(), seller_token))
    }

    async fn create_order_chain_checkout(
        amount: u64,
        order_items: &OrderItems,
        data: Option<String>,
        merchant_result: &mut MerchantResult,
        mint_keypair: &Keypair,
    ) -> (Pubkey, Pubkey) {
        let transaction = create_chain_checkout_transaction(
            amount,
            order_items,
            data,
            merchant_result,
            mint_keypair,
        )
        .await;

        assert!(transaction.is_ok());
        transaction.unwrap()
    }

    async fn run_merchant_tests(result: MerchantResult) -> MerchantAccount {
        let program_id = result.0;
        let merchant = result.1;
        let mut banks_client = result.2;
        let payer = result.3;
        // test contents of merchant account
        let merchant_account = banks_client.get_account(merchant).await;
        let merchant_account = match merchant_account {
            Ok(data) => match data {
                None => panic!("Oo"),
                Some(value) => value,
            },
            Err(error) => panic!("Problem: {:?}", error),
        };
        assert_eq!(merchant_account.owner, program_id);
        let merchant_data = MerchantAccount::unpack(&merchant_account.data);
        let merchant_data = match merchant_data {
            Ok(data) => data,
            Err(error) => panic!("Problem: {:?}", error),
        };
        assert_eq!(true, merchant_data.is_initialized());
        assert_eq!(payer.pubkey(), Pubkey::new_from_array(merchant_data.owner));

        merchant_data
    }

    #[tokio::test]
    async fn test_register_merchant() {
        let result =
            create_merchant_account(Option::None, Option::None, Option::None, Option::None).await;
        let merchant_data = run_merchant_tests(result).await;
        assert_eq!(DEFAULT_FEE_IN_LAMPORTS, merchant_data.fee);
        assert_eq!(String::from("{}"), merchant_data.data);
    }

    #[tokio::test]
    async fn test_register_merchant_with_seed() {
        let result = create_merchant_account(
            Some(String::from("mosh")),
            Option::None,
            Option::None,
            Option::None,
        )
        .await;
        let merchant = result.1;
        let payer = result.3;
        let program_id = result.0;
        assert_eq!(
            merchant,
            Pubkey::create_with_seed(&payer.pubkey(), "mosh", &program_id).unwrap()
        );
    }

    #[tokio::test]
    /// assert that the minimum fee is used when custom fee too low
    async fn test_register_merchant_fee_default() {
        let result =
            create_merchant_account(Option::None, Some(10), Option::None, Option::None).await;
        let merchant_data = run_merchant_tests(result).await;
        assert_eq!(MIN_FEE_IN_LAMPORTS, merchant_data.fee);
    }

    #[tokio::test]
    async fn test_register_merchant_with_all_stuff() {
        let seed = String::from("mosh");
        let sponsor_pk = Pubkey::new_unique();
        let data = String::from(
            r#"{"code":200,"success":true,"payload":{"features":["awesome","easyAPI","lowLearningCurve"]}}"#,
        );
        let datas = data.clone();
        let result =
            create_merchant_account(Some(seed), Some(90000), Some(&sponsor_pk), Some(data)).await;
        let merchant_data = run_merchant_tests(result).await;
        assert_eq!(datas, merchant_data.data);
        assert_eq!(90000, merchant_data.fee);
        assert_eq!(sponsor_pk, Pubkey::new_from_array(merchant_data.sponsor));
        // just for sanity verify that you can get some of the JSON values
        let json_value: Value = serde_json::from_str(&merchant_data.data).unwrap();
        assert_eq!(200, json_value["code"]);
        assert_eq!(true, json_value["success"]);
    }

    async fn run_common_checkout_tests(
        amount: u64,
        merchant_result: &mut MerchantResult,
        order_acc_pubkey: &Pubkey,
        seller_account_pubkey: &Pubkey,
        mint_keypair: &Keypair,
    ) -> OrderAccount {
        // program_id => merchant_result.0;
        // merchant_account_pubkey => merchant_result.1;
        // banks_client => merchant_result.2;
        // payer => merchant_result.3;

        let order_account = merchant_result.2.get_account(*order_acc_pubkey).await;
        let order_account = match order_account {
            Ok(data) => match data {
                None => panic!("Oo"),
                Some(value) => value,
            },
            Err(error) => panic!("Problem: {:?}", error),
        };
        assert_eq!(order_account.owner, merchant_result.0,);

        let order_data = OrderAccount::unpack(&order_account.data);
        let order_data = match order_data {
            Ok(data) => data,
            Err(error) => panic!("Problem: {:?}", error),
        };
        assert_eq!(true, order_data.is_initialized());
        assert_eq!(OrderStatus::Paid as u8, order_data.status);
        assert_eq!(merchant_result.1.to_bytes(), order_data.merchant);
        assert_eq!(mint_keypair.pubkey().to_bytes(), order_data.mint);
        assert_eq!(seller_account_pubkey.to_bytes(), order_data.token);
        assert_eq!(merchant_result.3.pubkey().to_bytes(), order_data.payer);
        assert_eq!(amount, order_data.expected_amount);
        assert_eq!(amount, order_data.paid_amount);
        assert_eq!(
            order_account.lamports,
            Rent::default().minimum_balance(get_order_account_size(
                &order_data.order_id,
                &order_data.secret,
                &order_data.data,
            ))
        );

        // test contents of seller token account
        let seller_token_account = merchant_result.2.get_account(*seller_account_pubkey).await;
        let seller_token_account = match seller_token_account {
            Ok(data) => match data {
                None => panic!("Oo"),
                Some(value) => value,
            },
            Err(error) => panic!("Problem: {:?}", error),
        };
        let seller_account_data = spl_token::state::Account::unpack(&seller_token_account.data);
        let seller_account_data = match seller_account_data {
            Ok(data) => data,
            Err(error) => panic!("Problem: {:?}", error),
        };
        let (pda, _bump_seed) = Pubkey::find_program_address(&[PDA_SEED], &merchant_result.0);
        assert_eq!(amount, seller_account_data.amount);
        assert_eq!(pda, seller_account_data.owner);
        assert_eq!(mint_keypair.pubkey(), seller_account_data.mint);

        // test that sponsor was saved okay
        let merchant_account = merchant_result.2.get_account(merchant_result.1).await;
        let merchant_data = match merchant_account {
            Ok(data) => match data {
                None => panic!("Oo"),
                Some(value) => match MerchantAccount::unpack(&value.data) {
                    Ok(data) => data,
                    Err(error) => panic!("Problem: {:?}", error),
                },
            },
            Err(error) => panic!("Problem: {:?}", error),
        };

        let program_owner_key = Pubkey::from_str(PROGRAM_OWNER).unwrap();
        let sponsor = Pubkey::new_from_array(merchant_data.sponsor);

        let program_owner_account = merchant_result.2.get_account(program_owner_key).await;
        let program_owner_account = match program_owner_account {
            Ok(data) => match data {
                None => panic!("Oo"),
                Some(value) => value,
            },
            Err(error) => panic!("Problem: {:?}", error),
        };

        if sponsor == program_owner_key {
            // test contents of program owner account
            assert_eq!(merchant_data.fee, program_owner_account.lamports);
        } else {
            // test contents of program owner account and sponsor account
            let (program_owner_fee, sponsor_fee) = get_amounts(merchant_data.fee, SPONSOR_FEE);
            let sponsor_account = merchant_result.2.get_account(sponsor).await;
            let sponsor_account = match sponsor_account {
                Ok(data) => match data {
                    None => panic!("Oo"),
                    Some(value) => value,
                },
                Err(error) => panic!("Problem: {:?}", error),
            };
            assert_eq!(program_owner_fee, program_owner_account.lamports);
            assert_eq!(sponsor_fee, sponsor_account.lamports);
        }

        order_data
    }

    async fn run_checkout_tests(
        amount: u64,
        order_id: String,
        secret: String,
        data: Option<String>,
        merchant_result: &mut MerchantResult,
        order_acc_pubkey: &Pubkey,
        seller_account_pubkey: &Pubkey,
        mint_keypair: &Keypair,
    ) {
        let order_data = run_common_checkout_tests(
            amount,
            merchant_result,
            order_acc_pubkey,
            seller_account_pubkey,
            mint_keypair,
        )
        .await;

        let data_string = match data {
            None => String::from("{}"),
            Some(value) => value,
        };
        assert_eq!(order_id, order_data.order_id);
        assert_eq!(secret, order_data.secret);
        assert_eq!(data_string, order_data.data);
    }

    async fn run_chain_checkout_tests(
        amount: u64,
        order_items: &OrderItems,
        data: Option<String>,
        merchant_result: &mut MerchantResult,
        order_acc_pubkey: &Pubkey,
        seller_account_pubkey: &Pubkey,
        mint_keypair: &Keypair,
    ) {
        // test contents of order account
        let order_data = run_common_checkout_tests(
            amount,
            merchant_result,
            order_acc_pubkey,
            seller_account_pubkey,
            mint_keypair,
        )
        .await;
        match data {
            None => {
                assert_eq!(json!({ PAID: order_items }).to_string(), order_data.data);
            }
            Some(value) => {
                let json_data: Value = match serde_json::from_str(&value) {
                    Err(error) => panic!("Problem: {:?}", error),
                    Ok(data) => data,
                };
                assert_eq!(
                    json!({ INITIAL: json_data, PAID: order_items }).to_string(),
                    order_data.data
                );
            }
        }
    }

    #[tokio::test]
    async fn test_chain_checkout() {
        let mint_keypair = Keypair::new();
        let amount: u64 = 2000000000;

        let mut order_items: OrderItems = BTreeMap::new();
        order_items.insert("1".to_string(), 1);
        order_items.insert("3".to_string(), 1);

        let merchant_data = format!(
            r#"{{
            "1": {{"price": 2000000, "mint": "{mint_key}"}},
            "2": {{"price": 3000000, "mint": "{mint_key}"}},
            "3": {{"price": 4000000, "mint": "{mint_key}"}},
            "4": {{"price": 4000000, "mint": "{mint_key}"}},
            "5": {{"price": 4000000, "mint": "{mint_key}"}}
        }}"#,
            mint_key = mint_keypair.pubkey()
        );

        let mut merchant_result = create_merchant_account(
            Some("chain".to_string()),
            Option::None,
            Option::None,
            Some(merchant_data),
        )
        .await;
        let (order_acc_pubkey, seller_account_pubkey) = create_order_chain_checkout(
            amount,
            &order_items,
            Option::None,
            &mut merchant_result,
            &mint_keypair,
        )
        .await;

        run_chain_checkout_tests(
            amount,
            &order_items,
            Option::None,
            &mut merchant_result,
            &order_acc_pubkey,
            &seller_account_pubkey,
            &mint_keypair,
        )
        .await;
    }

    #[tokio::test]
    async fn test_chain_checkout_with_data() {
        let mint_keypair = Keypair::new();
        let amount: u64 = 2000000000;

        let mut order_items: OrderItems = BTreeMap::new();
        order_items.insert("1".to_string(), 1);

        let merchant_data = format!(
            r#"{{
            "1": {{"price": 2000000, "mint": "{mint_key}"}},
            "2": {{"price": 3000000, "mint": "{mint_key}"}}
        }}"#,
            mint_key = mint_keypair.pubkey()
        );

        let mut merchant_result = create_merchant_account(
            Some("chain2".to_string()),
            Option::None,
            Option::None,
            Some(merchant_data),
        )
        .await;
        let (order_acc_pubkey, seller_account_pubkey) = create_order_chain_checkout(
            amount,
            &order_items,
            Some(String::from(r#"{"foo": "bar"}"#)),
            &mut merchant_result,
            &mint_keypair,
        )
        .await;

        run_chain_checkout_tests(
            amount,
            &order_items,
            Some(String::from(r#"{"foo": "bar"}"#)),
            &mut merchant_result,
            &order_acc_pubkey,
            &seller_account_pubkey,
            &mint_keypair,
        )
        .await;
    }

    async fn chain_checkout_failing_test_helper(
        order_item_id: u8,
        paid_amount: u64,
        input_mint: &Keypair,
        registered_item_id: u8,
        expected_amount: u64,
        registered_mint: &Keypair,
        expected_error: InstructionError,
    ) -> bool {
        let mut order_items: OrderItems = BTreeMap::new();
        order_items.insert(format!("{}", order_item_id), 1);

        let mut merchant_data = String::from("5");

        if registered_item_id != 0 {
            merchant_data = format!(
                r#"{{"{registered_item_id}": {{"price": {expected_amount}, "mint": "{mint_key}"}}}}"#,
                registered_item_id = registered_item_id,
                expected_amount = expected_amount,
                mint_key = registered_mint.pubkey()
            );
        }

        let mut merchant_result = create_merchant_account(
            Some("test".to_string()),
            Option::None,
            Option::None,
            Some(merchant_data),
        )
        .await;

        match create_chain_checkout_transaction(
            paid_amount,
            &order_items,
            Option::None,
            &mut merchant_result,
            &input_mint,
        )
        .await
        {
            Err(error) => {
                assert_eq!(
                    error.unwrap(),
                    TransactionError::InstructionError(0, expected_error)
                );
            }
            Ok(_value) => panic!("Oo... we expect an error"),
        };

        true
    }

    #[tokio::test]
    async fn test_chain_checkout_failure() {
        let mint_a = Keypair::new();
        let mint_b = Keypair::new();

        // insufficient funds
        assert!(
            chain_checkout_failing_test_helper(
                1,       // id of item being ordered
                20,      // amount to pay
                &mint_a, // mint being used for payment
                1,       // registered item id
                30,      // expected amount
                &mint_a, // expected mint
                InstructionError::InsufficientFunds
            )
            .await
        );

        // wrong item id in order
        assert!(
            chain_checkout_failing_test_helper(
                7,       // id of item being ordered
                20,      // amount to pay
                &mint_a, // mint being used for payment
                1,       // registered item id
                30,      // expected amount
                &mint_a, // expected mint
                InstructionError::Custom(PaymentProcessorError::InvalidOrderData as u32)
            )
            .await
        );

        // wrong mint in order
        assert!(
            chain_checkout_failing_test_helper(
                1,       // id of item being ordered
                20,      // amount to pay
                &mint_a, // mint being used for payment
                1,       // registered item id
                20,      // expected amount
                &mint_b, // expected mint
                InstructionError::Custom(PaymentProcessorError::WrongMint as u32)
            )
            .await
        );

        // invalid merchant data
        assert!(
            chain_checkout_failing_test_helper(
                1,       // id of item being ordered
                20,      // amount to pay
                &mint_a, // mint being used for payment
                0,       // registered item id
                20,      // expected amount
                &mint_a, // expected mint
                InstructionError::Custom(PaymentProcessorError::InvalidMerchantData as u32)
            )
            .await
        );
    }

    #[tokio::test]
    async fn test_express_checkout() {
        let amount: u64 = 2000000000;
        let order_id = String::from("1337");
        let secret = String::from("hunter2");
        let mut merchant_result =
            create_merchant_account(Option::None, Option::None, Option::None, Option::None).await;
        let mint_keypair = Keypair::new();
        let (order_acc_pubkey, seller_account_pubkey) = create_order_express_checkout(
            amount,
            &order_id,
            &secret,
            Option::None,
            &mut merchant_result,
            &mint_keypair,
        )
        .await;

        run_checkout_tests(
            amount,
            order_id,
            secret,
            Option::None,
            &mut merchant_result,
            &order_acc_pubkey,
            &seller_account_pubkey,
            &mint_keypair,
        )
        .await;
    }

    #[tokio::test]
    /// test checkout with all merchant options
    async fn test_express_checkout_with_all_options() {
        let sponsor_pk = Pubkey::new_unique();
        let amount: u64 = 2000000000;
        let order_id = String::from("123-SQT-MX");
        let secret = String::from("supersecret");
        let mut merchant_result = create_merchant_account(
            Some(String::from("Oo")),
            Some(123456),
            Some(&sponsor_pk),
            Some(String::from(r#"{"foo": "bar"}"#)),
        )
        .await;
        let mint_keypair = Keypair::new();
        let (order_acc_pubkey, seller_account_pubkey) = create_order_express_checkout(
            amount,
            &order_id,
            &secret,
            Some(String::from(r#"{"a": "b"}"#)),
            &mut merchant_result,
            &mint_keypair,
        )
        .await;
        run_checkout_tests(
            amount,
            order_id,
            secret,
            Some(String::from(r#"{"a": "b"}"#)),
            &mut merchant_result,
            &order_acc_pubkey,
            &seller_account_pubkey,
            &mint_keypair,
        )
        .await;
    }

    async fn run_order_token_account_refund_tests(
        order_payment_token_acc: &Option<solana_sdk::account::Account>,
        account_to_receive_sol_refund_before: &Option<solana_sdk::account::Account>,
        account_to_receive_sol_refund_after: &Option<solana_sdk::account::Account>,
        previous_order_account: &Option<solana_sdk::account::Account>,
    ) {
        // order token account is closed
        assert!(order_payment_token_acc.is_none());
        let order_account_rent = match previous_order_account {
            None => 0,
            Some(account) => account.lamports,
        };
        match account_to_receive_sol_refund_before {
            None => panic!("Oo"),
            Some(account_before) => match account_to_receive_sol_refund_after {
                None => panic!("Oo"),
                Some(account_after) => {
                    // the before balance has increased by the rent amount of both token and order account
                    assert_eq!(
                        account_before.lamports,
                        account_after.lamports
                            - (Rent::default().minimum_balance(TokenAccount::LEN)
                                + order_account_rent)
                    );
                }
            },
        };
    }

    async fn withdraw_helper(
        amount: u64,
        close_order_account: bool,
    ) -> (
        BanksClient,
        Option<solana_sdk::account::Account>,
        Pubkey,
        Pubkey,
        Option<solana_sdk::account::Account>,
        Option<solana_sdk::account::Account>,
    ) {
        let mut merchant_result =
            create_merchant_account(Option::None, Option::None, Option::None, Option::None).await;
        let merchant_token_keypair = Keypair::new();
        let order_id = String::from("PD17CUSZ75");
        let secret = String::from("i love oov");
        let mint_keypair = Keypair::new();
        let (order_acc_pubkey, _seller_account_pubkey) = create_order_express_checkout(
            amount,
            &order_id,
            &secret,
            Option::None,
            &mut merchant_result,
            &mint_keypair,
        )
        .await;
        let program_id = merchant_result.0;
        let merchant_account_pubkey = merchant_result.1;
        let mut banks_client = merchant_result.2;
        let payer = merchant_result.3;
        let recent_blockhash = merchant_result.4;
        let (pda, _bump_seed) = Pubkey::find_program_address(&[PDA_SEED], &program_id);

        // create and initialize merchant token account
        assert_matches!(
            banks_client
                .process_transaction(create_token_account_transaction(
                    &payer,
                    &mint_keypair,
                    recent_blockhash,
                    &merchant_token_keypair,
                    &payer.pubkey(),
                    0,
                ))
                .await,
            Ok(())
        );
        let (order_payment_token_acc_pubkey, _bump_seed) = Pubkey::find_program_address(
            &[
                &order_acc_pubkey.to_bytes(),
                &spl_token::id().to_bytes(),
                &mint_keypair.pubkey().to_bytes(),
            ],
            &program_id,
        );

        let account_to_receive_sol_refund_pubkey = Pubkey::from_str(PROGRAM_OWNER).unwrap();
        let account_to_receive_sol_refund_before = banks_client
            .get_account(account_to_receive_sol_refund_pubkey)
            .await
            .unwrap();

        let previous_order_account = banks_client.get_account(order_acc_pubkey).await;
        let previous_order_account = match previous_order_account {
            Err(error) => panic!("Problem: {:?}", error),
            Ok(value) => value,
        };

        // call withdraw ix
        let mut transaction = Transaction::new_with_payer(
            &[withdraw(
                program_id,
                payer.pubkey(),
                order_acc_pubkey,
                merchant_account_pubkey,
                order_payment_token_acc_pubkey,
                merchant_token_keypair.pubkey(),
                account_to_receive_sol_refund_pubkey,
                pda,
                Option::None,
                close_order_account,
            )],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));

        // test contents of merchant token account
        let merchant_token_account = banks_client
            .get_account(merchant_token_keypair.pubkey())
            .await;
        let merchant_account_data = match merchant_token_account {
            Ok(data) => match data {
                None => panic!("Oo"),
                Some(value) => match spl_token::state::Account::unpack(&value.data) {
                    Ok(data) => data,
                    Err(error) => panic!("Problem: {:?}", error),
                },
            },
            Err(error) => panic!("Problem: {:?}", error),
        };
        assert_eq!(amount, merchant_account_data.amount);

        let order_account = banks_client.get_account(order_acc_pubkey).await;
        let order_account = match order_account {
            Err(error) => panic!("Problem: {:?}", error),
            Ok(value) => value,
        };

        (
            banks_client,
            order_account,
            order_payment_token_acc_pubkey,
            account_to_receive_sol_refund_pubkey,
            account_to_receive_sol_refund_before,
            previous_order_account,
        )
    }

    #[tokio::test]
    async fn test_withdraw() {
        let amount: u64 = 1234567890;
        let (
            mut banks_client,
            order_account,
            order_payment_token_acc_pubkey,
            account_to_receive_sol_refund_pubkey,
            account_to_receive_sol_refund_before,
            _previous_order_account,
        ) = withdraw_helper(amount, false).await;
        // test contents of order account
        let order_data = match order_account.clone() {
            None => panic!("Oo"),
            Some(value) => match OrderAccount::unpack(&value.data) {
                Ok(data) => data,
                Err(error) => panic!("Problem: {:?}", error),
            },
        };
        assert_eq!(OrderStatus::Withdrawn as u8, order_data.status);
        assert_eq!(amount, order_data.expected_amount);
        assert_eq!(amount, order_data.paid_amount);
        // test that token account was closed and that the refund was sent to expected account
        let order_payment_token_acc = banks_client
            .get_account(order_payment_token_acc_pubkey)
            .await
            .unwrap();
        let account_to_receive_sol_refund_after = banks_client
            .get_account(account_to_receive_sol_refund_pubkey)
            .await
            .unwrap();
        run_order_token_account_refund_tests(
            &order_payment_token_acc,
            &account_to_receive_sol_refund_before,
            &account_to_receive_sol_refund_after,
            &Option::None,
        )
        .await;
    }

    #[tokio::test]
    async fn test_withdraw_close_order_account() {
        let amount: u64 = 10001;
        let (
            mut banks_client,
            order_account,
            order_payment_token_acc_pubkey,
            account_to_receive_sol_refund_pubkey,
            account_to_receive_sol_refund_before,
            previous_order_account,
        ) = withdraw_helper(amount, true).await;
        // test closure of order account
        assert!(order_account.is_none());
        // test that accounts were closed and that refunds sent to expected account
        let order_payment_token_acc = banks_client
            .get_account(order_payment_token_acc_pubkey)
            .await
            .unwrap();
        let account_to_receive_sol_refund_after = banks_client
            .get_account(account_to_receive_sol_refund_pubkey)
            .await
            .unwrap();
        run_order_token_account_refund_tests(
            &order_payment_token_acc,
            &account_to_receive_sol_refund_before,
            &account_to_receive_sol_refund_after,
            &previous_order_account,
        )
        .await;
    }

    async fn run_subscribe_tests(
        amount: u64,
        package_name: &str,
        merchant_data: &str,
        mint_keypair: &Keypair,
    ) -> (
        Result<(), TransportError>,
        Option<(SubscriptionAccount, MerchantResult, Pubkey, Pubkey)>,
    ) {
        let mut merchant_result = create_merchant_account(
            Some(String::from("subscription test")),
            Option::None,
            Option::None,
            Some(String::from(merchant_data)),
        )
        .await;

        let (subscription, _bump_seed) = Pubkey::find_program_address(
            &[
                &merchant_result.3.pubkey().to_bytes(), // payer
                &merchant_result.1.to_bytes(),          // merchant
                &package_name.as_bytes(),
            ],
            &merchant_result.0, // program id
        );

        let order_data = format!(r#"{{"subscription": "{}"}}"#, subscription.to_string());

        let (order_acc_pubkey, _seller_account_pubkey) = create_order_express_checkout(
            amount,
            &String::from(package_name),
            &String::from(""),
            Some(order_data),
            &mut merchant_result,
            &mint_keypair,
        )
        .await;

        let program_id = merchant_result.0;
        let merchant_account_pubkey = merchant_result.1;
        let payer = &merchant_result.3;
        let recent_blockhash = merchant_result.4;

        // call subscribe ix
        let mut transaction = Transaction::new_with_payer(
            &[subscribe(
                program_id,
                payer.pubkey(),
                subscription,
                merchant_account_pubkey,
                order_acc_pubkey,
                String::from(package_name),
                Option::None,
            )],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[payer], recent_blockhash);

        let result = merchant_result.2.process_transaction(transaction).await;

        if result.is_ok() {
            // test contents of subscription token account
            let subscription_account = &merchant_result.2.get_account(subscription).await;
            let subscription_data = match subscription_account {
                Ok(data) => match data {
                    None => panic!("Oo"),
                    Some(value) => match SubscriptionAccount::unpack(&value.data) {
                        Ok(data) => data,
                        Err(error) => panic!("Problem: {:?}", error),
                    },
                },
                Err(error) => panic!("Problem: {:?}", error),
            };
            assert_eq!(
                (SubscriptionStatus::Initialized as u8),
                subscription_data.status
            );
            assert_eq!(String::from(package_name), subscription_data.name);
            assert_eq!(
                payer.pubkey(),
                Pubkey::new_from_array(subscription_data.owner)
            );
            assert_eq!(
                merchant_account_pubkey,
                Pubkey::new_from_array(subscription_data.merchant)
            );
            assert_eq!(String::from("{}"), subscription_data.data);

            return (
                result,
                Some((
                    subscription_data,
                    merchant_result,
                    order_acc_pubkey,
                    subscription,
                )),
            );
        }

        (result, Option::None)
    }

    #[tokio::test]
    async fn test_subscribe() {
        let mint_keypair = Keypair::new();
        let packages = format!(
            r#"{{"packages":[{{"name":"basic","price":1000000,"duration":720,"mint":"{mint}"}},{{"name":"annual","price":11000000,"duration":262800,"mint":"{mint}"}}]}}"#,
            mint = mint_keypair.pubkey().to_string()
        );
        assert!(
            (run_subscribe_tests(1000000, "basic", &packages, &mint_keypair).await)
                .0
                .is_ok()
        );
    }

    #[tokio::test]
    /// test what happens when there are 0 packages
    async fn test_subscribe_no_packages() {
        let mint_keypair = Keypair::new();
        let packages = r#"{"packages":[]}"#;
        assert!(
            (run_subscribe_tests(1337, "basic", packages, &mint_keypair).await)
                .0
                .is_err()
        );
    }

    #[tokio::test]
    /// test what happens when there are duplicate packages
    async fn test_subscribe_duplicate_packages() {
        let mint_keypair = Keypair::new();
        let packages = format!(
            r#"{{"packages":[{{"name":"a","price":100,"duration":720,"mint":"{mint}"}},{{"name":"a","price":222,"duration":262800,"mint":"{mint}"}}]}}"#,
            mint = mint_keypair.pubkey().to_string()
        );

        let result = run_subscribe_tests(100, "a", &packages, &mint_keypair).await;
        assert!(result.0.is_ok());

        let _ = match result.1 {
            None => (),
            Some(value) => {
                let subscription_account = value.0;
                // use the duration of the first package in the array to check
                // that the subscription was created using the first array element
                assert_eq!(
                    720,
                    subscription_account.period_end - subscription_account.period_start
                );
                ()
            }
        };
    }

    #[tokio::test]
    /// test what happens when the package is not found
    async fn test_subscribe_package_not_found() {
        let mint_keypair = Keypair::new();
        let packages = format!(
            r#"{{"packages":[{{"name":"a","price":100,"duration":720,"mint":"{mint}"}}]}}"#,
            mint = mint_keypair.pubkey().to_string()
        );
        assert!(
            (run_subscribe_tests(100, "zz", &packages, &mint_keypair).await)
                .0
                .is_err()
        );
    }

    #[tokio::test]
    /// test what happens when there is no packages object in the JSON
    async fn test_subscribe_no_packages_json() {
        let mint_keypair = Keypair::new();
        assert!(
            (run_subscribe_tests(250, "package", r#"{}"#, &mint_keypair).await)
                .0
                .is_err()
        );
    }

    #[tokio::test]
    /// test what happens when there is no valid JSON
    async fn test_subscribe_no_json() {
        let mint_keypair = Keypair::new();
        assert!(
            (run_subscribe_tests(250, "package", "what is?", &mint_keypair).await)
                .0
                .is_err()
        );
    }

    #[tokio::test]
    /// test what happens when the amount paid is insufficient
    async fn test_subscribe_not_enough_paid() {
        let mint_keypair = Keypair::new();
        let packages = format!(
            r#"{{"packages":[{{"name":"basic","price":100,"duration":720,"mint":"{mint}"}}]}}"#,
            mint = mint_keypair.pubkey().to_string()
        );
        assert!(
            (run_subscribe_tests(10, "Netflix-basic", &packages, &mint_keypair).await)
                .0
                .is_err()
        );
    }

    #[tokio::test]
    async fn test_subscription_renewal() {
        let mint_keypair = Keypair::new();
        let name = "short";
        // create a package that lasts only 1 second
        let packages = format!(
            r#"{{"packages":[{{"name":"{name}","price":999999,"duration":1,"mint":"{mint}"}}]}}"#,
            mint = mint_keypair.pubkey().to_string(),
            name = name
        );
        let result = run_subscribe_tests(1000000, name, &packages, &mint_keypair).await;
        assert!(result.0.is_ok());
        let subscribe_result = result.1;
        let _ = match subscribe_result {
            None => (),
            Some(mut subscribe_result) => {
                let subscription_account = subscribe_result.0;
                let subscription = subscribe_result.3; // the subscription pubkey

                let order_data = format!(r#"{{"subscription": "{}"}}"#, subscription.to_string());

                let (order_acc_pubkey, _seller_account_pubkey) = create_order_express_checkout(
                    999999 * 600,
                    &format!("{name}", name = name),
                    &String::from(""),
                    Some(order_data),
                    &mut subscribe_result.1,
                    &mint_keypair,
                )
                .await;

                // call subscription  ix
                let mut transaction = Transaction::new_with_payer(
                    &[renew_subscription(
                        subscribe_result.1 .0,          // program_id,
                        subscribe_result.1 .3.pubkey(), // payer,
                        subscription,
                        Pubkey::new_from_array(subscription_account.merchant),
                        order_acc_pubkey,
                        600,
                    )],
                    Some(&subscribe_result.1 .3.pubkey()),
                );
                transaction.sign(&[&subscribe_result.1 .3], subscribe_result.1 .4);
                assert_matches!(
                    subscribe_result.1 .2.process_transaction(transaction).await,
                    Ok(())
                );

                // assert that period end has been updated
                let subscription_account2 = subscribe_result.1 .2.get_account(subscription).await;
                let subscription_account2 = match subscription_account2 {
                    Ok(data) => match data {
                        None => panic!("Oo"),
                        Some(value) => match SubscriptionAccount::unpack(&value.data) {
                            Ok(data) => data,
                            Err(error) => panic!("Problem: {:?}", error),
                        },
                    },
                    Err(error) => panic!("Problem: {:?}", error),
                };
                assert_eq!(
                    // the new period_end is equal to the old period_end + (1 * 600)
                    subscription_account.period_end + 600,
                    subscription_account2.period_end
                );

                return ();
            }
        };
    }

    async fn run_subscription_withdrawal_tests(
        name: &str,
        packages: &str,
        mint_keypair: &Keypair,
        error_expected: bool,
    ) {
        // create the subscription
        let result = run_subscribe_tests(1000000, name, &packages, &mint_keypair).await;
        assert!(result.0.is_ok());
        let subscribe_result = result.1;
        let _ = match subscribe_result {
            None => (),
            Some(mut subscribe_result) => {
                let subscription = subscribe_result.3; // the subscription pubkey
                let order_acc_pubkey = subscribe_result.2;
                let merchant_token_keypair = Keypair::new();
                let (pda, _bump_seed) =
                    Pubkey::find_program_address(&[PDA_SEED], &subscribe_result.1 .0);

                // create and initialize merchant token account
                assert_matches!(
                    subscribe_result
                        .1
                         .2
                        .process_transaction(create_token_account_transaction(
                            &subscribe_result.1 .3,
                            &mint_keypair,
                            subscribe_result.1 .4, // recent_blockhash
                            &merchant_token_keypair,
                            &subscribe_result.1 .3.pubkey(), // payer,
                            0,
                        ))
                        .await,
                    Ok(())
                );
                let (order_payment_token_acc_pubkey, _bump_seed) = Pubkey::find_program_address(
                    &[
                        &order_acc_pubkey.to_bytes(),
                        &spl_token::id().to_bytes(),
                        &mint_keypair.pubkey().to_bytes(),
                    ],
                    &subscribe_result.1 .0, // program_id
                );

                // call withdraw ix
                let mut transaction = Transaction::new_with_payer(
                    &[withdraw(
                        subscribe_result.1 .0,          // program_id
                        subscribe_result.1 .3.pubkey(), // payer,
                        order_acc_pubkey,
                        subscribe_result.1 .1, // the merchant pubkey
                        order_payment_token_acc_pubkey,
                        merchant_token_keypair.pubkey(),
                        Pubkey::from_str(PROGRAM_OWNER).unwrap(),
                        pda,
                        Some(subscription),
                        false,
                    )],
                    Some(&subscribe_result.1 .3.pubkey()),
                );
                transaction.sign(&[&subscribe_result.1 .3], subscribe_result.1 .4);

                if error_expected {
                    assert!(subscribe_result
                        .1
                         .2
                        .process_transaction(transaction)
                        .await
                        .is_err());
                } else {
                    assert!(subscribe_result
                        .1
                         .2
                        .process_transaction(transaction)
                        .await
                        .is_ok());
                }

                return ();
            }
        };
    }

    #[tokio::test]
    async fn test_withdraw_during_trial() {
        let mint_keypair = Keypair::new();
        let name = "trialFirst";
        // create a package that has a short trial period
        let packages = format!(
            r#"{{"packages":[{{"name":"{name}","price":99,"trial":0,"duration":604800,"mint":"{mint}"}}]}}"#,
            mint = mint_keypair.pubkey().to_string(),
            name = name
        );
        // withdraw goes okay
        run_subscription_withdrawal_tests(name, &packages, &mint_keypair, false).await;
    }

    #[tokio::test]
    async fn test_cannot_withdraw_during_trial() {
        let mint_keypair = Keypair::new();
        let name = "try1st";
        // create a package that has a week long trial period
        let packages = format!(
            r#"{{"packages":[{{"name":"{name}","price":99,"trial":604800,"duration":604800,"mint":"{mint}"}}]}}"#,
            mint = mint_keypair.pubkey().to_string(),
            name = name
        );
        // withdrawal errors out as you cant withdraw during trial
        run_subscription_withdrawal_tests(name, &packages, &mint_keypair, true).await;
    }

    async fn run_subscription_cancel_tests(
        amount: u64,
        name: &str,
        packages: &str,
        mint_keypair: &Keypair,
    ) -> Option<(
        SubscriptionAccount,
        Option<solana_sdk::account::Account>,
        Option<solana_sdk::account::Account>,
        spl_token::state::Account,
        SubscriptionAccount,
        Option<solana_sdk::account::Account>,
        Option<solana_sdk::account::Account>,
        Option<solana_sdk::account::Account>,
    )> {
        // create the subscription
        let result = run_subscribe_tests(amount, name, &packages, &mint_keypair).await;
        assert!(result.0.is_ok());
        let subscribe_result = result.1;
        match subscribe_result {
            None => Option::None,
            Some(mut subscribe_result) => {
                let subscription = subscribe_result.3; // the subscription pubkey

                let previous_subscription_account =
                    subscribe_result.1 .2.get_account(subscription).await;
                let previous_subscription_account = match previous_subscription_account {
                    Ok(data) => match data {
                        None => panic!("Oo"),
                        Some(value) => match SubscriptionAccount::unpack(&value.data) {
                            Ok(data) => data,
                            Err(error) => panic!("Problem: {:?}", error),
                        },
                    },
                    Err(error) => panic!("Problem: {:?}", error),
                };

                let order_acc_pubkey = subscribe_result.2;
                let previous_order_account =
                    subscribe_result.1 .2.get_account(order_acc_pubkey).await;
                let previous_order_account = match previous_order_account {
                    Err(error) => panic!("Problem: {:?}", error),
                    Ok(value) => value,
                };

                let refund_token_acc_keypair = Keypair::new();
                let (pda, _bump_seed) =
                    Pubkey::find_program_address(&[PDA_SEED], &subscribe_result.1 .0);

                // create and initialize refund token account
                assert_matches!(
                    subscribe_result
                        .1
                         .2
                        .process_transaction(create_token_account_transaction(
                            &subscribe_result.1 .3,
                            &mint_keypair,
                            subscribe_result.1 .4, // recent_blockhash
                            &refund_token_acc_keypair,
                            &subscribe_result.1 .3.pubkey(), // payer,
                            0,
                        ))
                        .await,
                    Ok(())
                );
                let (order_token_acc_pubkey, _bump_seed) = Pubkey::find_program_address(
                    &[
                        &order_acc_pubkey.to_bytes(),
                        &spl_token::id().to_bytes(),
                        &mint_keypair.pubkey().to_bytes(),
                    ],
                    &subscribe_result.1 .0, // program_id
                );

                let account_to_receive_sol_refund_pubkey = Pubkey::from_str(PROGRAM_OWNER).unwrap();
                let account_to_receive_sol_refund_before = subscribe_result
                    .1
                     .2
                    .get_account(account_to_receive_sol_refund_pubkey)
                    .await
                    .unwrap();

                // call cancel ix
                let mut transaction = Transaction::new_with_payer(
                    &[cancel_subscription(
                        subscribe_result.1 .0,          // program_id
                        subscribe_result.1 .3.pubkey(), // payer,
                        subscription,
                        subscribe_result.1 .1, // the merchant pubkey
                        order_acc_pubkey,
                        order_token_acc_pubkey,
                        refund_token_acc_keypair.pubkey(),
                        account_to_receive_sol_refund_pubkey,
                        pda,
                    )],
                    Some(&subscribe_result.1 .3.pubkey()),
                );
                transaction.sign(&[&subscribe_result.1 .3], subscribe_result.1 .4);

                let _cancel_result = subscribe_result.1 .2.process_transaction(transaction).await;

                let subscription_account = subscribe_result.1 .2.get_account(subscription).await;
                let subscription_account = match subscription_account {
                    Ok(data) => match data {
                        None => panic!("Oo"),
                        Some(value) => match SubscriptionAccount::unpack(&value.data) {
                            Ok(data) => data,
                            Err(error) => panic!("Problem: {:?}", error),
                        },
                    },
                    Err(error) => panic!("Problem: {:?}", error),
                };
                let order_account = subscribe_result.1 .2.get_account(order_acc_pubkey).await;
                let order_account = match order_account {
                    Ok(value) => value,
                    Err(error) => panic!("Problem: {:?}", error),
                };
                let order_token_account = subscribe_result
                    .1
                     .2
                    .get_account(order_token_acc_pubkey)
                    .await
                    .unwrap();
                let refund_token_account = subscribe_result
                    .1
                     .2
                    .get_account(refund_token_acc_keypair.pubkey())
                    .await;
                let refund_token_account = match refund_token_account {
                    Ok(data) => match data {
                        None => panic!("Oo"),
                        Some(value) => match TokenAccount::unpack(&value.data) {
                            Ok(data) => data,
                            Err(error) => panic!("Problem: {:?}", error),
                        },
                    },
                    Err(error) => panic!("Problem: {:?}", error),
                };

                let account_to_receive_sol_refund_after = subscribe_result
                    .1
                     .2
                    .get_account(account_to_receive_sol_refund_pubkey)
                    .await
                    .unwrap();

                Some((
                    subscription_account,
                    order_account,
                    order_token_account,
                    refund_token_account,
                    previous_subscription_account,
                    previous_order_account,
                    account_to_receive_sol_refund_before,
                    account_to_receive_sol_refund_after,
                ))
            }
        }
    }

    #[tokio::test]
    async fn test_cancel_subscription_during_trial() {
        let mint_keypair = Keypair::new();
        let name = "trialFirst";
        // create a package that has a short trial period
        let packages = format!(
            r#"{{"packages":[{{"name":"{name}","price":6699,"trial":604800,"duration":604800,"mint":"{mint}"}}]}}"#,
            mint = mint_keypair.pubkey().to_string(),
            name = name
        );
        // cancel goes okay
        let result = run_subscription_cancel_tests(6699, name, &packages, &mint_keypair)
            .await
            .unwrap();
        let (
            subscription_account,
            order_account,
            order_token_account,
            refund_token_account,
            previous_subscription_account,
            previous_order_account,
            account_to_receive_sol_refund_before,
            account_to_receive_sol_refund_after,
        ) = result;
        // subscription was canceled
        assert_eq!(
            SubscriptionStatus::Initialized as u8,
            previous_subscription_account.status
        );
        assert_eq!(
            SubscriptionStatus::Cancelled as u8,
            subscription_account.status
        );
        // period end has changed to an earlier time
        assert!(previous_subscription_account.period_end > subscription_account.period_end);
        // order account was closed
        assert!(order_account.is_none());
        // amount was withdrawn
        assert_eq!(6699, refund_token_account.amount);
        // order token account was closed and SOL refunded
        run_order_token_account_refund_tests(
            &order_token_account,
            &account_to_receive_sol_refund_before,
            &account_to_receive_sol_refund_after,
            &previous_order_account,
        )
        .await;
    }

    #[tokio::test]
    async fn test_cancel_subscription_after_trial() {
        let mint_keypair = Keypair::new();
        let name = "trialFirst";
        // create a package that has a short trial period
        let packages = format!(
            r#"{{"packages":[{{"name":"{name}","price":1337,"trial":0,"duration":604800,"mint":"{mint}"}}]}}"#,
            mint = mint_keypair.pubkey().to_string(),
            name = name
        );
        // cancel goes okay but no refund
        let result = run_subscription_cancel_tests(1337, name, &packages, &mint_keypair)
            .await
            .unwrap();
        let (
            subscription_account,
            order_account,
            order_token_account,
            refund_token_account,
            previous_subscription_account,
            previous_order_account,
            account_to_receive_sol_refund_before,
            account_to_receive_sol_refund_after,
        ) = result;
        // subscription was canceled
        assert_eq!(
            SubscriptionStatus::Initialized as u8,
            previous_subscription_account.status
        );
        assert_eq!(
            SubscriptionStatus::Cancelled as u8,
            subscription_account.status
        );
        assert_eq!(
            previous_subscription_account.period_end,
            subscription_account.period_end
        );
        // order account was not changed
        let order_account = match order_account {
            None => panic!("Oo"),
            Some(value) => match OrderAccount::unpack(&value.data) {
                Ok(data) => data,
                Err(error) => panic!("Problem: {:?}", error),
            },
        };
        let previous_order_account = match previous_order_account {
            None => panic!("Oo"),
            Some(value) => match OrderAccount::unpack(&value.data) {
                Ok(data) => data,
                Err(error) => panic!("Problem: {:?}", error),
            },
        };
        assert_eq!(order_account, previous_order_account);
        assert_eq!(OrderStatus::Paid as u8, order_account.status);
        // nothing was refunded
        assert_eq!(0, refund_token_account.amount);
        let order_token_account = match order_token_account {
            None => panic!("Oo"),
            Some(value) => match TokenAccount::unpack(&value.data) {
                Ok(data) => data,
                Err(error) => panic!("Problem: {:?}", error),
            },
        };
        assert_eq!(order_account.paid_amount, order_token_account.amount);
        match account_to_receive_sol_refund_before {
            None => panic!("Oo"),
            Some(account_before) => match account_to_receive_sol_refund_after {
                None => panic!("Oo"),
                Some(account_after) => {
                    assert_eq!(account_before.lamports, account_after.lamports);
                }
            },
        };
    }
}
