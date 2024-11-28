use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimVault<'info> {
    #[account(mut,
        seeds = [
            VAULT_SEED.as_ref(),
            proposal.key().as_ref(),
            authority.key().as_ref(),
        ],
        bump
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<ClaimVault>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let authority = &ctx.accounts.authority;
    let proposal = &mut ctx.accounts.proposal;

    require!(
        proposal.author == authority.key(),
        ErrorMessages::NotAuthorize
    );

    let vault_amount = vault.to_account_info().lamports();
    let proposal_amount = proposal.amount;
    require!(vault_amount >= proposal.amount, ErrorMessages::UserClaimed);
    msg!("Vault balance: {}", vault_amount);
    msg!("Proposal amount: {}", proposal_amount);

    let seeds: &[&[&[u8]]] = &[&[
        VAULT_SEED.as_ref(),
        &proposal.key().to_bytes(),
        &authority.key().to_bytes(),
        &[ctx.bumps.vault],
    ]];

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: vault.to_account_info(),
            to: authority.to_account_info(),
        },
        seeds,
    );
    system_program::transfer(cpi_context, vault_amount)?;
    msg!(
        "Claimed {} from vault to {}",
        vault_amount.to_string(),
        authority.key()
    );

    Ok(())
}
