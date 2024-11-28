use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct UpdateVault<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateVault>, vault: Pubkey) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;

    require!(
        proposal.author == *ctx.accounts.authority.key,
        ErrorMessages::NotAuthorize
    );
    require!(
        proposal.start_at < Clock::get()?.unix_timestamp,
        ErrorMessages::ProposalAlreadyStarted
    );

    let old_vault = proposal.vault.clone();
    proposal.vault = vault;

    msg!("Proposal vault updated successfully");
    msg!("Old vault: {}", old_vault);
    msg!("New vault: {}", proposal.vault);

    emit!(ProposalVaultUpdated {
        proposal: proposal.key(),
        author: proposal.author,
        executor: ctx.accounts.authority.key(),
        old_vault: old_vault,
        new_vault: proposal.vault,
    });

    Ok(())
}
