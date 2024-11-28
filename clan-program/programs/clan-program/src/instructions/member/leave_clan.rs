use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct LeaveClan<'info> {
    #[account(
        mut,
        seeds = [
            MEMBER_SEED.as_ref(), 
            clan.key().as_ref(), 
            authority.key.as_ref()
            ],
        bump
    )]
    pub member: Account<'info, Member>,
    #[account(
        mut,
        seeds = [CLAN_SEED.as_ref(), clan.id.to_le_bytes().as_ref()],
        bump
    )]
    pub clan: Account<'info, Clan>,
    #[account(mut, seeds = [TREASURER_SEED.as_ref(), clan.key().as_ref()], bump)]
    /// CHECK: Just a pure account
    pub treasurer: AccountInfo<'info>,
    /// CHECK: We're about to create this with Metaplex
    #[account(
        mut,
        seeds = [CARD_SEED.as_ref(), clan.key().as_ref(), authority.key.as_ref()],
        bump,
        // mint::decimals = 0,
        // mint::authority = authority,
        // mint::freeze_authority = authority,
    )]
    pub card: Account<'info, token::Mint>,
    /// CHECK: We're about to create this with Metaplex
    #[account(
        mut,
        associated_token::mint = card,
        associated_token::authority = authority,
    )]
    pub token_account: Account<'info, token::TokenAccount>,
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    /// CHECK: We're about to create this with Metaplex
    pub token_metadata_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<LeaveClan>) -> Result<()> {
    let member = &mut ctx.accounts.member;
    let clan = &mut ctx.accounts.clan;
    let token_account = &mut ctx.accounts.token_account;
    let master_edition = &mut ctx.accounts.master_edition;
    let card = &mut ctx.accounts.card;

    require!(member.clan == clan.key(), ErrorMessages::NotMember);

    require!(
        member.status != MemberStatus::Banned,
        ErrorMessages::MemberIsBanned
    );
    require!(
        ctx.accounts.authority.key() == member.wallet,
        ErrorMessages::NotAuthorize,
    );

    require!(clan.leader != member.wallet, ErrorMessages::LeaderCannotLeave);

    //  check nft card is owned by member
    require!(
        token_account.owner == ctx.accounts.authority.key()
            && token_account.mint == member.mint_account.key()
            && token_account.amount == 1,
        ErrorMessages::InvalidTokenAccount
    );

    // check master editon is owned by member
    let me_seed = &[
        "metadata".as_bytes(),
        METADATA_PROGRAM_ID.as_ref(),
        member.mint_account.as_ref(),
        "edition".as_bytes(),
    ];

    let (master_edition_key, _) = Pubkey::find_program_address(me_seed, ctx.accounts.token_metadata_program.key);

    require!(
        master_edition_key == master_edition.key() && !master_edition.data_is_empty(),
        ErrorMessages::InvalidMasterEdition
    );

    // TODO: check metadata
    // https://medium.com/@Arrivant_/how-to-verify-nfts-in-an-anchor-program-a051299acde8

    msg!("Master edition key: {}", master_edition_key);

    msg!("Burn card token NFT ...");

    let burn_acounts = vec![
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        card.to_account_info(),
        token_account.to_account_info(),
        master_edition.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
    ];

    invoke(
        &token_metadata_instruction::burn_nft(
            METADATA_PROGRAM_ID,
            ctx.accounts.metadata.key(),
            ctx.accounts.authority.key(),
            member.mint_account.key(),
            token_account.key(),
            master_edition.key(),
            ctx.accounts.token_program.key(),
            None,
        ),
        burn_acounts.as_slice(),
    )?;

    msg!("Burn card token NFT done");

    msg!("Refund process started");
    let member_power = member.power;

    if member_power > 0 {
        msg!("Refund amount: {}", member_power);
        // refund the member's power
        let seeds: &[&[&[u8]]] = &[&[
            TREASURER_SEED.as_ref(),
            &clan.key().to_bytes(),
            &[ctx
                .bumps.treasurer],
        ]];

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.treasurer.to_account_info(),
                to: ctx.accounts.authority.to_account_info(),
            },
            seeds,
        );
        system_program::transfer(cpi_context, member_power)?;

        msg!("Refund done");
        msg!("Refund amount: {} from {} to {}", member_power, ctx.accounts.treasurer.key(), ctx.accounts.authority.key());
    }
    clan.member_count -= 1;
    msg!("Member clan updated: {}", clan.member_count);

    // member.power = 0;
    clan.power -= member_power;
    msg!("Clan power updated: {}", clan.power);

    msg!("Member account closing ...");
    member.close(ctx.accounts.authority.to_account_info())?;

    Ok(())
}