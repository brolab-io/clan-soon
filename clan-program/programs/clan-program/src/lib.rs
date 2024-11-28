pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2GVYqx8UJQLD32m1ntHh8atTTu4itJFRmqazxWpJx2tg");

#[program]
pub mod clan_program {
    use super::*;

    pub fn create_clan(
        ctx: Context<CreateClan>,
        id: u64,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        instructions::clan::create_clan::handler(ctx, id, name, symbol, uri)
    }

    pub fn join_clan(ctx: Context<JoinClan>, clan: Pubkey) -> Result<()> {
        instructions::member::join_clan::handler(ctx, clan)
    }

    pub fn deposit_to_clan(ctx: Context<DepositToClan>, amount: u64) -> Result<()> {
        instructions::member::deposit_to_clan::handler(ctx, amount)
    }

    pub fn leave_clan(ctx: Context<LeaveClan>) -> Result<()> {
        instructions::member::leave_clan::handler(ctx)
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        id: u64,
        title: String,
        description: String,
        start_at: i64,
        end_at: i64,
        amount: u64,
    ) -> Result<()> {
        instructions::proposal::create_proposal::handler(
            ctx,
            id,
            title,
            description,
            start_at,
            end_at,
            amount,
        )
    }

    pub fn update_proposal_vault(ctx: Context<UpdateVault>, vault: Pubkey) -> Result<()> {
        instructions::proposal::update_vault::handler(ctx, vault)
    }

    pub fn update_proposal_amount(ctx: Context<UpdateAmount>, amount: u64) -> Result<()> {
        instructions::proposal::update_amount::handler(ctx, amount)
    }

    pub fn vote(ctx: Context<Vote>, vote: bool) -> Result<()> {
        instructions::member::vote::handler(ctx, vote)
    }

    pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
        instructions::proposal::execute::handler(ctx)
    }

    pub fn claim_vault(ctx: Context<ClaimVault>) -> Result<()> {
        instructions::proposal::claim_vault::handler(ctx)
    }
}
