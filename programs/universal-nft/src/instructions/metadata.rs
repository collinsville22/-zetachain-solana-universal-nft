use anchor_lang::prelude::*;
use mpl_token_metadata::{
    accounts::Metadata,
    instructions::UpdateMetadataAccountV2,
    types::DataV2,
};
use solana_program::program::invoke_signed;

use crate::state::*;
use crate::errors::*;
use crate::utils::*;

/// Update NFT metadata (owner only)
pub fn update_metadata(
    ctx: Context<UpdateMetadata>,
    new_uri: String,
    new_name: Option<String>,
    new_symbol: Option<String>,
) -> Result<()> {
    let config = &ctx.accounts.config;
    
    // Check if program is paused
    require!(!config.is_paused, UniversalNftError::ProgramPaused);
    
    let universal_nft = &mut ctx.accounts.universal_nft;
    
    // Check NFT is not locked
    require!(!universal_nft.is_locked, UniversalNftError::NftLocked);
    
    // Verify ownership
    require!(
        universal_nft.owner == ctx.accounts.owner.key(),
        UniversalNftError::InvalidNftOwner
    );

    // Validate new metadata
    MetadataUtils::validate_uri(&new_uri)?;
    
    if let Some(ref name) = new_name {
        MetadataUtils::validate_name(name)?;
    }
    
    if let Some(ref symbol) = new_symbol {
        MetadataUtils::validate_symbol(symbol)?;
    }

    // Update Universal NFT account
    universal_nft.uri = new_uri.clone();
    
    if let Some(name) = new_name.clone() {
        universal_nft.name = name;
    }
    
    if let Some(symbol) = new_symbol.clone() {
        universal_nft.symbol = symbol;
    }

    // Update on-chain metadata
    let metadata_seeds = &[
        b"universal_nft",
        ctx.accounts.mint.key().as_ref(),
        &[universal_nft.bump],
    ];
    let signer_seeds = &[&metadata_seeds[..]];

    // Prepare updated metadata
    let data = DataV2 {
        name: universal_nft.name.clone(),
        symbol: universal_nft.symbol.clone(),
        uri: new_uri,
        seller_fee_basis_points: 0,
        creators: None, // Keep existing creators
        collection: universal_nft.collection_mint.map(|mint| {
            mpl_token_metadata::types::Collection {
                verified: false,
                key: mint,
            }
        }),
        uses: None,
    };

    // Create update metadata instruction
    let update_metadata_ix = UpdateMetadataAccountV2 {
        metadata: ctx.accounts.metadata.key(),
        update_authority: ctx.accounts.update_authority.key(),
    };

    invoke_signed(
        &update_metadata_ix.instruction(mpl_token_metadata::types::UpdateMetadataAccountArgsV2 {
            data: Some(data),
            update_authority: Some(ctx.accounts.update_authority.key()),
            primary_sale_happened: None,
            is_mutable: Some(true),
        }),
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.update_authority.to_account_info(),
        ],
        signer_seeds,
    )?;

    msg!("NFT metadata updated successfully");
    msg!("Token ID: {}", universal_nft.origin_token_id);
    msg!("New URI: {}", universal_nft.uri);

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateMetadata<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,

    #[account(
        mut,
        seeds = [b"universal_nft", mint.key().as_ref()],
        bump = universal_nft.bump
    )]
    pub universal_nft: Account<'info, UniversalNft>,

    #[account(mut)]
    pub mint: Account<'info, anchor_spl::token::Mint>,

    /// CHECK: Metadata account validated by seeds
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

    /// CHECK: Update authority PDA
    #[account(
        seeds = [b"universal_nft", mint.key().as_ref()],
        bump = universal_nft.bump
    )]
    pub update_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

/// Verify collection membership for an NFT
pub fn verify_collection(ctx: Context<VerifyCollection>) -> Result<()> {
    let config = &ctx.accounts.config;
    
    // Check if program is paused
    require!(!config.is_paused, UniversalNftError::ProgramPaused);
    
    let universal_nft = &mut ctx.accounts.universal_nft;
    let collection = &ctx.accounts.collection;
    
    // Verify collection authority
    require!(
        collection.authority == ctx.accounts.collection_authority.key(),
        UniversalNftError::Unauthorized
    );

    // Update NFT to reference the collection
    universal_nft.collection_mint = Some(collection.mint);

    msg!("Collection verified for NFT");
    msg!("Token ID: {}", universal_nft.origin_token_id);
    msg!("Collection: {}", collection.mint);

    Ok(())
}

#[derive(Accounts)]
pub struct VerifyCollection<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,

    #[account(
        mut,
        seeds = [b"universal_nft", mint.key().as_ref()],
        bump = universal_nft.bump
    )]
    pub universal_nft: Account<'info, UniversalNft>,

    #[account(mut)]
    pub mint: Account<'info, anchor_spl::token::Mint>,

    #[account(
        seeds = [b"collection", collection_mint.key().as_ref()],
        bump = collection.bump
    )]
    pub collection: Account<'info, UniversalCollection>,

    pub collection_mint: Account<'info, anchor_spl::token::Mint>,

    pub collection_authority: Signer<'info>,
}

/// Create a new universal collection
pub fn create_collection(
    ctx: Context<CreateCollection>,
    name: String,
    symbol: String,
    uri: String,
    max_supply: u64,
) -> Result<()> {
    let config = &ctx.accounts.config;
    
    // Check if program is paused
    require!(!config.is_paused, UniversalNftError::ProgramPaused);
    
    // Validate metadata
    MetadataUtils::validate_name(&name)?;
    MetadataUtils::validate_symbol(&symbol)?;
    MetadataUtils::validate_uri(&uri)?;

    // Initialize collection
    let collection = &mut ctx.accounts.collection;
    collection.mint = ctx.accounts.mint.key();
    collection.authority = ctx.accounts.authority.key();
    collection.name = name.clone();
    collection.symbol = symbol.clone();
    collection.uri = uri.clone();
    collection.total_supply = 0;
    collection.max_supply = max_supply;
    collection.is_verified = true;
    collection.bump = ctx.bumps.collection;

    // Mint collection token
    let cpi_accounts = anchor_spl::token::MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    anchor_spl::token::mint_to(cpi_ctx, 1)?;

    // Create collection metadata
    let metadata_seeds = &[
        b"collection",
        ctx.accounts.mint.key().as_ref(),
        &[collection.bump],
    ];
    let signer_seeds = &[&metadata_seeds[..]];

    let data = DataV2 {
        name: name.clone(),
        symbol: symbol.clone(),
        uri: uri.clone(),
        seller_fee_basis_points: 0,
        creators: Some(vec![mpl_token_metadata::types::CreatorV2 {
            address: ctx.accounts.authority.key(),
            verified: true,
            share: 100,
        }]),
        collection: None,
        uses: None,
    };

    // Create metadata account instruction
    let create_metadata_ix = mpl_token_metadata::instructions::CreateMetadataAccountV3 {
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
            collection_details: Some(mpl_token_metadata::types::CollectionDetailsValue::V1 {
                size: max_supply,
            }),
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

    msg!("Universal collection created successfully");
    msg!("Collection: {}", ctx.accounts.mint.key());
    msg!("Name: {}", name);
    msg!("Max Supply: {}", max_supply);

    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateCollection<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,

    #[account(
        init,
        payer = payer,
        space = 8 + UniversalCollection::INIT_SPACE,
        seeds = [b"collection", mint.key().as_ref()],
        bump
    )]
    pub collection: Account<'info, UniversalCollection>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub mint: Account<'info, anchor_spl::token::Mint>,

    /// CHECK: Metadata account validated by seeds
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

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub token_account: Account<'info, anchor_spl::token::TokenAccount>,

    /// CHECK: PDA authority for collection
    #[account(
        seeds = [b"collection", mint.key().as_ref()],
        bump
    )]
    pub mint_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: SystemAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub system_program: Program<'info, System>,
    /// CHECK: Rent sysvar
    pub rent: UncheckedAccount<'info>,
}