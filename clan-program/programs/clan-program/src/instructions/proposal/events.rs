use anchor_lang::prelude::*;

#[event]
pub struct ProposalAmountUpdated {
    pub proposal: Pubkey,
    pub author: Pubkey,
    pub executor: Pubkey,
    pub old_amount: u64,
    pub new_amount: u64,
}

#[event]
pub struct ProposalVaultUpdated {
    pub proposal: Pubkey,
    pub author: Pubkey,
    pub executor: Pubkey,
    pub old_vault: Pubkey,
    pub new_vault: Pubkey,
}
