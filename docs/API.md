# API Reference

## Program Instructions

### Core Instructions

#### `initialize`
Initialize the Universal NFT program with configuration.

```rust
pub fn initialize(
    ctx: Context<Initialize>, 
    gateway_authority: Pubkey
) -> Result<()>
```

**Parameters:**
- `gateway_authority`: The authorized ZetaChain gateway program

**Accounts:**
- `config` (init): Program configuration PDA
- `authority` (signer): Program authority
- `system_program`: Solana system program

**Example:**
```typescript
await program.methods
  .initialize(gatewayAuthority)
  .accounts({
    config: configPda,
    authority: authority.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .signers([authority])
  .rpc();
```

#### `mint_nft`
Mint a new universal NFT with cross-chain compatibility.

```rust
pub fn mint_nft(
    ctx: Context<MintNft>,
    name: String,
    symbol: String,
    uri: String,
    collection_mint: Option<Pubkey>,
) -> Result<()>
```

**Parameters:**
- `name`: NFT name (max 32 characters)
- `symbol`: NFT symbol (max 16 characters)
- `uri`: Metadata URI (max 200 characters)
- `collection_mint`: Optional collection this NFT belongs to

**Validation:**
- Name: ASCII printable characters only
- Symbol: Alphanumeric characters only
- URI: Must start with http://, https://, ipfs://, or ar://

**Example:**
```typescript
await program.methods
  .mintNft("My Universal NFT", "MUNFT", "https://example.com/metadata.json", null)
  .accounts({
    config: configPda,
    universalNft: universalNftPda,
    mint: mint.publicKey,
    // ... other accounts
  })
  .signers([authority, mint])
  .rpc();
```

### Cross-Chain Instructions

#### `on_call`
Handle incoming cross-chain calls from ZetaChain Gateway.

```rust
pub fn on_call(
    ctx: Context<OnCall>,
    sender: [u8; 20],
    source_chain_id: u64,
    message: Vec<u8>,
) -> Result<()>
```

**Parameters:**
- `sender`: Ethereum-style address of sender (20 bytes)
- `source_chain_id`: Chain ID where message originated
- `message`: Serialized cross-chain message

**Message Types:**
```rust
pub enum CrossChainMessage {
    MintNft {
        token_id: String,
        name: String,
        symbol: String,
        uri: String,
        recipient: Pubkey,
        collection_mint: Option<Pubkey>,
    },
    BurnNft {
        token_id: String,
        owner: Pubkey,
    },
    TransferOwnership {
        token_id: String,
        new_owner: Pubkey,
    },
    UpdateMetadata {
        token_id: String,
        new_uri: String,
        new_name: Option<String>,
        new_symbol: Option<String>,
    },
}
```

#### `burn_and_transfer`
Burn NFT on Solana and initiate cross-chain transfer.

```rust
pub fn burn_and_transfer(
    ctx: Context<BurnAndTransfer>,
    destination_chain_id: u64,
    recipient: Vec<u8>,
    gas_limit: u64,
) -> Result<()>
```

**Parameters:**
- `destination_chain_id`: Target blockchain ID
- `recipient`: Recipient address on destination chain
- `gas_limit`: Gas limit for destination transaction (21,000 - 10,000,000)

**Supported Chain IDs:**
- `7000`: ZetaChain Mainnet
- `7001`: ZetaChain Testnet
- `1`: Ethereum Mainnet
- `5`: Ethereum Goerli
- `56`: BSC Mainnet
- `97`: BSC Testnet

#### `on_revert`
Handle revert operations for failed cross-chain transactions.

```rust
pub fn on_revert(
    ctx: Context<OnRevert>,
    sender: [u8; 20],
    source_chain_id: u64,
    message: Vec<u8>,
) -> Result<()>
```

### Transfer Instructions

#### `transfer_nft`
Transfer NFT to another address on Solana.

```rust
pub fn transfer_nft(ctx: Context<TransferNft>) -> Result<()>
```

**Requirements:**
- Current owner must sign transaction
- NFT must not be locked for cross-chain transfer
- Destination token account created automatically

#### `approve_transfer`
Approve another account to transfer the NFT.

```rust
pub fn approve_transfer(ctx: Context<ApproveTransfer>) -> Result<()>
```

#### `transfer_from`
Transfer NFT using delegate authority.

```rust
pub fn transfer_from(ctx: Context<TransferFrom>) -> Result<()>
```

### Metadata Instructions

#### `update_metadata`
Update NFT metadata (owner only).

```rust
pub fn update_metadata(
    ctx: Context<UpdateMetadata>,
    new_uri: String,
    new_name: Option<String>,
    new_symbol: Option<String>,
) -> Result<()>
```

**Requirements:**
- Only NFT owner can update
- NFT must not be locked
- New values must pass validation

#### `create_collection`
Create a new universal collection.

```rust
pub fn create_collection(
    ctx: Context<CreateCollection>,
    name: String,
    symbol: String,
    uri: String,
    max_supply: u64,
) -> Result<()>
```

### Security Instructions

#### `verify_signature`
Verify ECDSA signature from ZetaChain TSS.

```rust
pub fn verify_signature(
    ctx: Context<VerifySignature>,
    message_hash: [u8; 32],
    signature: [u8; 64],
    recovery_id: u8,
) -> Result<()>
```

#### `verify_cross_chain_message`
Verify cross-chain message with nonce validation.

```rust
pub fn verify_cross_chain_message(
    ctx: Context<VerifyCrossChainMessage>,
    nonce: u64,
    chain_id: u64,
    recipient: Vec<u8>,
    amount: u64,
    data: Vec<u8>,
    signature: [u8; 64],
    recovery_id: u8,
) -> Result<()>
```

## Account Structures

### ProgramConfig
```rust
pub struct ProgramConfig {
    pub authority: Pubkey,           // 32 bytes
    pub gateway_authority: Pubkey,   // 32 bytes
    pub tss_authority: Pubkey,       // 32 bytes
    pub nonce: u64,                  // 8 bytes
    pub bump: u8,                    // 1 byte
    pub is_paused: bool,             // 1 byte
}
```

### UniversalNft
```rust
pub struct UniversalNft {
    pub mint: Pubkey,                    // 32 bytes
    pub origin_chain_id: u64,            // 8 bytes
    pub origin_token_id: String,         // 4 + 64 bytes
    pub owner: Pubkey,                   // 32 bytes
    pub uri: String,                     // 4 + 200 bytes
    pub name: String,                    // 4 + 32 bytes
    pub symbol: String,                  // 4 + 16 bytes
    pub collection_mint: Option<Pubkey>, // 1 + 32 bytes
    pub creation_block: u64,             // 8 bytes
    pub creation_timestamp: i64,         // 8 bytes
    pub bump: u8,                        // 1 byte
    pub is_locked: bool,                 // 1 byte
}
```

### CrossChainTransfer
```rust
pub struct CrossChainTransfer {
    pub nft_mint: Pubkey,               // 32 bytes
    pub source_chain_id: u64,           // 8 bytes
    pub destination_chain_id: u64,      // 8 bytes
    pub sender: [u8; 20],               // 20 bytes
    pub recipient: Vec<u8>,             // 4 + 64 bytes
    pub gas_limit: u64,                 // 8 bytes
    pub nonce: u64,                     // 8 bytes
    pub timestamp: i64,                 // 8 bytes
    pub status: TransferStatus,         // 1 byte
    pub bump: u8,                       // 1 byte
}
```

## Error Codes

### Common Errors
- `Unauthorized`: Caller lacks required permissions
- `ProgramPaused`: Program is currently paused
- `InvalidGatewayAuthority`: Invalid gateway authority provided
- `InvalidTssSignature`: TSS signature verification failed
- `NftLocked`: NFT is locked for cross-chain transfer

### Validation Errors
- `InvalidMessageFormat`: Cross-chain message format invalid
- `InvalidChainId`: Unsupported or invalid chain ID
- `InvalidRecipient`: Invalid recipient address format
- `InvalidMetadataUri`: Metadata URI format invalid
- `InvalidTokenName`: Token name validation failed

### Security Errors
- `NonceMismatch`: Nonce validation failed (replay protection)
- `InvalidSignatureRecovery`: Signature recovery failed
- `SenderVerificationFailed`: Cross-chain sender verification failed
- `InvalidCallOrigin`: Call origin validation failed

## Gas and Compute Limits

### Compute Units by Operation
- `mint_nft`: ~50,000 CU
- `transfer_nft`: ~20,000 CU
- `burn_and_transfer`: ~30,000 CU
- `on_call`: ~100,000 CU
- `verify_signature`: ~40,000 CU
- `update_metadata`: ~25,000 CU

### Account Rent Requirements
- `ProgramConfig`: ~1.7 SOL rent exemption
- `UniversalNft`: ~2.5 SOL rent exemption
- `CrossChainTransfer`: ~2.0 SOL rent exemption

## Integration Examples

### TypeScript/JavaScript
```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { UniversalNft } from "./target/types/universal_nft";

const program = anchor.workspace.UniversalNft as Program<UniversalNft>;

// Mint NFT
const tx = await program.methods
  .mintNft("My NFT", "MNFT", "https://example.com/metadata.json", null)
  .accounts({
    // ... accounts
  })
  .rpc();
```

### Rust Client
```rust
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::{Client, Cluster};

let payer = Keypair::new();
let client = Client::new(Cluster::Devnet, payer);
let program = client.program(program_id);

let tx = program
    .request()
    .instruction(mint_nft_ix)
    .send()?;
```

## Rate Limits and Quotas

### Per-Account Limits
- Maximum 100 NFTs per owner in single transaction
- Maximum 10 cross-chain transfers per block
- Maximum 1,000 metadata updates per day

### Global Limits
- Maximum 10,000 NFTs minted per day
- Maximum 1,000 cross-chain transfers per hour
- Maximum 100 signature verifications per transaction