use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    clock::UnixTimestamp,
    program_pack::{IsInitialized, Sealed},
};
use std::mem::size_of;

pub type PublicKey = [u8; 32];

pub trait Serdes: Sized + BorshSerialize + BorshDeserialize {
    fn pack(&self, dst: &mut [u8]) {
        let encoded = self.try_to_vec().unwrap();
        dst[..encoded.len()].copy_from_slice(&encoded);
    }
    fn unpack(src: &[u8]) -> Result<Self, std::io::Error> {
        Self::try_from_slice(src)
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum Discriminator {
    Uninitialized = 0,
    Merchant = 10,
    MerchantSubscription = 11,
    MerchantSubscriptionWithTrial = 12,
    MerchantChainCheckout = 15,
    OrderExpressCheckout = 20,
    OrderChainCheckout = 21,
    Subscription = 30,
    Closed = 255,
}

#[derive(BorshSerialize, BorshSchema, BorshDeserialize, Debug, PartialEq)]
pub struct MerchantAccount {
    pub discriminator: u8,
    pub owner: PublicKey,
    pub sponsor: PublicKey,
    /// represents the fee (in SOL lamports) that will be charged for transactions
    pub fee: u64,
    /// this is represented as a string but really is meant to hold JSON
    /// found this to be a convenient hack to allow flexible data
    pub data: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum OrderStatus {
    Uninitialized = 0,
    Pending = 1,
    Paid = 2,
    Withdrawn = 3,
    Cancelled = 4,
}

#[derive(BorshSerialize, BorshSchema, BorshDeserialize, Debug, PartialEq)]
pub struct OrderAccount {
    pub discriminator: u8,
    pub status: u8,
    pub created: UnixTimestamp,
    pub modified: UnixTimestamp,
    pub merchant: PublicKey,
    pub mint: PublicKey,  // represents the token/currency in use
    pub token: PublicKey, // represents the token account that holds the money
    pub payer: PublicKey,
    pub expected_amount: u64,
    pub paid_amount: u64,
    pub order_id: String,
    pub secret: String,
    /// this is represented as a string but really is meant to hold JSON
    /// found this to be a convenient hack to allow flexible data
    pub data: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum SubscriptionStatus {
    Uninitialized = 0,
    Initialized = 1,
    Cancelled = 2,
}

#[derive(BorshSerialize, BorshSchema, BorshDeserialize, Debug, PartialEq)]
pub struct SubscriptionAccount {
    pub discriminator: u8,
    pub status: u8,
    pub owner: PublicKey,
    pub merchant: PublicKey,
    pub name: String,
    pub joined: UnixTimestamp,
    pub period_start: UnixTimestamp,
    pub period_end: UnixTimestamp,
    /// this is represented as a string but really is meant to hold JSON
    /// found this to be a convenient hack to allow flexible data
    pub data: String,
}

// impl for MerchantAccount
impl Sealed for MerchantAccount {}

impl Serdes for MerchantAccount {}

impl MerchantAccount {
    pub const MIN_LEN: usize =
        size_of::<u8>() + size_of::<PublicKey>() + size_of::<PublicKey>() + size_of::<u64>();
}

// impl for OrderAccount
impl Sealed for OrderAccount {}

impl Serdes for OrderAccount {}

impl OrderAccount {
    pub const MIN_LEN: usize = size_of::<u8>()
        + size_of::<u8>()
        + size_of::<UnixTimestamp>()
        + size_of::<UnixTimestamp>()
        + size_of::<PublicKey>()
        + size_of::<PublicKey>()
        + size_of::<PublicKey>()
        + size_of::<PublicKey>()
        + size_of::<u64>()
        + size_of::<u64>();
}

// impl for SubscriptionAccount
impl Sealed for SubscriptionAccount {}

impl Serdes for SubscriptionAccount {}

impl SubscriptionAccount {
    pub const MIN_LEN: usize = size_of::<u8>()
        + size_of::<u8>()
        + size_of::<PublicKey>()
        + size_of::<PublicKey>()
        + size_of::<UnixTimestamp>()
        + size_of::<UnixTimestamp>()
        + size_of::<UnixTimestamp>();
}

/// Check if a program account state is closed
pub trait IsClosed {
    /// Is closed
    fn is_closed(&self) -> bool;
}

macro_rules! impl_IsInitialized {
    (for $($t:ty),+) => {
        $(impl IsInitialized for $t {
            fn is_initialized(&self) -> bool {
                self.discriminator != Discriminator::Uninitialized as u8
            }
        })*
    }
}

macro_rules! impl_IsClosed {
    (for $($t:ty),+) => {
        $(impl IsClosed for $t {
            fn is_closed(&self) -> bool {
                self.discriminator == Discriminator::Closed as u8
            }
        })*
    }
}

impl_IsInitialized!(for MerchantAccount, OrderAccount, SubscriptionAccount);
impl_IsClosed!(for MerchantAccount, OrderAccount, SubscriptionAccount);
