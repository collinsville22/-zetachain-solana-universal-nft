use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use crate::utils::*;

/// Verify cross-chain signature from ZetaChain TSS
pub fn verify_signature(
    ctx: Context<VerifySignature>,
    message_hash: [u8; 32],
    signature: [u8; 64],
    recovery_id: u8,
) -> Result<()> {
    let config = &ctx.accounts.config;
    
    // Check if program is paused
    require!(!config.is_paused, UniversalNftError::ProgramPaused);
    
    // Ensure TSS authority is set
    require!(
        config.tss_authority != Pubkey::default(),
        UniversalNftError::InvalidTssSignature
    );

    // Convert TSS authority to Ethereum address format
    let tss_eth_address = pubkey_to_eth_address(&config.tss_authority);

    // Verify the signature
    let is_valid = SignatureUtils::verify_ecdsa_signature(
        &message_hash,
        &signature,
        recovery_id,
        &tss_eth_address,
    )?;

    require!(is_valid, UniversalNftError::InvalidTssSignature);

    msg!("TSS signature verified successfully");
    msg!("Message hash: {:?}", message_hash);
    msg!("TSS authority: {}", config.tss_authority);

    Ok(())
}

/// Verify a cross-chain message with nonce validation
pub fn verify_cross_chain_message(
    ctx: Context<VerifyCrossChainMessage>,
    nonce: u64,
    chain_id: u64,
    recipient: Vec<u8>,
    amount: u64,
    data: Vec<u8>,
    signature: [u8; 64],
    recovery_id: u8,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    // Check if program is paused
    require!(!config.is_paused, UniversalNftError::ProgramPaused);
    
    // Validate nonce to prevent replay attacks
    require!(nonce > config.nonce, UniversalNftError::NonceMismatch);
    
    // Validate chain ID
    CrossChainUtils::validate_chain_id(chain_id)?;
    
    // Validate recipient
    CrossChainUtils::validate_recipient(&recipient)?;

    // Hash the message components
    let message_hash = SignatureUtils::hash_message(
        nonce,
        chain_id,
        &recipient,
        amount,
        &data,
    );

    // Convert TSS authority to Ethereum address format
    let tss_eth_address = pubkey_to_eth_address(&config.tss_authority);

    // Verify the signature
    let is_valid = SignatureUtils::verify_ecdsa_signature(
        &message_hash,
        &signature,
        recovery_id,
        &tss_eth_address,
    )?;

    require!(is_valid, UniversalNftError::InvalidTssSignature);

    // Update nonce to prevent replay
    config.nonce = nonce;

    msg!("Cross-chain message verified successfully");
    msg!("Nonce: {}", nonce);
    msg!("Chain ID: {}", chain_id);
    msg!("Amount: {}", amount);

    Ok(())
}

/// Batch verify multiple signatures for efficiency
pub fn batch_verify_signatures(
    ctx: Context<BatchVerifySignatures>,
    messages: Vec<[u8; 32]>,
    signatures: Vec<[u8; 64]>,
    recovery_ids: Vec<u8>,
) -> Result<()> {
    let config = &ctx.accounts.config;
    
    // Check if program is paused
    require!(!config.is_paused, UniversalNftError::ProgramPaused);
    
    // Validate input lengths match
    require!(
        messages.len() == signatures.len() && 
        messages.len() == recovery_ids.len(),
        UniversalNftError::InvalidMessageFormat
    );

    // Limit batch size for compute budget management
    require!(messages.len() <= 10, UniversalNftError::InvalidMessageFormat);

    // Convert TSS authority to Ethereum address format
    let tss_eth_address = pubkey_to_eth_address(&config.tss_authority);

    // Verify each signature
    for (i, ((message_hash, signature), recovery_id)) in messages
        .iter()
        .zip(signatures.iter())
        .zip(recovery_ids.iter())
        .enumerate()
    {
        let is_valid = SignatureUtils::verify_ecdsa_signature(
            message_hash,
            signature,
            *recovery_id,
            &tss_eth_address,
        )?;

        require!(is_valid, UniversalNftError::InvalidTssSignature);
        
        msg!("Signature {} verified", i);
    }

    msg!("Batch signature verification completed");
    msg!("Verified {} signatures", messages.len());

    Ok(())
}

/// Recover public key from signature for testing/debugging
pub fn recover_public_key(
    ctx: Context<RecoverPublicKey>,
    message_hash: [u8; 32],
    signature: [u8; 64],
    recovery_id: u8,
) -> Result<()> {
    let config = &ctx.accounts.config;
    
    // Check if program is paused (allow if authority for debugging)
    if config.is_paused {
        require!(
            ctx.accounts.authority.key() == config.authority,
            UniversalNftError::ProgramPaused
        );
    }

    // Recover public key using Solana's secp256k1_recover
    let recovered_pubkey = solana_program::secp256k1_recover::secp256k1_recover(
        &message_hash,
        recovery_id,
        &signature,
    )
    .map_err(|_| UniversalNftError::PublicKeyRecoveryFailed)?;

    // Convert to Ethereum address
    let ethereum_address = SignatureUtils::pubkey_to_ethereum_address(&recovered_pubkey.to_bytes());

    msg!("Public key recovery successful");
    msg!("Message hash: {:?}", message_hash);
    msg!("Recovered Ethereum address: {:?}", ethereum_address);
    msg!("Recovery ID: {}", recovery_id);

    Ok(())
}

// Helper function to convert Solana pubkey to Ethereum address
fn pubkey_to_eth_address(pubkey: &Pubkey) -> [u8; 20] {
    // This is a simplified conversion - in practice, you'd need to handle
    // the proper conversion between Solana and Ethereum address formats
    let pubkey_bytes = pubkey.to_bytes();
    let mut eth_address = [0u8; 20];
    eth_address.copy_from_slice(&pubkey_bytes[..20]);
    eth_address
}

// Account validation structures

#[derive(Accounts)]
pub struct VerifySignature<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,
}

#[derive(Accounts)]
pub struct VerifyCrossChainMessage<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,
}

#[derive(Accounts)]
pub struct BatchVerifySignatures<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,
}

#[derive(Accounts)]
pub struct RecoverPublicKey<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,
    
    /// Only required when program is paused
    pub authority: Option<Signer<'info>>,
}

/// Verify sender authority for cross-chain operations
pub fn verify_sender_authority(
    ctx: Context<VerifySenderAuthority>,
    sender_address: [u8; 20],
    message_hash: [u8; 32],
    signature: [u8; 64],
    recovery_id: u8,
) -> Result<()> {
    let config = &ctx.accounts.config;
    
    // Check if program is paused
    require!(!config.is_paused, UniversalNftError::ProgramPaused);

    // Verify the signature matches the claimed sender
    let is_valid = SignatureUtils::verify_ecdsa_signature(
        &message_hash,
        &signature,
        recovery_id,
        &sender_address,
    )?;

    require!(is_valid, UniversalNftError::SenderVerificationFailed);

    msg!("Sender authority verified");
    msg!("Sender address: {:?}", sender_address);

    Ok(())
}

#[derive(Accounts)]
pub struct VerifySenderAuthority<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::hash::hash;

    #[test]
    fn test_pubkey_to_eth_address() {
        let pubkey = Pubkey::new_unique();
        let eth_address = pubkey_to_eth_address(&pubkey);
        assert_eq!(eth_address.len(), 20);
    }

    #[test] 
    fn test_message_hash() {
        let nonce = 1;
        let chain_id = 7000;
        let recipient = vec![1u8; 20];
        let amount = 100;
        let data = vec![2u8; 32];
        
        let hash1 = SignatureUtils::hash_message(nonce, chain_id, &recipient, amount, &data);
        let hash2 = SignatureUtils::hash_message(nonce, chain_id, &recipient, amount, &data);
        
        assert_eq!(hash1, hash2); // Same inputs should produce same hash
        
        let hash3 = SignatureUtils::hash_message(nonce + 1, chain_id, &recipient, amount, &data);
        assert_ne!(hash1, hash3); // Different nonce should produce different hash
    }
}