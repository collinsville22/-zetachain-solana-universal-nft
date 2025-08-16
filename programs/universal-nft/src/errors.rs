use anchor_lang::prelude::*;

#[error_code]
pub enum UniversalNftError {
    #[msg("Unauthorized access - caller is not the authority")]
    Unauthorized,
    
    #[msg("Program is currently paused")]
    ProgramPaused,
    
    #[msg("Invalid gateway authority")]
    InvalidGatewayAuthority,
    
    #[msg("Invalid TSS signature")]
    InvalidTssSignature,
    
    #[msg("Invalid cross-chain message format")]
    InvalidMessageFormat,
    
    #[msg("NFT is currently locked for cross-chain transfer")]
    NftLocked,
    
    #[msg("Invalid chain ID")]
    InvalidChainId,
    
    #[msg("Invalid recipient address")]
    InvalidRecipient,
    
    #[msg("Insufficient gas limit for cross-chain operation")]
    InsufficientGasLimit,
    
    #[msg("Nonce mismatch - potential replay attack")]
    NonceMismatch,
    
    #[msg("Invalid signature recovery")]
    InvalidSignatureRecovery,
    
    #[msg("Transfer already exists")]
    TransferAlreadyExists,
    
    #[msg("Transfer not found")]
    TransferNotFound,
    
    #[msg("Invalid transfer status")]
    InvalidTransferStatus,
    
    #[msg("NFT not found")]
    NftNotFound,
    
    #[msg("Invalid NFT owner")]
    InvalidNftOwner,
    
    #[msg("Collection verification failed")]
    CollectionVerificationFailed,
    
    #[msg("Maximum supply exceeded")]
    MaxSupplyExceeded,
    
    #[msg("Invalid metadata URI")]
    InvalidMetadataUri,
    
    #[msg("Invalid token name")]
    InvalidTokenName,
    
    #[msg("Invalid token symbol")]
    InvalidTokenSymbol,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[msg("Invalid instruction origin")]
    InvalidInstructionOrigin,
    
    #[msg("Cross-chain call origin validation failed")]
    InvalidCallOrigin,
    
    #[msg("Message hash mismatch")]
    MessageHashMismatch,
    
    #[msg("Invalid ECDSA signature format")]
    InvalidEcdsaSignature,
    
    #[msg("Public key recovery failed")]
    PublicKeyRecoveryFailed,
    
    #[msg("Sender verification failed")]
    SenderVerificationFailed,
}