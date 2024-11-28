use anchor_lang::prelude::*;

use crate::{error::ErrorMessages, Clan, Member, MemberStatus, Proposal, BALLOT_SEED};

#[derive(Accounts)]
#[instruction(vote: bool)]
pub struct Vote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut,
        constraint = proposal.clan == clan.key()
    )]
    pub clan: Account<'info, Clan>,
    #[account(mut,
        constraint = member.clan == clan.key(),
        constraint = member.status.eq(&MemberStatus::Active),
        constraint = member.power > 0,
        constraint = member.wallet == authority.key()
    )]
    pub member: Account<'info, Member>,
    #[account(
        init_if_needed,
        seeds = [
            BALLOT_SEED.as_ref(),
            proposal.key().as_ref(),
            member.key().as_ref(),
            authority.key().as_ref()
        ],
        bump,
        payer = authority,
        space = Ballot::LEN
    )]
    pub ballot: Account<'info, Ballot>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<Vote>, vote: bool) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let member = &mut ctx.accounts.member;
    let ballot = &mut ctx.accounts.ballot;

    require!(
        proposal.start_at.le(&Clock::get()?.unix_timestamp),
        ErrorMessages::ProposalNotStarted
    );
    require!(
        proposal.end_at.gt(&Clock::get()?.unix_timestamp),
        ErrorMessages::ProposalAlreadyEnded
    );
    // require!(
    //     proposal.author != *ctx.accounts.authority.key,
    //     ErrorMessages::NotAuthorize
    // );

    require!(
        member.status.eq(&MemberStatus::Active) && member.power > 0,
        ErrorMessages::MemberInactive
    );

    proposal.votes += member.power;
    msg!("Proposal votes updated {}", proposal.votes);
    ballot.amount = member.power;
    ballot.vote = vote;
    ballot.created_at = Clock::get()?.unix_timestamp;
    ballot.updated_at = Clock::get()?.unix_timestamp;
    ballot.member = member.key();
    ballot.proposal = proposal.key();

    if vote == true {
        proposal.yes_votes += member.power;
    } else {
        proposal.no_votes += member.power;
    }

    msg!("Proposal voted successfully");
    msg!(
        "Member {} vote {} for proposal {}",
        member.key(),
        vote.to_string(),
        proposal.key()
    );
    msg!("Proposal yes votes {}", proposal.yes_votes);
    msg!("Proposal no votes {}", proposal.no_votes);

    Ok(())
}
