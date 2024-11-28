use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, PartialEq, InitSpace)]
pub enum MemberStatus {
    #[default]
    Inactive,
    Active,
    Banned,
}

#[account]
#[derive(Default, InitSpace)]
pub struct Member {
    pub clan: Pubkey,
    pub wallet: Pubkey,
    pub mint_account: Pubkey,
    pub status: MemberStatus,
    pub power: u64,
    pub timestamp: i64,
}
