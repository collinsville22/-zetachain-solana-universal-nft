use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};
use anchor_spl::associated_token::AssociatedToken;
use mpl_token_metadata::{
    accounts::{Metadata, MasterEdition},
    instructions::{CreateMetadataAccountV3, CreateMasterEditionV3},
    types::{DataV2, CreatorV2, CollectionDetails},
};
use solana_program::{
    program::invoke_signed,
    system_instruction,
};

use crate::state::*;
use crate::errors::*;
use crate::utils::*;

pub fn mint_nft(
    ctx: Context<MintNft>,
    name: String,
    symbol: String,
    uri: String,
    collection_mint: Option<Pubkey>,
) -> Result<()> {
    let config = &ctx.accounts.config;
    
    // Check if program is paused
    require!(!config.is_paused, UniversalNftError::ProgramPaused);
    
    // Validate metadata
    MetadataUtils::validate_name(&name)?;
    MetadataUtils::validate_symbol(&symbol)?;
    MetadataUtils::validate_uri(&uri)?;

    // Get current slot and timestamp for token ID generation
    let clock = Clock::get()?;
    let slot = clock.slot;
    let timestamp = clock.unix_timestamp;

    // Generate unique token ID
    let token_id = SignatureUtils::generate_token_id(
        &ctx.accounts.mint.key(),
        slot,
        timestamp,
    );

    // Initialize Universal NFT account
    let universal_nft = &mut ctx.accounts.universal_nft;
    universal_nft.mint = ctx.accounts.mint.key();
    universal_nft.origin_chain_id = 900; // Solana chain ID (custom)
    universal_nft.origin_token_id = token_id.clone();
    universal_nft.owner = ctx.accounts.owner.key();
    universal_nft.uri = uri.clone();
    universal_nft.name = name.clone();
    universal_nft.symbol = symbol.clone();
    universal_nft.collection_mint = collection_mint;
    universal_nft.creation_block = slot;
    universal_nft.creation_timestamp = timestamp;
    universal_nft.bump = ctx.bumps.universal_nft;
    universal_nft.is_locked = false;

    // Mint token to owner
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::mint_to(cpi_ctx, 1)?;

    // Create metadata account
    let metadata_seeds = &[
        b"universal_nft",
        ctx.accounts.mint.key().as_ref(),
        &[universal_nft.bump],
    ];
    let signer_seeds = &[&metadata_seeds[..]];

    // Prepare metadata
    let data = DataV2 {
        name: name.clone(),
        symbol: symbol.clone(),
        uri: uri.clone(),
        seller_fee_basis_points: 0,
        creators: Some(vec![CreatorV2 {
            address: ctx.accounts.owner.key(),
            verified: true,
            share: 100,
        }]),
        collection: collection_mint.map(|mint| mpl_token_metadata::types::Collection {
            verified: false,
            key: mint,
        }),
        uses: None,
    };

    // Create metadata account instruction
    let create_metadata_ix = CreateMetadataAccountV3 {
        metadata: ctx.accounts.metadata.key(),
        mint: ctx.accounts.mint.key(),
        mint_authority: ctx.accounts.mint_authority.key(),
        payer: ctx.accounts.payer.key(),
        update_authority: ctx.accounts.mint_authority.key(),
        system_program: ctx.accounts.system_program.key(),
        rent: ctx.accounts.rent.key(),
    };

    invoke_signed(
        &create_metadata_ix.instruction(mpl_token_metadata::types::CreateMetadataAccountArgsV3 {
            data,
            is_mutable: true,
            collection_details: None,
        }),
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        signer_seeds,
    )?;

    // Create master edition for unique NFT
    let create_master_edition_ix = CreateMasterEditionV3 {
        edition: ctx.accounts.master_edition.key(),
        mint: ctx.accounts.mint.key(),
        update_authority: ctx.accounts.mint_authority.key(),
        mint_authority: ctx.accounts.mint_authority.key(),
        payer: ctx.accounts.payer.key(),
        metadata: ctx.accounts.metadata.key(),
        token_program: ctx.accounts.token_program.key(),
        system_program: ctx.accounts.system_program.key(),
        rent: ctx.accounts.rent.key(),
    };

    invoke_signed(
        &create_master_edition_ix.instruction(mpl_token_metadata::types::CreateMasterEditionArgs {
            max_supply: Some(0), // Unique NFT
        }),
        &[
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        signer_seeds,
    )?;

    msg!("Universal NFT minted successfully");
    msg!("Token ID: {}", token_id);
    msg!("Mint: {}", ctx.accounts.mint.key());
    msg!("Owner: {}", ctx.accounts.owner.key());

    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct MintNft<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,

    #[account(
        init,
        payer = payer,
        space = 8 + UniversalNft::INIT_SPACE,
        seeds = [b"universal_nft", mint.key().as_ref()],
        bump
    )]
    pub universal_nft: Account<'info, UniversalNft>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,

    /// CHECK: This account will be validated by the metadata program
    #[account(
        mut,
        seeds = [
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = mpl_token_metadata::ID,
    )]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: This account will be validated by the metadata program
    #[account(
        mut,
        seeds = [
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            mint.key().as_ref(),
            b"edition",
        ],
        bump,
        seeds::program = mpl_token_metadata::ID,
    )]
    pub master_edition: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    pub token_account: Account<'info, TokenAccount>,

    /// CHECK: PDA authority for minting
    #[account(
        seeds = [b"universal_nft", mint.key().as_ref()],
        bump
    )]
    pub mint_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub owner: SystemAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    /// CHECK: Rent sysvar
    pub rent: UncheckedAccount<'info>,
}