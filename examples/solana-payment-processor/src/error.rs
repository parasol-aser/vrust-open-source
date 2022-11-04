//! Error types

use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq, FromPrimitive)]
pub enum PaymentProcessorError {
    /// The Amount Is Already Withdrawn
    #[error("Error: The Amount Is Already Withdrawn")]
    AlreadyWithdrawn,
    /// Cannot withdraw during trial period
    #[error("Error: Cannot withdraw during trial period")]
    CantWithdrawDuringTrial,
    /// Account already closed
    #[error("Error: Account already closed")]
    ClosedAccount,
    /// Invalid instruction
    #[error("Error: Invalid Instruction")]
    InvalidInstruction,
    /// Invalid Merchant Data
    #[error("Error: Invalid Merchant Data")]
    InvalidMerchantData,
    /// Invalid Subscription Data
    #[error("Error: Invalid Subscription Data")]
    InvalidSubscriptionData,
    /// Invalid Subscription Package
    #[error("Error: Invalid Subscription Package")]
    InvalidSubscriptionPackage,
    /// The Order Account Is Invalid
    #[error("Error: The Order Account Is Invalid")]
    InvalidOrder,
    /// The Order Data Is Invalid
    #[error("Error: The Order Data Is Invalid")]
    InvalidOrderData,
    /// Seller And Buyer Mints Not The Same
    #[error("Error: Seller And Buyer Mints Not The Same")]
    MintNotEqual,
    /// The Payment Has Not Been Received In Full
    #[error("Error: The Payment Has Not Been Received In Full")]
    NotFullyPaid,
    /// The Payment Has Not Yet Been Made
    #[error("Error: The Payment Has Not Yet Been Made")]
    NotPaid,
    /// The Provided Merchant Is Wrong
    #[error("Error: The Provided Merchant Is Wrong")]
    WrongMerchant,
    /// The Provided Order Account Is Wrong
    #[error("Error: The Provided Order Account Is Wrong")]
    WrongOrderAccount,
    /// The Payer Is Wrong
    #[error("Error: The Payer Is Wrong")]
    WrongPayer,
    /// The Provided Program Owner Is Wrong
    #[error("Error: The Provided Program Owner Is Wrong")]
    WrongProgramOwner,
    /// The Provided Sponsor Is Wrong
    #[error("Error: The Provided Sponsor Is Wrong")]
    WrongSponsor,
    /// The Provided mint Is Wrong
    #[error("Error: The Provided mint Is Wrong")]
    WrongMint,
}

impl From<PaymentProcessorError> for ProgramError {
    fn from(e: PaymentProcessorError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for PaymentProcessorError {
    fn type_of() -> &'static str {
        "Solana Payment Processor Error"
    }
}

impl PrintProgramError for PaymentProcessorError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}
