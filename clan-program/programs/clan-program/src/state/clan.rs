use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, PartialEq, InitSpace)]
pub enum ClanStatus {
    #[default]
    Inactive,
    Active,
}

#[account]
#[derive(Default, InitSpace)]
pub struct Clan {
    pub id: u64,
    pub creator: Pubkey,
    pub leader: Pubkey,
    pub status: ClanStatus,
    pub member_count: u64,
    pub power: u64,
    #[max_len(4)]
    pub symbol: String,
    #[max_len(32)]
    pub name: String,
    #[max_len(100)]
    pub uri: String,
    pub created_at: i64,
    pub updated_at: i64,
}
