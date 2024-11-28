use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut,
        constraint = proposal.clan == member.clan,
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut,
        constraint = member.status.eq(&MemberStatus::Active),
        constraint = member.power > 0,
        constraint = member.clan == clan.key(),
    )]
    pub member: Account<'info, Member>,
    #[account(mut,
        constraint = clan.key() == proposal.clan,
    )]
    pub clan: Account<'info, Clan>,
    #[account(mut,
        seeds = [TREASURER_SEED.as_ref(), clan.key().as_ref()],
        bump
    )]
    /// CHECK: Just a pure account
    pub treasurer: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [
            VAULT_SEED.as_ref(),
            proposal.key().as_ref(),
            proposal.author.as_ref(),
        ],
        bump,
    )]
    /// CHECK: Just a pure account
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<ExecuteProposal>) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    // let member = &mut ctx.accounts.member;
    let clan = &mut ctx.accounts.clan;

    require!(
        proposal.end_at > Clock::get()?.unix_timestamp,
        ErrorMessages::ProposalNotEnded
    );

    // TODO: just simple check, should do more and more ...
    require!(
        proposal.yes_votes > proposal.no_votes,
        ErrorMessages::ProposalNotPassed
    );

    require!(
        proposal.executed == false,
        ErrorMessages::ProposalAlreadyExecuted
    );

    require!(clan.power > proposal.amount, ErrorMessages::NotEnoughPower);

    msg!("Executing proposal...");

    let seeds: &[&[&[u8]]] = &[&[
        TREASURER_SEED.as_ref(),
        &clan.key().to_bytes(),
        &[ctx.bumps.treasurer],
    ]];

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.treasurer.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        },
        seeds,
    );
    system_program::transfer(cpi_context, proposal.amount)?;
    msg!(
        "Transferred {} to vault {}",
        proposal.amount,
        ctx.accounts.vault.key()
    );

    proposal.executed = true;
    proposal.executed_at = Some(Clock::get()?.unix_timestamp);
    proposal.executed_by = Some(ctx.accounts.authority.key());

    msg!(
        "Proposal executed successfully by {} at {}",
        ctx.accounts.authority.key(),
        Clock::get()?.unix_timestamp
    );
    Ok(())
}
