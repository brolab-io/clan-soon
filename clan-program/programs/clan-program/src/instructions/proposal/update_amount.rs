use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct UpdateAmount<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateAmount>, amount: u64) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;

    require!(
        proposal.author == *ctx.accounts.authority.key,
        ErrorMessages::NotAuthorize
    );
    require!(amount > 0, ErrorMessages::InvalidAmount);
    require!(
        proposal.start_at < Clock::get()?.unix_timestamp,
        ErrorMessages::ProposalAlreadyStarted
    );
    let old_amount = proposal.amount.clone();

    proposal.amount = amount;

    msg!("Proposal amount updated successfully");
    msg!("Old amount: {}", old_amount);
    msg!("New amount: {}", proposal.amount);

    emit!(ProposalAmountUpdated {
        proposal: proposal.key(),
        author: proposal.author,
        executor: ctx.accounts.authority.key(),
        old_amount: old_amount,
        new_amount: proposal.amount,
    });
    Ok(())
}
