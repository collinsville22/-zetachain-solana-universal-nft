# Security Model

## Overview

The ZetaChain Solana Universal NFT program implements multiple layers of security to ensure safe cross-chain operations while maintaining decentralization and user control.

## Threat Model

### Identified Threats
1. **Cross-chain replay attacks**
2. **Unauthorized minting/burning**
3. **Signature forgery**
4. **Front-running attacks**
5. **Reentrancy vulnerabilities**
6. **Gateway compromise**
7. **TSS authority compromise**

## Security Mechanisms

### 1. Threshold Signature Scheme (TSS) Verification

#### Implementation
```rust
pub fn verify_ecdsa_signature(
    message_hash: &[u8; 32],
    signature: &[u8; 64],
    recovery_id: u8,
    expected_signer: &[u8; 20],
) -> Result<bool>
```

#### Security Properties
- **Distributed Trust**: No single point of failure
- **Signature Verification**: ECDSA secp256k1 compatibility
- **Authority Validation**: Only authorized TSS can sign
- **Message Integrity**: Hash-based message verification

### 2. Replay Attack Prevention

#### Nonce-Based Protection
```rust
pub struct CrossChainTransfer {
    pub nonce: u64,
    pub timestamp: i64,
    pub status: TransferStatus,
    // ...
}
```

#### Implementation Details
- **Monotonic Nonces**: Strictly increasing sequence numbers
- **Temporal Validation**: Timestamp-based expiry
- **State Tracking**: Transfer status management
- **Hash Validation**: Message integrity verification

### 3. Origin Verification

#### Instruction Origin Validation
```rust
fn verify_instruction_origin(instructions_sysvar: &UncheckedAccount) -> Result<()> {
    let instructions = SysvarInstructions::from_account_info(instructions_sysvar)?;
    // Validate calling program is authorized gateway
}
```

#### Gateway Authentication
- **Program ID Verification**: Only authorized gateway can call
- **Instruction Stack Validation**: Prevents indirect calls
- **Authority Checking**: Gateway must be properly configured

### 4. Access Control

#### Authority Hierarchy
```rust
pub struct ProgramConfig {
    pub authority: Pubkey,           // Program admin
    pub gateway_authority: Pubkey,   // Cross-chain gateway
    pub tss_authority: Pubkey,       // TSS signature authority
}
```

#### Permission Model
- **Program Authority**: Configuration updates, emergency pause
- **Gateway Authority**: Cross-chain message processing
- **TSS Authority**: Signature verification for cross-chain ops
- **NFT Owner**: Transfer, metadata updates for owned NFTs

### 5. State Validation

#### Account Constraints
```rust
#[account(
    mut,
    seeds = [b"universal_nft", mint.key().as_ref()],
    bump = universal_nft.bump,
    constraint = !universal_nft.is_locked @ UniversalNftError::NftLocked
)]
pub universal_nft: Account<'info, UniversalNft>,
```

#### Safety Checks
- **Lock Status**: Prevents double-spending during transfers
- **Ownership Verification**: Only owner can transfer/update
- **Mint Validation**: Ensures NFT account matches mint
- **Collection Verification**: Validates collection membership

### 6. Input Validation

#### Metadata Validation
```rust
impl MetadataUtils {
    pub fn validate_name(name: &str) -> Result<bool> { /* ... */ }
    pub fn validate_symbol(symbol: &str) -> Result<bool> { /* ... */ }
    pub fn validate_uri(uri: &str) -> Result<bool> { /* ... */ }
}
```

#### Cross-Chain Parameters
```rust
impl CrossChainUtils {
    pub fn validate_chain_id(chain_id: u64) -> Result<bool> { /* ... */ }
    pub fn validate_recipient(recipient: &[u8]) -> Result<bool> { /* ... */ }
    pub fn validate_gas_limit(gas_limit: u64) -> Result<bool> { /* ... */ }
}
```

## Attack Scenarios and Mitigations

### 1. Replay Attack

**Scenario**: Attacker resubmits valid cross-chain message
**Mitigation**: 
- Nonce validation ensures messages are processed once
- Timestamp validation prevents old message reuse
- State tracking prevents duplicate processing

### 2. Signature Forgery

**Scenario**: Attacker attempts to forge TSS signature
**Mitigation**:
- ECDSA signature verification with public key recovery
- Authorized TSS authority validation
- Message hash integrity checking

### 3. Unauthorized Minting

**Scenario**: Attacker tries to mint NFTs without authorization
**Mitigation**:
- Gateway authority verification required
- TSS signature validation for cross-chain mints
- Origin chain validation

### 4. Double Spending

**Scenario**: NFT transferred while already in cross-chain transit
**Mitigation**:
- Lock mechanism prevents operations on transferring NFTs
- Status tracking ensures atomic operations
- Revert mechanism for failed transfers

### 5. Front-running

**Scenario**: Attacker observes transaction and tries to front-run
**Mitigation**:
- Deterministic token ID generation
- Nonce-based ordering
- Time-based validation windows

## Emergency Procedures

### Program Pause

```rust
pub fn update_config(
    ctx: Context<UpdateConfig>,
    paused: Option<bool>,
) -> Result<()> {
    if let Some(is_paused) = paused {
        config.is_paused = is_paused;
    }
}
```

#### Pause Conditions
- Detected security vulnerability
- Gateway compromise
- Abnormal transaction patterns
- Emergency maintenance

#### Pause Effects
- All operations except emergency functions disabled
- Existing NFTs remain safe
- Cross-chain operations suspended
- Metadata updates restricted

### Authority Rotation

#### TSS Authority Update
```rust
config.tss_authority = new_tss_authority;
```

#### Gateway Authority Update
```rust
config.gateway_authority = new_gateway_authority;
```

## Audit Considerations

### Code Review Focus Areas
1. **Signature verification logic**
2. **Nonce handling and validation**
3. **Cross-program invocation security**
4. **Account constraint validation**
5. **Error handling and edge cases**

### Testing Requirements
1. **Signature verification with invalid signatures**
2. **Replay attack scenarios**
3. **Unauthorized access attempts**
4. **Edge cases in cross-chain flows**
5. **Compute budget exhaustion scenarios**

### Formal Verification
- **State transition safety**
- **Invariant preservation**
- **Access control correctness**
- **Cross-chain consistency**

## Security Best Practices

### Development
- Input validation on all user-provided data
- Fail-safe defaults for security-critical operations
- Minimal privilege principle for all authorities
- Comprehensive error handling and logging

### Deployment
- Multi-signature requirements for authority updates
- Gradual rollout with monitoring
- Regular security assessments
- Incident response procedures

### Operations
- Real-time monitoring of cross-chain operations
- Anomaly detection for unusual patterns
- Regular authority key rotation
- Backup and recovery procedures

## Compliance and Standards

### Industry Standards
- **OWASP Blockchain Security**: General blockchain security principles
- **Solana Security Best Practices**: Platform-specific guidelines
- **Cross-chain Security Standards**: Emerging best practices

### Regulatory Considerations
- **Data Privacy**: Minimal on-chain personal data
- **Anti-Money Laundering**: Transaction monitoring capabilities
- **Know Your Customer**: Integration with compliant services
- **Jurisdictional Compliance**: Configurable restrictions