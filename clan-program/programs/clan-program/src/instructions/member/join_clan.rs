use anchor_lang::prelude::*;

use crate::Member;
#[derive(Accounts)]
pub struct JoinClan<'info> {
    #[account(
        init_if_needed,
        seeds = [
            MEMBER_SEED.as_ref(), 
            clan.key().as_ref(), 
            authority.key.as_ref()
            ],
        bump,
        payer = authority,
        space = 8 + Member::INIT_SPACE
    )]
    pub member: Account<'info, Member>,
    #[account(
        mut,
        seeds = [CLAN_SEED.as_ref(), clan.id.to_le_bytes().as_ref()],
        bump
    )]
    pub clan: Account<'info, Clan>,
    /// CHECK: We're about to create this with Metaplex
    #[account(
        init_if_needed,
        seeds = [CARD_SEED.as_ref(), clan.key().as_ref(), authority.key.as_ref()],
        bump,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority
    )]
    pub card: Account<'info, token::Mint>,
    /// CHECK: We're about to create this with Metaplex
    #[account(
        init_if_needed,
        payer = authority,
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
    pub rent: Sysvar<'info, Rent>
}

pub fn handler(ctx: Context<JoinClan>, _clan: Pubkey) -> Result<()> {
    let member = &mut ctx.accounts.member;
    let clan = &mut ctx.accounts.clan;
    msg!("Joining clan : {}",clan.key());

    // check clan account exists
    require!(
        clan.id != 0 &&
        clan.status.eq(&ClanStatus::Active),
        ErrorMessages::ClanInactive
    );

    msg!("Clan is active : {}", clan.key());
    // check if member is already in a clan
    require!(
        !member.clan.eq(&clan.key()),
        ErrorMessages::AlreadyMember
    );

    msg!("Member is not already a member of this clan : {}", clan.key());
    member.clan = clan.key();
    member.wallet = *ctx.accounts.authority.key;
    member.mint_account = ctx.accounts.card.key(); // TODO: Change this to the mint account of the member's token
    member.power = 0;
    member.status = MemberStatus::Inactive;
    member.timestamp = Clock::get()?.unix_timestamp;

    clan.member_count += 1;

    msg!("Member joined clan successfully! :");

    msg!("Initializing mint NFT card");
    msg!("Creating mint account...");
    msg!("Mint: {}", &ctx.accounts.card.key());
    // system_program::create_account(
    //     CpiContext::new(
    //         ctx.accounts.token_program.to_account_info(),
    //         system_program::CreateAccount {
    //             from: ctx.accounts.authority.to_account_info(),
    //             to: ctx.accounts.card.to_account_info(),
    //         },
    //     ),
    //     10000000,//lamports: u64
    //     82,//space: u64 for size
    //     &ctx.accounts.token_program.key(),
    // )?;

    msg!("Initializing mint account...");
    msg!("Mint: {}", &ctx.accounts.card.key());
    // token::initialize_mint(
    //     CpiContext::new(
    //         ctx.accounts.token_program.to_account_info(),
    //         token::InitializeMint {
    //             mint: ctx.accounts.card.to_account_info(),
    //             rent: ctx.accounts.rent.to_account_info(),
    //         },
    //     ),
    //     0,//zero decimals for the mint
    //     &ctx.accounts.authority.key(),
    //     Some(&ctx.accounts.authority.key()),
    // )?;


    msg!("Creating token account...");
    // msg!("Token Address: {}", &ctx.accounts.token_account.key());    
    // associated_token::create(
    //     CpiContext::new(
    //         ctx.accounts.associated_token_program.to_account_info(),
    //         associated_token::Create {
    //             payer: ctx.accounts.authority.to_account_info(),
    //             associated_token: ctx.accounts.token_account.to_account_info(),
    //             authority: ctx.accounts.authority.to_account_info(),
    //             mint: ctx.accounts.card.to_account_info(),
    //             system_program: ctx.accounts.system_program.to_account_info(),
    //             token_program: ctx.accounts.token_program.to_account_info(),
    //         },
    //     ),
    // )?;

    msg!("Minting token to token account...");
    msg!("Mint: {}", &ctx.accounts.card.to_account_info().key());   
    msg!("Token Address: {}", &ctx.accounts.token_account.key());     
    
    let cpi_accounts = token::MintTo {
        mint: ctx.accounts.card.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    token::mint_to(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        1
    )?;
    msg!("Minted NFT card to member");

    msg!("Creating metadata account: {}", &ctx.accounts.metadata.to_account_info().key());

    let account_info = vec![
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.card.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.system_program.to_account_info()
    ];
    // msg!("Account Info for metadata account: {:?}", account_info);

    let creator = vec![
        mpl_token_metadata::state::Creator {
            address: clan.key(),
            verified: false,
            share: 100,
        },
        mpl_token_metadata::state::Creator {
            address: ctx.accounts.authority.key(),
            verified: false,
            share: 0,
        },
    ];

    msg!("Creator Assigned to metadata account");
    invoke(
        &instruction::create_metadata_accounts_v3(
            METADATA_PROGRAM_ID,
            ctx.accounts.metadata.key(),
            ctx.accounts.card.key(),
            ctx.accounts.authority.key(),
            ctx.accounts.authority.key(),
            ctx.accounts.authority.key(),
            clan.name.clone(),
            clan.symbol.clone(),
            clan.uri.clone(),
            Some(creator),
            1,
            true,
            false,
            None, 
            None, 
            None
        )
        , account_info.as_slice()
    )?;
    msg!("Metadata Account Created !!!");
    let master_edition_infos = vec![
        ctx.accounts.master_edition.to_account_info(),
        ctx.accounts.card.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];
    msg!("Master Edition Account Infos Assigned");
    invoke(
        &instruction::create_master_edition_v3(
            METADATA_PROGRAM_ID, 
            ctx.accounts.master_edition.key(), 
            ctx.accounts.card.key(), 
            ctx.accounts.authority.key(), 
            ctx.accounts.authority.key(), 
            ctx.accounts.metadata.key(), 
            ctx.accounts.authority.key(), 
            Some(0),
        ),
        master_edition_infos.as_slice(),
    )?;
    msg!("Token mint process completed successfully.");
    Ok(())
}