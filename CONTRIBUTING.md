# Contributing to ZetaChain Solana Universal NFT

Thank you for your interest in contributing to the ZetaChain Solana Universal NFT project! This document provides guidelines and information for contributors.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Contributing Guidelines](#contributing-guidelines)
- [Code Style](#code-style)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Security](#security)
- [Community](#community)

## Getting Started

### Prerequisites

Before contributing, ensure you have:

- **Rust** (latest stable version)
- **Solana CLI** v1.18+
- **Anchor Framework** v0.30.1+
- **Node.js** v18+
- **Git**
- Basic understanding of Solana programming
- Familiarity with cross-chain concepts

### Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/your-username/zetachain-solana-nft.git
   cd zetachain-solana-nft
   ```

2. **Install Dependencies**
   ```bash
   npm install
   ```

3. **Build the Project**
   ```bash
   anchor build
   ```

4. **Run Tests**
   ```bash
   anchor test
   ```

5. **Verify Setup**
   ```bash
   npm run test:devnet
   ```

## Contributing Guidelines

### Types of Contributions

We welcome the following types of contributions:

- **Bug Fixes**: Fix issues in the codebase
- **Feature Enhancements**: Improve existing functionality
- **New Features**: Add new capabilities
- **Documentation**: Improve docs, tutorials, examples
- **Testing**: Add or improve test coverage
- **Performance**: Optimize code performance
- **Security**: Address security vulnerabilities

### Before You Start

1. **Check Existing Issues**: Look for existing issues or discussions
2. **Create an Issue**: For new features or major changes, create an issue first
3. **Discuss**: Engage with maintainers and community before starting work
4. **Assign Yourself**: Comment on the issue to indicate you're working on it

### Development Workflow

1. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/issue-number
   ```

2. **Make Changes**
   - Follow code style guidelines
   - Add appropriate tests
   - Update documentation if needed

3. **Test Your Changes**
   ```bash
   anchor test
   npm run lint
   npm run test:devnet
   ```

4. **Commit Changes**
   ```bash
   git add .
   git commit -m "feat: add cross-chain signature verification"
   ```

5. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   ```

## Code Style

### Rust Code Style

Follow standard Rust conventions:

```rust
// Use descriptive names
pub struct UniversalNft {
    pub mint: Pubkey,
    pub owner: Pubkey,
    // ...
}

// Add comprehensive documentation
/// Verify cross-chain signature from ZetaChain TSS
/// 
/// # Arguments
/// * `message_hash` - Hash of the message to verify
/// * `signature` - ECDSA signature bytes
/// * `recovery_id` - Recovery ID for public key recovery
/// 
/// # Returns
/// * `Result<()>` - Ok if signature is valid, Err otherwise
pub fn verify_signature(
    ctx: Context<VerifySignature>,
    message_hash: [u8; 32],
    signature: [u8; 64],
    recovery_id: u8,
) -> Result<()> {
    // Implementation
}
```

### TypeScript Code Style

Follow TypeScript/JavaScript best practices:

```typescript
// Use proper types
interface NftMetadata {
  name: string;
  symbol: string;
  uri: string;
  collection?: PublicKey;
}

// Use async/await properly
async function mintUniversalNft(
  program: Program<UniversalNft>,
  metadata: NftMetadata
): Promise<string> {
  try {
    const tx = await program.methods
      .mintNft(metadata.name, metadata.symbol, metadata.uri, metadata.collection)
      .rpc();
    return tx;
  } catch (error) {
    console.error('Failed to mint NFT:', error);
    throw error;
  }
}
```

### Formatting

Use the provided formatters:

```bash
# Format Rust code
cargo fmt

# Format TypeScript/JavaScript
npm run lint:fix
```

## Testing

### Test Categories

1. **Unit Tests**: Test individual functions and components
2. **Integration Tests**: Test program interactions
3. **Cross-Chain Tests**: Test cross-chain functionality
4. **Security Tests**: Test security features

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_token_id() {
        let mint = Pubkey::new_unique();
        let block_number = 12345;
        let timestamp = 1640995200;
        
        let token_id = SignatureUtils::generate_token_id(&mint, block_number, timestamp);
        assert!(!token_id.is_empty());
        assert!(token_id.len() > 10);
    }
}
```

### Running Tests

```bash
# Run all tests
anchor test

# Run specific test file
anchor test tests/universal-nft.ts

# Run with coverage
npm run test:coverage
```

## Pull Request Process

### PR Checklist

Before submitting a PR, ensure:

- [ ] Code follows style guidelines
- [ ] All tests pass
- [ ] Documentation is updated
- [ ] Security considerations are addressed
- [ ] Performance impact is considered
- [ ] Breaking changes are documented

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests pass
- [ ] Manual testing completed

## Security
- [ ] Security review completed
- [ ] No sensitive data exposed
- [ ] Input validation added

## Documentation
- [ ] Code comments updated
- [ ] API documentation updated
- [ ] Tutorial/examples updated
```

### Review Process

1. **Automated Checks**: CI/CD pipeline runs automatically
2. **Code Review**: Maintainers review code quality and design
3. **Security Review**: Security-sensitive changes get extra scrutiny
4. **Testing**: Comprehensive testing on devnet
5. **Approval**: At least one maintainer approval required
6. **Merge**: Squash and merge after approval

## Security

### Security-First Development

- **Validate All Inputs**: Never trust user input
- **Follow Best Practices**: Use established security patterns
- **Test Edge Cases**: Consider attack scenarios
- **Document Security Features**: Explain security mechanisms

### Reporting Security Issues

**Do NOT create public issues for security vulnerabilities.**

Instead:

1. Email security@example.com with details
2. Include proof of concept if possible
3. Allow time for responsible disclosure
4. Coordinate public disclosure timing

### Security Review Requirements

The following changes require security review:

- Cross-chain message handling
- Signature verification logic
- Authority and permission changes
- Economic mechanisms (fees, rewards)
- State transitions and validation

## Community

### Communication Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and ideas
- **Discord**: Real-time community chat
- **Twitter**: Updates and announcements

### Code of Conduct

We follow the [Contributor Covenant](https://www.contributor-covenant.org/):

- **Be Respectful**: Treat everyone with respect
- **Be Inclusive**: Welcome contributors from all backgrounds
- **Be Constructive**: Provide helpful feedback
- **Be Patient**: Understand that reviews take time

### Recognition

Contributors are recognized through:

- **Contributors File**: Listed in CONTRIBUTORS.md
- **Release Notes**: Acknowledged in releases
- **Social Media**: Highlighted on Twitter
- **Bounties**: Eligible for development bounties

## Development Guidelines

### Performance Considerations

- **Optimize for Solana**: Consider compute unit limits
- **Efficient Data Structures**: Use appropriate account sizes
- **Minimize Cross-Program Calls**: Reduce transaction complexity
- **Test Under Load**: Ensure performance at scale

### Cross-Chain Considerations

- **Chain Compatibility**: Test with multiple chains
- **Message Format**: Maintain backward compatibility
- **Error Handling**: Implement proper revert mechanisms
- **Gas Estimation**: Provide accurate gas estimates

### Documentation Standards

- **Code Comments**: Explain complex logic
- **API Documentation**: Document all public interfaces
- **Examples**: Provide working examples
- **Tutorials**: Step-by-step guides for common tasks

## Release Process

### Version Numbering

We use [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

- [ ] All tests pass
- [ ] Documentation updated
- [ ] Security review completed
- [ ] Devnet testing completed
- [ ] Breaking changes documented
- [ ] Migration guide provided (if needed)

## Getting Help

If you need help:

1. **Check Documentation**: Review existing docs and tutorials
2. **Search Issues**: Look for similar questions
3. **Ask in Discussions**: Use GitHub Discussions for questions
4. **Join Discord**: Real-time help from community
5. **Create Issue**: For bugs or feature requests

## Thank You

Thank you for contributing to the ZetaChain Solana Universal NFT project! Your contributions help build the future of cross-chain NFTs.

---

*This contributing guide is a living document and may be updated as the project evolves.*