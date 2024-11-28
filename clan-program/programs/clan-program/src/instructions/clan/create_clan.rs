use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};


use crate::ClanCreatedEvent;
use crate::{error::ErrorMessages, Clan, ClanStatus, Member, MemberStatus, CARD_SEED, CLAN_SEED, MEMBER_SEED};
#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateClan<'info> {
    #[account(
        init_if_needed,
        seeds = [CLAN_SEED.as_ref(), id.to_le_bytes().as_ref()],
        bump,
        payer = authority,
        space = 8 + Clan::INIT_SPACE
    )]
    pub clan: Account<'info, Clan>,
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
    /// CHECK: We're about to create this with Metaplex
    #[account(
        init_if_needed,
        seeds = [CARD_SEED.as_ref(), clan.key().as_ref(), authority.key.as_ref()],
        bump,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority,
    )]
    pub card: Account<'info, token::Mint>,
    /// CHECK: We're about to create this with Metaplex
    #[account(
        init,
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

pub fn handler(ctx: Context<CreateClan>, id: u64, name: String, symbol: String, uri: String) -> Result<()> {
    let clan = &mut ctx.accounts.clan;
    require!(name.len() <= 32, ErrorMessages::ClanNameTooLong);
    require!(symbol.len() <= 4, ErrorMessages::ClanSymbolTooLong);
    require!(clan.id == 0, ErrorMessages::ClanAlreadyExists);

    msg!("Creating clan...");

    clan.id = id;
    clan.creator = *ctx.accounts.authority.key;
    clan.name = name;
    clan.symbol = symbol;
    clan.uri = uri;
    clan.leader = *ctx.accounts.authority.key;
    clan.status = ClanStatus::Active;
    clan.power = 0;
    clan.created_at = Clock::get()?.unix_timestamp;
    clan.updated_at = Clock::get()?.unix_timestamp;

    msg!("Set creator is first member...");
    let member = &mut ctx.accounts.member;
    member.clan = clan.key();
    member.status = MemberStatus::Active;
    member.power = 0;
    member.timestamp = Clock::get()?.unix_timestamp;
    member.wallet = *ctx.accounts.authority.key;

    clan.member_count = 1;
    

    msg!("Initializing mint NFT card");
    // msg!("Creating mint account...");
    // msg!("Mint: {}", &ctx.accounts.card.key());
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
    // let card_seed: &[&[&[u8]]] = &[&[
    //     CARD_SEED.as_ref(),
    //     &clan.key().to_bytes(),
    //     &ctx.accounts.authority.key().to_bytes(),
    //     &[*ctx
    //         .bumps
    //         .get(std::str::from_utf8(CARD_SEED).unwrap())
    //         .unwrap()],
    // ]];
    // token::initialize_mint(
    //     CpiContext::new_with_signer(
    //         ctx.accounts.token_program.to_account_info(),
    //         token::InitializeMint {
    //             mint: ctx.accounts.card.to_account_info(),
    //             rent: ctx.accounts.rent.to_account_info(),
    //         },
    //         card_seed
    //     ),
    //     0,//zero decimals for the mint
    //     &ctx.accounts.authority.key(),
    //     Some(&ctx.accounts.authority.key()),
    // )?;


    msg!("Creating token account...");
    msg!("Token Address: {}", &ctx.accounts.token_account.key());    
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
        Creator {
            address: clan.key(),
            verified: false,
            share: 100,
        },
        Creator {
            address: ctx.accounts.authority.key(),
            verified: false,
            share: 0,
        },
    ];

    msg!("Creator Assigned to metadata account");


    invoke(
        &instructions::create_metadata_accounts_v3(
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

    msg!("NFT card minted.");

    // Emit event
    emit!(ClanCreatedEvent {
        clan_id: clan.id.clone(),
        name: clan.name.clone(),
        creator: clan.creator.clone(),
        leader: clan.leader.clone(),
        status: clan.status.clone(),
        power: clan.power.clone(),
        symbol: clan.symbol.clone(),
        uri: clan.uri.clone(),
        created_at: clan.created_at.clone(),
        updated_at: clan.updated_at.clone(),
        member_count: clan.member_count.clone(),
    });

    Ok(())
}