use crate::{
    engine::cancel_subscription::process_cancel_subscription,
    engine::pay::process_express_checkout, engine::pay::process_chain_checkout, engine::register::process_register_merchant,
    engine::renew::process_renew_subscription, engine::subscribe::process_subscribe,
    engine::withdraw::process_withdraw_payment, instruction::PaymentProcessorInstruction,
};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

/// Processes the instruction
impl PaymentProcessorInstruction {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = PaymentProcessorInstruction::try_from_slice(&instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        match instruction {
            PaymentProcessorInstruction::RegisterMerchant { seed, fee, data } => {
                msg!("SolPayments: RegisterMerchant");
                process_register_merchant(program_id, accounts, seed, fee, data)
            }
            PaymentProcessorInstruction::ExpressCheckout {
                amount,
                order_id,
                secret,
                data,
            } => {
                msg!("SolPayments: ExpressCheckout");
                process_express_checkout(program_id, accounts, amount, order_id, secret, data)
            }
            PaymentProcessorInstruction::ChainCheckout {
                amount,
                order_items,
                data,
            } => {
                msg!("SolPayments: ChainCheckout");
                process_chain_checkout(program_id, accounts, amount, order_items, data)
            }
            PaymentProcessorInstruction::Withdraw { close_order_account } => {
                msg!("SolPayments: Withdraw");
                process_withdraw_payment(program_id, accounts, close_order_account)
            }
            PaymentProcessorInstruction::Subscribe { name, data } => {
                msg!("SolPayments: Subscribe");
                process_subscribe(program_id, accounts, name, data)
            }
            PaymentProcessorInstruction::RenewSubscription { quantity } => {
                msg!("SolPayments: RenewSubscription");
                process_renew_subscription(program_id, accounts, quantity)
            }
            PaymentProcessorInstruction::CancelSubscription => {
                msg!("SolPayments: CancelSubscription");
                process_cancel_subscription(program_id, accounts)
            }
        }
    }
}
