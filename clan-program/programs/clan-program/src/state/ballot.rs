use anchor_lang::prelude::*;
#[account]
#[derive(Default, InitSpace)]
pub struct Ballot {
    pub proposal: Pubkey,
    pub member: Pubkey,
    pub vote: bool,
    pub amount: u64,
    pub created_at: i64,
    pub updated_at: i64,
}
