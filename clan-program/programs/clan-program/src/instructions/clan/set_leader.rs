use anchor_lang::prelude::*;

use crate::Clan;

#[derive(Accounts)]
pub struct SetLeader<'info> {
    #[account(mut)]
    pub clan: Account<'info, Clan>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<SetLeader>, new_leader: Pubkey) -> Result<()> {
    msg!("Setting new leader...");

    require!(
        ctx.accounts.clan.creator == *ctx.accounts.authority.key
            || ctx.accounts.clan.leader == *ctx.accounts.authority.key,
        ErrorMessages::NotAuthorize
    );

    ctx.accounts.clan.leader = new_leader;
    msg!("Leader updated successfully to: {}", new_leader);
    Ok(())
}
