use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct Proposal {
    pub id: u64,
    pub clan: Pubkey,
    pub author: Pubkey,
    pub amount: u64,
    pub vault: Pubkey,
    pub executed: bool,
    pub executed_by: Option<Pubkey>,
    pub votes: u64,
    pub yes_votes: u64,
    pub no_votes: u64,
    #[max_len(32)]
    pub title: String,
    #[max_len(200)]
    pub description: String,
    pub start_at: i64,
    pub end_at: i64,
    pub created_at: i64,
    pub executed_at: Option<i64>,
}
