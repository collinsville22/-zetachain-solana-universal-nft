use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use mpl_token_metadata::instructions::{CreateMetadataAccountV3, UpdateMetadataAccountV2};
use mpl_token_metadata::types::{DataV2, CreatorV2, CollectionDetails};
use solana_program::{
    system_instruction,
    sysvar::{Sysvar, SysvarId},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    instruction::Instruction,
};

declare_id!("EiGgwyFXtqcNEutPaUe94J9c9sPaPnDWj64sFcD7W9sz");

pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use errors::*;
use instructions::*;
use state::*;
use utils::*;

#[program]
pub mod universal_nft {
    use super::*;

    /// Initialize the Universal NFT program
    pub fn initialize(ctx: Context<Initialize>, gateway_authority: Pubkey) -> Result<()> {
        instructions::initialize(ctx, gateway_authority)
    }

    /// Mint a new universal NFT with Solana compute optimization
    pub fn mint_nft(
        ctx: Context<MintNft>,
        name: String,
        symbol: String,
        uri: String,
        collection_mint: Option<Pubkey>,
    ) -> Result<()> {
        // Solana compute budget optimization - rent exemption handled in instructions
        instructions::mint_nft(ctx, name, symbol, uri, collection_mint)
    }

    /// Handle incoming cross-chain calls from ZetaChain Gateway
    pub fn on_call(
        ctx: Context<OnCall>,
        sender: [u8; 20],
        source_chain_id: u64,
        message: Vec<u8>,
    ) -> Result<()> {
        instructions::on_call(ctx, sender, source_chain_id, message)
    }

    /// Handle revert operations for failed cross-chain transactions
    pub fn on_revert(
        ctx: Context<OnRevert>,
        sender: [u8; 20],
        source_chain_id: u64,
        message: Vec<u8>,
    ) -> Result<()> {
        instructions::on_revert(ctx, sender, source_chain_id, message)
    }

    /// Burn NFT and initiate cross-chain transfer
    pub fn burn_and_transfer(
        ctx: Context<BurnAndTransfer>,
        destination_chain_id: u64,
        recipient: Vec<u8>,
        gas_limit: u64,
    ) -> Result<()> {
        instructions::burn_and_transfer(ctx, destination_chain_id, recipient, gas_limit)
    }

    /// Transfer NFT to another address on Solana
    pub fn transfer_nft(ctx: Context<TransferNft>) -> Result<()> {
        instructions::transfer_nft(ctx)
    }

    /// Update NFT metadata (owner only)
    pub fn update_metadata(
        ctx: Context<UpdateMetadata>,
        new_uri: String,
        new_name: Option<String>,
        new_symbol: Option<String>,
    ) -> Result<()> {
        instructions::update_metadata(ctx, new_uri, new_name, new_symbol)
    }

    /// Verify cross-chain signature from ZetaChain TSS
    pub fn verify_signature(
        ctx: Context<VerifySignature>,
        message_hash: [u8; 32],
        signature: [u8; 64],
        recovery_id: u8,
    ) -> Result<()> {
        instructions::verify_signature(ctx, message_hash, signature, recovery_id)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + ProgramConfig::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, ProgramConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}