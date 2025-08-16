use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;
use crate::errors::*;

/// Transfer NFT to another address on Solana
pub fn transfer_nft(ctx: Context<TransferNft>) -> Result<()> {
    let config = &ctx.accounts.config;
    
    // Check if program is paused
    require!(!config.is_paused, UniversalNftError::ProgramPaused);
    
    let universal_nft = &mut ctx.accounts.universal_nft;
    
    // Check NFT is not locked for cross-chain transfer
    require!(!universal_nft.is_locked, UniversalNftError::NftLocked);
    
    // Verify current ownership
    require!(
        universal_nft.owner == ctx.accounts.current_owner.key(),
        UniversalNftError::InvalidNftOwner
    );

    // Perform the token transfer
    let cpi_accounts = Transfer {
        from: ctx.accounts.from_token_account.to_account_info(),
        to: ctx.accounts.to_token_account.to_account_info(),
        authority: ctx.accounts.current_owner.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, 1)?;

    // Update ownership in Universal NFT account
    universal_nft.owner = ctx.accounts.new_owner.key();

    msg!("NFT transferred successfully");
    msg!("Token ID: {}", universal_nft.origin_token_id);
    msg!("From: {}", ctx.accounts.current_owner.key());
    msg!("To: {}", ctx.accounts.new_owner.key());

    Ok(())
}

#[derive(Accounts)]
pub struct TransferNft<'info> {
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
        mut,
        associated_token::mint = mint,
        associated_token::authority = current_owner,
    )]
    pub from_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = current_owner,
        associated_token::mint = mint,
        associated_token::authority = new_owner,
    )]
    pub to_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub current_owner: Signer<'info>,

    /// CHECK: New owner account
    pub new_owner: SystemAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub system_program: Program<'info, System>,
}

/// Approve another account to transfer the NFT
pub fn approve_transfer(ctx: Context<ApproveTransfer>) -> Result<()> {
    let config = &ctx.accounts.config;
    
    // Check if program is paused
    require!(!config.is_paused, UniversalNftError::ProgramPaused);
    
    let universal_nft = &ctx.accounts.universal_nft;
    
    // Check NFT is not locked
    require!(!universal_nft.is_locked, UniversalNftError::NftLocked);
    
    // Verify ownership
    require!(
        universal_nft.owner == ctx.accounts.owner.key(),
        UniversalNftError::InvalidNftOwner
    );

    // Approve the delegate
    let cpi_accounts = anchor_spl::token::Approve {
        to: ctx.accounts.token_account.to_account_info(),
        delegate: ctx.accounts.delegate.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    anchor_spl::token::approve(cpi_ctx, 1)?;

    msg!("Transfer approval granted");
    msg!("Token ID: {}", universal_nft.origin_token_id);
    msg!("Delegate: {}", ctx.accounts.delegate.key());

    Ok(())
}

#[derive(Accounts)]
pub struct ApproveTransfer<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,

    #[account(
        seeds = [b"universal_nft", mint.key().as_ref()],
        bump = universal_nft.bump
    )]
    pub universal_nft: Account<'info, UniversalNft>,

    #[account(mut)]
    pub mint: Account<'info, anchor_spl::token::Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: Delegate account to approve
    pub delegate: SystemAccount<'info>,

    pub token_program: Program<'info, Token>,
}

/// Transfer NFT using delegate authority
pub fn transfer_from(ctx: Context<TransferFrom>) -> Result<()> {
    let config = &ctx.accounts.config;
    
    // Check if program is paused
    require!(!config.is_paused, UniversalNftError::ProgramPaused);
    
    let universal_nft = &mut ctx.accounts.universal_nft;
    
    // Check NFT is not locked
    require!(!universal_nft.is_locked, UniversalNftError::NftLocked);

    // Transfer using delegate authority
    let cpi_accounts = Transfer {
        from: ctx.accounts.from_token_account.to_account_info(),
        to: ctx.accounts.to_token_account.to_account_info(),
        authority: ctx.accounts.delegate.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, 1)?;

    // Update ownership in Universal NFT account
    universal_nft.owner = ctx.accounts.new_owner.key();

    msg!("NFT transferred by delegate");
    msg!("Token ID: {}", universal_nft.origin_token_id);
    msg!("Delegate: {}", ctx.accounts.delegate.key());
    msg!("To: {}", ctx.accounts.new_owner.key());

    Ok(())
}

#[derive(Accounts)]
pub struct TransferFrom<'info> {
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
        mut,
        associated_token::mint = mint,
        associated_token::authority = current_owner,
    )]
    pub from_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = delegate,
        associated_token::mint = mint,
        associated_token::authority = new_owner,
    )]
    pub to_token_account: Account<'info, TokenAccount>,

    /// CHECK: Current owner (not signer since delegate is transferring)
    pub current_owner: SystemAccount<'info>,

    /// CHECK: New owner account
    pub new_owner: SystemAccount<'info>,

    #[account(mut)]
    pub delegate: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub system_program: Program<'info, System>,
}

/// Revoke transfer approval
pub fn revoke_approval(ctx: Context<RevokeApproval>) -> Result<()> {
    let config = &ctx.accounts.config;
    
    // Check if program is paused
    require!(!config.is_paused, UniversalNftError::ProgramPaused);
    
    let universal_nft = &ctx.accounts.universal_nft;
    
    // Verify ownership
    require!(
        universal_nft.owner == ctx.accounts.owner.key(),
        UniversalNftError::InvalidNftOwner
    );

    // Revoke the approval
    let cpi_accounts = anchor_spl::token::Revoke {
        source: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    anchor_spl::token::revoke(cpi_ctx)?;

    msg!("Transfer approval revoked");
    msg!("Token ID: {}", universal_nft.origin_token_id);

    Ok(())
}

#[derive(Accounts)]
pub struct RevokeApproval<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,

    #[account(
        seeds = [b"universal_nft", mint.key().as_ref()],
        bump = universal_nft.bump
    )]
    pub universal_nft: Account<'info, UniversalNft>,

    #[account(mut)]
    pub mint: Account<'info, anchor_spl::token::Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub token_program: Program<'info, Token>,
}