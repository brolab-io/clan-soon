use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateProposal<'info> {
    #[account(
        init_if_needed,
        seeds = [
            PROPOSAL_SEED.as_ref(), 
            id.to_le_bytes().as_ref(),
            clan.key().as_ref(),
            authority.key().as_ref()
            ],
        bump,
        payer = authority,
        space = Proposal::LEN
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(
        mut,
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
    pub clan: Account<'info, Clan>,
    #[account(mut)]
    pub member: Account<'info, Member>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<CreateProposal>,
    id: u64,
    title: String,
    description: String,
    start_at: i64,
    end_at: i64,
    amount: u64
) -> Result<()> {
    msg!("Creating proposal...");
    let proposal = &mut ctx.accounts.proposal;
    let clan = &mut ctx.accounts.clan;
    let authority = &mut ctx.accounts.authority;
    let member = &mut ctx.accounts.member;
    let vault= &mut ctx.accounts.vault;

    require!(
        start_at < end_at,
        ErrorMessages::ProposalStartAfterEnd);

    require!(
        start_at > Clock::get()?.unix_timestamp,
        ErrorMessages::ProposalStartInPast);
    
    require!(amount > 0, ErrorMessages::ProposalAmountZero);

    msg!("Checking proposal...");
    require!(title.len() <= MAX_TITLE_LENGTH, ErrorMessages::TitleTooLong);
    require!(
        description.len() <= MAX_DESC_LENGTH,
        ErrorMessages::DescriptionTooLong
    );

    require!(member.clan.eq(&clan.key()), ErrorMessages::NotMember);
    require!(
        member.status.eq(&MemberStatus::Active),
        ErrorMessages::MemberInactiveOrBanned
    );
    msg!("Checking proposal...done");

    proposal.id = id;
    proposal.clan = clan.key();
    proposal.author = authority.key();
    proposal.amount = amount;
    proposal.vault = vault.key();
    proposal.executed = false;
    proposal.executed_by = None;
    proposal.votes = 0;
    proposal.yes_votes = 0;
    proposal.no_votes = 0;
    proposal.title = title;
    proposal.description = description;
    proposal.start_at = start_at;
    proposal.end_at = end_at;
    proposal.created_at = Clock::get()?.unix_timestamp;
    proposal.executed_at = None;

    msg!("Proposal vault: {}", vault.key());
    Ok(())
}