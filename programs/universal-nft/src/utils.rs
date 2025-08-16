use anchor_lang::prelude::*;
use solana_program::{
    keccak,
    program_error::ProgramError,
    secp256k1_recover::{secp256k1_recover, Secp256k1Pubkey},
};
use libsecp256k1::{PublicKey, SecretKey, Message, sign, verify};
use sha2::{Sha256, Digest};
use crate::errors::UniversalNftError;

/// Utilities for signature verification and cross-chain operations
pub struct SignatureUtils;

impl SignatureUtils {
    /// Verify ECDSA signature using secp256k1 curve (Ethereum-compatible)
    pub fn verify_ecdsa_signature(
        message_hash: &[u8; 32],
        signature: &[u8; 64],
        recovery_id: u8,
        expected_signer: &[u8; 20],
    ) -> Result<bool> {
        // Recover the public key from the signature
        let recovered_pubkey = secp256k1_recover(message_hash, recovery_id, signature)
            .map_err(|_| UniversalNftError::PublicKeyRecoveryFailed)?;

        // Convert public key to Ethereum address
        let ethereum_address = Self::pubkey_to_ethereum_address(&recovered_pubkey.to_bytes());
        
        // Compare with expected signer
        Ok(ethereum_address == *expected_signer)
    }

    /// Convert secp256k1 public key to Ethereum address
    pub fn pubkey_to_ethereum_address(pubkey: &[u8; 64]) -> [u8; 20] {
        let hash = keccak::hash(pubkey);
        let mut address = [0u8; 20];
        address.copy_from_slice(&hash.to_bytes()[12..]);
        address
    }

    /// Generate deterministic token ID from mint, block, and timestamp
    pub fn generate_token_id(
        mint: &Pubkey,
        block_number: u64,
        timestamp: i64,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(mint.to_bytes());
        hasher.update(block_number.to_le_bytes());
        hasher.update(timestamp.to_le_bytes());
        let result = hasher.finalize();
        bs58::encode(result).into_string()
    }

    /// Validate cross-chain message format
    pub fn validate_message_format(message: &[u8]) -> Result<bool> {
        // Basic validation - message should be non-empty and within size limits
        if message.is_empty() || message.len() > 1024 {
            return Err(UniversalNftError::InvalidMessageFormat.into());
        }
        Ok(true)
    }

    /// Hash message for signature verification
    pub fn hash_message(
        nonce: u64,
        chain_id: u64,
        recipient: &[u8],
        amount: u64,
        data: &[u8],
    ) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(nonce.to_le_bytes());
        hasher.update(chain_id.to_le_bytes());
        hasher.update(recipient);
        hasher.update(amount.to_le_bytes());
        hasher.update(data);
        let result = hasher.finalize();
        result.into()
    }
}

/// Utilities for cross-chain operations
pub struct CrossChainUtils;

impl CrossChainUtils {
    /// Validate chain ID
    pub fn validate_chain_id(chain_id: u64) -> Result<bool> {
        // Define supported chain IDs
        const SUPPORTED_CHAINS: &[u64] = &[
            7000, // ZetaChain Mainnet
            7001, // ZetaChain Testnet
            1,    // Ethereum Mainnet
            5,    // Ethereum Goerli
            56,   // BSC Mainnet
            97,   // BSC Testnet
        ];

        if SUPPORTED_CHAINS.contains(&chain_id) {
            Ok(true)
        } else {
            Err(UniversalNftError::InvalidChainId.into())
        }
    }

    /// Validate recipient address format
    pub fn validate_recipient(recipient: &[u8]) -> Result<bool> {
        // Ethereum-style addresses should be 20 bytes
        // Solana addresses are 32 bytes
        // Allow both formats
        match recipient.len() {
            20 | 32 => Ok(true),
            _ => Err(UniversalNftError::InvalidRecipient.into()),
        }
    }

    /// Validate gas limit for cross-chain operations
    pub fn validate_gas_limit(gas_limit: u64) -> Result<bool> {
        const MIN_GAS_LIMIT: u64 = 21000;
        const MAX_GAS_LIMIT: u64 = 10_000_000;

        if gas_limit >= MIN_GAS_LIMIT && gas_limit <= MAX_GAS_LIMIT {
            Ok(true)
        } else {
            Err(UniversalNftError::InsufficientGasLimit.into())
        }
    }
}

/// Utilities for NFT metadata validation
pub struct MetadataUtils;

impl MetadataUtils {
    /// Validate NFT name
    pub fn validate_name(name: &str) -> Result<bool> {
        if name.is_empty() || name.len() > 32 {
            return Err(UniversalNftError::InvalidTokenName.into());
        }
        // Check for valid UTF-8 and printable characters
        if !name.chars().all(|c| c.is_ascii_graphic() || c == ' ') {
            return Err(UniversalNftError::InvalidTokenName.into());
        }
        Ok(true)
    }

    /// Validate NFT symbol
    pub fn validate_symbol(symbol: &str) -> Result<bool> {
        if symbol.is_empty() || symbol.len() > 16 {
            return Err(UniversalNftError::InvalidTokenSymbol.into());
        }
        // Symbol should be alphanumeric
        if !symbol.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(UniversalNftError::InvalidTokenSymbol.into());
        }
        Ok(true)
    }

    /// Validate metadata URI
    pub fn validate_uri(uri: &str) -> Result<bool> {
        if uri.is_empty() || uri.len() > 200 {
            return Err(UniversalNftError::InvalidMetadataUri.into());
        }
        // Basic URI validation - should start with http/https/ipfs/ar
        let valid_prefixes = ["http://", "https://", "ipfs://", "ar://"];
        if !valid_prefixes.iter().any(|prefix| uri.starts_with(prefix)) {
            return Err(UniversalNftError::InvalidMetadataUri.into());
        }
        Ok(true)
    }
}

/// Compute budget utilities for Solana optimization
pub struct ComputeUtils;

impl ComputeUtils {
    /// Calculate required compute units for NFT operations
    pub fn calculate_compute_units(operation_type: OperationType) -> u32 {
        match operation_type {
            OperationType::MintNft => 50_000,
            OperationType::TransferNft => 20_000,
            OperationType::BurnNft => 30_000,
            OperationType::CrossChainCall => 100_000,
            OperationType::VerifySignature => 40_000,
            OperationType::UpdateMetadata => 25_000,
        }
    }

    /// Check if sufficient compute budget is available
    pub fn check_compute_budget() -> Result<bool> {
        // This would integrate with Solana's compute budget in a real implementation
        // For now, we'll assume sufficient budget is available
        Ok(true)
    }
}

#[derive(Clone, Copy)]
pub enum OperationType {
    MintNft,
    TransferNft,
    BurnNft,
    CrossChainCall,
    VerifySignature,
    UpdateMetadata,
}

/// Rent calculation utilities
pub struct RentUtils;

impl RentUtils {
    /// Calculate rent exemption for account size
    pub fn calculate_rent_exemption(data_len: usize) -> u64 {
        // This is a simplified calculation
        // In practice, you'd use Rent::minimum_balance
        const LAMPORTS_PER_BYTE: u64 = 6960;
        (data_len as u64) * LAMPORTS_PER_BYTE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_token_id() {
        let mint = Pubkey::new_unique();
        let block_number = 12345;
        let timestamp = 1640995200; // Example timestamp
        
        let token_id = SignatureUtils::generate_token_id(&mint, block_number, timestamp);
        assert!(!token_id.is_empty());
        assert!(token_id.len() > 10);
    }

    #[test]
    fn test_validate_name() {
        assert!(MetadataUtils::validate_name("My NFT").is_ok());
        assert!(MetadataUtils::validate_name("").is_err());
        assert!(MetadataUtils::validate_name(&"a".repeat(50)).is_err());
    }

    #[test]
    fn test_validate_symbol() {
        assert!(MetadataUtils::validate_symbol("NFT").is_ok());
        assert!(MetadataUtils::validate_symbol("").is_err());
        assert!(MetadataUtils::validate_symbol("NFT!@#").is_err());
    }

    #[test]
    fn test_validate_uri() {
        assert!(MetadataUtils::validate_uri("https://example.com/metadata.json").is_ok());
        assert!(MetadataUtils::validate_uri("ipfs://QmHash").is_ok());
        assert!(MetadataUtils::validate_uri("invalid://uri").is_err());
        assert!(MetadataUtils::validate_uri("").is_err());
    }
}