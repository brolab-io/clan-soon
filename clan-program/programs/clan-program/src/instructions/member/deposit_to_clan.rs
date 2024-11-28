use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DepositToClan<'info> {
    #[account(mut)]
    pub member: Account<'info, Member>,
    #[account(mut)]
    pub clan: Account<'info, Clan>,
    #[account(mut,seeds = [TREASURER_SEED.as_ref(), clan.key().as_ref()], bump)]
    /// CHECK: Just a pure account
    pub treasurer: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DepositToClan>, amount: u64) -> Result<()> {
    require!(amount > 0, ErrorMessages::InvalidAmount);
    let clan = &mut ctx.accounts.clan;
    let member = &mut ctx.accounts.member;

    // check clan account exists
    require!(
        clan.status.eq(&ClanStatus::Active),
        ErrorMessages::ClanInactive
    );

    // check member account exists in clan
    require!(
        !member.status.eq(&MemberStatus::Banned),
        ErrorMessages::MemberIsBanned
    );

    msg!("depositing {} tokens to clan", amount);

    // transfer solana tokens from member to clan
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.authority.to_account_info(),
            to: ctx.accounts.treasurer.to_account_info(),
        },
    );
    system_program::transfer(cpi_context, amount)?;

    // increase member's clan power by amount
    member.power += amount;
    member.status = MemberStatus::Active;

    msg!("member power: {}", member.power);

    clan.power += amount;

    msg!("clan power: {}", clan.power);

    Ok(())
}
