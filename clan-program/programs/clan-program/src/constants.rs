use anchor_lang::prelude::*;

pub const DISCRIMINATOR_LENGTH: usize = std::mem::size_of::<u64>(); //8
pub const PUBLIC_KEY_LENGTH: usize = std::mem::size_of::<Pubkey>(); //32
pub const TIMESTAMP_LENGTH: usize = std::mem::size_of::<i64>(); //8

pub const STRING_LENGTH_PREFIX: usize = 4;
pub const BOOL_SIZE: usize = std::mem::size_of::<bool>();
pub const I64_SIZE: usize = std::mem::size_of::<i64>();
pub const U64_SIZE: usize = std::mem::size_of::<u64>();
pub const U8_SIZE: usize = std::mem::size_of::<u8>();
pub const STATUS_ENUM_SIZE: usize = 1 + std::mem::size_of::<u8>();

pub const MAX_NAME_LENGTH: usize = 32 * 4;
pub const MAX_SYMBOL_LENGTH: usize = 4 * 4;
pub const MAX_URI_LENGTH: usize = 100 * 4;
pub const MAX_TITLE_LENGTH: usize = 32 * 4;
pub const MAX_DESC_LENGTH: usize = 200 * 4;

pub const CLAN_SEED: &[u8] = b"clan";
pub const MEMBER_SEED: &[u8] = b"member";
pub const TREASURER_SEED: &[u8] = b"treasurer";
pub const CARD_SEED: &[u8] = b"card";
pub const PROPOSAL_SEED: &[u8] = b"proposal";
pub const BALLOT_SEED: &[u8] = b"ballot";
pub const VAULT_SEED: &[u8] = b"vault";

pub const DEFAULT_SYMBOL: &str = "SCLA";
