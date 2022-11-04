/// the word merchant as a string
pub const MERCHANT: &str = "merchant";
/// the word trial as a string
pub const TRIAL: &str = "trial";
/// the word packages as a string
pub const PACKAGES: &str = "packages";
/// the word packages as a string
pub const PAID: &str = "_paid";
/// the word packages as a string
pub const INITIAL: &str = "_initial";
/// seed for pgram derived addresses
pub const PDA_SEED: &[u8] = b"sol_payment_processor";
/// the program owner
pub const PROGRAM_OWNER: &str = "mosh782eoKyPca9eotWfepHVSKavjDMBjNkNE3Gge6Z";
/// minimum transaction fee percentage
pub const MIN_FEE_IN_LAMPORTS: u64 = 50000;
/// default transaction fee percentage
pub const DEFAULT_FEE_IN_LAMPORTS: u64 = 500000;
/// sponsor fee percentage
pub const SPONSOR_FEE: u128 = 3;
/// default data value
pub const DEFAULT_DATA: &str = "{}";
// these are purely by trial and error ... TODO: understand these some more
/// the mem size of string ... apparently
pub const STRING_SIZE: usize = 4;
