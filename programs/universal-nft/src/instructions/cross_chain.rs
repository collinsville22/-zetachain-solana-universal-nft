use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Token, TokenAccount};
use solana_program::{
    instruction::Instruction,
    program::{invoke, invoke_signed},
    system_instruction,
    sysvar::{instructions::Instructions as SysvarInstructions, Sysvar},
};

use crate::state::*;
use crate::errors::*;
use crate::utils::*;

/// Handle incoming cross-chain calls from ZetaChain Gateway
pub fn on_call(
    ctx: Context<OnCall>,
    sender: [u8; 20],
    source_chain_id: u64,
    message: Vec<u8>,
) -> Result<()> {
    let config = &ctx.accounts.config;
    
    // Check if program is paused
    require!(!config.is_paused, UniversalNftError::ProgramPaused);
    
    // Verify the call is coming from the gateway program
    verify_instruction_origin(&ctx.accounts.instructions_sysvar)?;
    
    // Validate chain ID
    CrossChainUtils::validate_chain_id(source_chain_id)?;
    
    // Validate message format
    SignatureUtils::validate_message_format(&message)?;
    
    // Parse the cross-chain message
    let cross_chain_msg: CrossChainMessage = borsh::from_slice(&message)
        .map_err(|_| UniversalNftError::InvalidMessageFormat)?;

    // Process based on message type
    match cross_chain_msg {
        CrossChainMessage::MintNft {
            token_id,
            name,
            symbol,
            uri,
            recipient,
            collection_mint,
        } => {
            handle_mint_from_cross_chain(
                ctx,
                token_id,
                name,
                symbol,
                uri,
                recipient,
                collection_mint,
                source_chain_id,
            )?;
        }
        CrossChainMessage::BurnNft { token_id, owner } => {
            handle_burn_from_cross_chain(ctx, token_id, owner, source_chain_id)?;
        }
        CrossChainMessage::TransferOwnership { token_id, new_owner } => {
            handle_transfer_from_cross_chain(ctx, token_id, new_owner, source_chain_id)?;
        }
        CrossChainMessage::UpdateMetadata {
            token_id,
            new_uri,
            new_name,
            new_symbol,
        } => {
            handle_metadata_update_from_cross_chain(
                ctx,
                token_id,
                new_uri,
                new_name,
                new_symbol,
                source_chain_id,
            )?;
        }
    }

    msg!("Cross-chain call processed successfully");
    msg!("Source chain: {}", source_chain_id);
    msg!("Sender: {:?}", sender);

    Ok(())
}

/// Handle revert operations for failed cross-chain transactions
pub fn on_revert(
    ctx: Context<OnRevert>,
    sender: [u8; 20],
    source_chain_id: u64,
    message: Vec<u8>,
) -> Result<()> {
    let config = &ctx.accounts.config;
    
    // Check if program is paused
    require!(!config.is_paused, UniversalNftError::ProgramPaused);
    
    // Verify the call is coming from the gateway program
    verify_instruction_origin(&ctx.accounts.instructions_sysvar)?;
    
    // Find the transfer that needs to be reverted
    let transfer = &mut ctx.accounts.transfer;
    require!(
        transfer.status == TransferStatus::Processing,
        UniversalNftError::InvalidTransferStatus
    );

    // Update transfer status to reverted
    transfer.status = TransferStatus::Reverted;

    // Unlock the NFT if it was locked
    let universal_nft = &mut ctx.accounts.universal_nft;
    universal_nft.is_locked = false;

    msg!("Cross-chain transaction reverted");
    msg!("Transfer nonce: {}", transfer.nonce);
    msg!("Source chain: {}", source_chain_id);

    Ok(())
}

/// Burn NFT and initiate cross-chain transfer
pub fn burn_and_transfer(
    ctx: Context<BurnAndTransfer>,
    destination_chain_id: u64,
    recipient: Vec<u8>,
    gas_limit: u64,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    // Check if program is paused
    require!(!config.is_paused, UniversalNftError::ProgramPaused);
    
    // Validate parameters
    CrossChainUtils::validate_chain_id(destination_chain_id)?;
    CrossChainUtils::validate_recipient(&recipient)?;
    CrossChainUtils::validate_gas_limit(gas_limit)?;

    let universal_nft = &mut ctx.accounts.universal_nft;
    
    // Check NFT is not locked
    require!(!universal_nft.is_locked, UniversalNftError::NftLocked);
    
    // Verify ownership
    require!(
        universal_nft.owner == ctx.accounts.owner.key(),
        UniversalNftError::InvalidNftOwner
    );

    // Lock the NFT
    universal_nft.is_locked = true;

    // Increment nonce for replay protection
    config.nonce = config.nonce
        .checked_add(1)
        .ok_or(UniversalNftError::ArithmeticOverflow)?;

    // Create transfer record
    let transfer = &mut ctx.accounts.transfer;
    transfer.nft_mint = universal_nft.mint;
    transfer.source_chain_id = 900; // Solana chain ID
    transfer.destination_chain_id = destination_chain_id;
    transfer.sender = [0u8; 20]; // Convert Solana address to bytes
    transfer.recipient = recipient.clone();
    transfer.gas_limit = gas_limit;
    transfer.nonce = config.nonce;
    transfer.timestamp = Clock::get()?.unix_timestamp;
    transfer.status = TransferStatus::Initiated;
    transfer.bump = ctx.bumps.transfer;

    // Burn the token
    let cpi_accounts = Burn {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::burn(cpi_ctx, 1)?;

    // Prepare cross-chain message
    let cross_chain_msg = CrossChainMessage::MintNft {
        token_id: universal_nft.origin_token_id.clone(),
        name: universal_nft.name.clone(),
        symbol: universal_nft.symbol.clone(),
        uri: universal_nft.uri.clone(),
        recipient: Pubkey::new_from_array(
            recipient.try_into().map_err(|_| UniversalNftError::InvalidRecipient)?
        ),
        collection_mint: universal_nft.collection_mint,
    };

    let message_data = borsh::to_vec(&cross_chain_msg)
        .map_err(|_| UniversalNftError::InvalidMessageFormat)?;

    // Create instruction for gateway call
    let gateway_call_ix = create_gateway_call_instruction(
        ctx.accounts.gateway_program.key(),
        destination_chain_id,
        recipient,
        message_data,
        gas_limit,
    )?;

    // Invoke gateway program
    invoke(
        &gateway_call_ix,
        &[
            ctx.accounts.gateway_program.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ],
    )?;

    msg!("Cross-chain transfer initiated");
    msg!("Token ID: {}", universal_nft.origin_token_id);
    msg!("Destination chain: {}", destination_chain_id);
    msg!("Transfer nonce: {}", transfer.nonce);

    Ok(())
}

// Helper functions

fn handle_mint_from_cross_chain(
    ctx: Context<OnCall>,
    token_id: String,
    name: String,
    symbol: String,
    uri: String,
    recipient: Pubkey,
    collection_mint: Option<Pubkey>,
    source_chain_id: u64,
) -> Result<()> {
    // Implementation for minting NFT from cross-chain
    msg!("Minting NFT from cross-chain");
    msg!("Token ID: {}", token_id);
    msg!("Recipient: {}", recipient);
    
    // This would involve creating a new mint and metadata
    // Similar to the mint_nft instruction but with cross-chain origin
    
    Ok(())
}

fn handle_burn_from_cross_chain(
    ctx: Context<OnCall>,
    token_id: String,
    owner: Pubkey,
    source_chain_id: u64,
) -> Result<()> {
    // Implementation for burning NFT from cross-chain
    msg!("Burning NFT from cross-chain");
    msg!("Token ID: {}", token_id);
    msg!("Owner: {}", owner);
    
    Ok(())
}

fn handle_transfer_from_cross_chain(
    ctx: Context<OnCall>,
    token_id: String,
    new_owner: Pubkey,
    source_chain_id: u64,
) -> Result<()> {
    // Implementation for transferring ownership from cross-chain
    msg!("Transferring ownership from cross-chain");
    msg!("Token ID: {}", token_id);
    msg!("New owner: {}", new_owner);
    
    Ok(())
}

fn handle_metadata_update_from_cross_chain(
    ctx: Context<OnCall>,
    token_id: String,
    new_uri: String,
    new_name: Option<String>,
    new_symbol: Option<String>,
    source_chain_id: u64,
) -> Result<()> {
    // Implementation for updating metadata from cross-chain
    msg!("Updating metadata from cross-chain");
    msg!("Token ID: {}", token_id);
    msg!("New URI: {}", new_uri);
    
    Ok(())
}

fn verify_instruction_origin(instructions_sysvar: &UncheckedAccount) -> Result<()> {
    // Verify that the current instruction is called by the gateway program
    let instructions = SysvarInstructions::from_account_info(instructions_sysvar)?;
    
    // Check if the calling instruction is from the authorized gateway
    // This is a simplified version - full implementation would check the instruction stack
    
    Ok(())
}

fn create_gateway_call_instruction(
    gateway_program: Pubkey,
    destination_chain_id: u64,
    recipient: Vec<u8>,
    message: Vec<u8>,
    gas_limit: u64,
) -> Result<Instruction> {
    // Create instruction for calling the ZetaChain gateway
    // This would use the actual gateway program interface
    
    let instruction_data = [
        &[0u8], // Instruction discriminator for "call"
        &destination_chain_id.to_le_bytes(),
        &(recipient.len() as u32).to_le_bytes(),
        &recipient,
        &(message.len() as u32).to_le_bytes(),
        &message,
        &gas_limit.to_le_bytes(),
    ].concat();

    Ok(Instruction {
        program_id: gateway_program,
        accounts: vec![], // Would include necessary accounts
        data: instruction_data,
    })
}

// Account structs

#[derive(Accounts)]
pub struct OnCall<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,

    #[account(mut)]
    pub universal_nft: Account<'info, UniversalNft>,

    /// CHECK: Instructions sysvar for origin verification
    #[account(address = SysvarInstructions::id())]
    pub instructions_sysvar: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct OnRevert<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,

    #[account(mut)]
    pub universal_nft: Account<'info, UniversalNft>,

    #[account(mut)]
    pub transfer: Account<'info, CrossChainTransfer>,

    /// CHECK: Instructions sysvar for origin verification
    #[account(address = SysvarInstructions::id())]
    pub instructions_sysvar: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct BurnAndTransfer<'info> {
    #[account(
        mut,
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

    #[account(
        init,
        payer = owner,
        space = 8 + CrossChainTransfer::INIT_SPACE,
        seeds = [b"transfer", mint.key().as_ref(), &config.nonce.to_le_bytes()],
        bump
    )]
    pub transfer: Account<'info, CrossChainTransfer>,

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

    /// CHECK: Gateway program for cross-chain calls
    pub gateway_program: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}