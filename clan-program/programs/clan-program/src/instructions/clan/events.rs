use anchor_lang::prelude::*;

use crate::ClanStatus;

#[event]
pub struct ClanCreatedEvent {
    pub clan_id: u64,
    pub creator: Pubkey,
    pub leader: Pubkey,
    pub status: ClanStatus,
    pub member_count: u64,
    pub power: u64,
    pub symbol: String,
    pub name: String,
    pub uri: String,
    pub created_at: i64,
    pub updated_at: i64,
}
