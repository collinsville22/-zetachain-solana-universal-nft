use anchor_lang::prelude::*;

/// Program configuration account
#[account]
#[derive(InitSpace)]
pub struct ProgramConfig {
    /// Authority that can update program settings
    pub authority: Pubkey,
    /// ZetaChain gateway authority for cross-chain operations
    pub gateway_authority: Pubkey,
    /// TSS (Threshold Signature Scheme) authority from ZetaChain
    pub tss_authority: Pubkey,
    /// Current nonce for replay protection
    pub nonce: u64,
    /// Program bump seed
    pub bump: u8,
    /// Whether the program is paused
    pub is_paused: bool,
}

/// Universal NFT account storing cross-chain metadata
#[account]
#[derive(InitSpace)]
pub struct UniversalNft {
    /// The mint account for this NFT
    pub mint: Pubkey,
    /// Original chain where this NFT was first minted
    pub origin_chain_id: u64,
    /// Original token ID from the source chain
    pub origin_token_id: String,
    /// Current owner of the NFT
    pub owner: Pubkey,
    /// Metadata URI pointing to JSON metadata
    pub uri: String,
    /// NFT name
    pub name: String,
    /// NFT symbol/collection symbol
    pub symbol: String,
    /// Optional collection mint this NFT belongs to
    pub collection_mint: Option<Pubkey>,
    /// Block number when NFT was created
    pub creation_block: u64,
    /// Timestamp when NFT was created
    pub creation_timestamp: i64,
    /// Bump seed for PDA derivation
    pub bump: u8,
    /// Whether this NFT is currently locked for cross-chain transfer
    pub is_locked: bool,
}

/// Cross-chain transfer state
#[account]
#[derive(InitSpace)]
pub struct CrossChainTransfer {
    /// The NFT being transferred
    pub nft_mint: Pubkey,
    /// Source chain ID
    pub source_chain_id: u64,
    /// Destination chain ID
    pub destination_chain_id: u64,
    /// Sender address on source chain
    pub sender: [u8; 20],
    /// Recipient address on destination chain
    pub recipient: Vec<u8>,
    /// Gas limit for destination transaction
    pub gas_limit: u64,
    /// Transfer nonce for replay protection
    pub nonce: u64,
    /// Timestamp of transfer initiation
    pub timestamp: i64,
    /// Status of the transfer
    pub status: TransferStatus,
    /// Bump seed for PDA derivation
    pub bump: u8,
}

/// Transfer status enumeration
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TransferStatus {
    /// Transfer has been initiated
    Initiated,
    /// Transfer is being processed
    Processing,
    /// Transfer completed successfully
    Completed,
    /// Transfer failed and was reverted
    Reverted,
    /// Transfer was cancelled
    Cancelled,
}

/// Cross-chain message types
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum CrossChainMessage {
    /// Mint NFT on destination chain
    MintNft {
        token_id: String,
        name: String,
        symbol: String,
        uri: String,
        recipient: Pubkey,
        collection_mint: Option<Pubkey>,
    },
    /// Burn NFT and return to source chain
    BurnNft {
        token_id: String,
        owner: Pubkey,
    },
    /// Transfer ownership
    TransferOwnership {
        token_id: String,
        new_owner: Pubkey,
    },
    /// Update metadata
    UpdateMetadata {
        token_id: String,
        new_uri: String,
        new_name: Option<String>,
        new_symbol: Option<String>,
    },
}

/// Collection information for universal NFTs
#[account]
#[derive(InitSpace)]
pub struct UniversalCollection {
    /// Collection mint account
    pub mint: Pubkey,
    /// Collection authority
    pub authority: Pubkey,
    /// Collection name
    pub name: String,
    /// Collection symbol
    pub symbol: String,
    /// Collection URI for metadata
    pub uri: String,
    /// Total supply of NFTs in this collection
    pub total_supply: u64,
    /// Maximum supply (0 for unlimited)
    pub max_supply: u64,
    /// Whether this collection is verified
    pub is_verified: bool,
    /// Bump seed for PDA derivation
    pub bump: u8,
}

impl ProgramConfig {
    pub const INIT_SPACE: usize = 
        32 + // authority
        32 + // gateway_authority
        32 + // tss_authority
        8 +  // nonce
        1 +  // bump
        1;   // is_paused
}

impl UniversalNft {
    pub const INIT_SPACE: usize = 
        32 + // mint
        8 +  // origin_chain_id
        4 + 64 + // origin_token_id (String with max 64 chars)
        32 + // owner
        4 + 200 + // uri (String with max 200 chars)
        4 + 32 + // name (String with max 32 chars)
        4 + 16 + // symbol (String with max 16 chars)
        1 + 32 + // collection_mint (Option<Pubkey>)
        8 +  // creation_block
        8 +  // creation_timestamp
        1 +  // bump
        1;   // is_locked
}

impl CrossChainTransfer {
    pub const INIT_SPACE: usize = 
        32 + // nft_mint
        8 +  // source_chain_id
        8 +  // destination_chain_id
        20 + // sender
        4 + 64 + // recipient (Vec<u8> with max 64 bytes)
        8 +  // gas_limit
        8 +  // nonce
        8 +  // timestamp
        1 +  // status (enum discriminator)
        1;   // bump
}

impl UniversalCollection {
    pub const INIT_SPACE: usize = 
        32 + // mint
        32 + // authority
        4 + 64 + // name (String with max 64 chars)
        4 + 16 + // symbol (String with max 16 chars)
        4 + 200 + // uri (String with max 200 chars)
        8 +  // total_supply
        8 +  // max_supply
        1 +  // is_verified
        1;   // bump
}