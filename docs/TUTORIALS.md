# Tutorials

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (latest stable version)
- **Solana CLI** (v1.18 or later)
- **Anchor Framework** (v0.30.1 or later)
- **Node.js** (v18 or later)
- **Git**

### Installation

```bash
# Clone the repository
git clone https://github.com/your-username/zetachain-solana-nft.git
cd zetachain-solana-nft

# Install dependencies
npm install

# Build the program
anchor build

# Run tests
anchor test
```

## Tutorial 1: Basic NFT Minting

### Objective
Learn how to mint a basic Universal NFT on Solana.

### Step 1: Setup

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { UniversalNft } from "../target/types/universal_nft";

const program = anchor.workspace.UniversalNft as Program<UniversalNft>;
const provider = anchor.AnchorProvider.env();
```

### Step 2: Initialize Program

```typescript
async function initializeProgram() {
  const [configPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );

  const gatewayAuthority = new PublicKey("GATEWAY_AUTHORITY_HERE");

  const tx = await program.methods
    .initialize(gatewayAuthority)
    .accounts({
      config: configPda,
      authority: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .rpc();

  console.log("Program initialized:", tx);
}
```

### Step 3: Mint Your First NFT

```typescript
async function mintNft() {
  const mint = Keypair.generate();
  const owner = provider.wallet.publicKey;

  // Find PDAs
  const [universalNftPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("universal_nft"), mint.publicKey.toBuffer()],
    program.programId
  );

  const [metadataPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.publicKey.toBuffer(),
    ],
    MPL_TOKEN_METADATA_PROGRAM_ID
  );

  // Mint the NFT
  const tx = await program.methods
    .mintNft(
      "My First Universal NFT",
      "MFUNFT", 
      "https://example.com/metadata.json",
      null
    )
    .accounts({
      config: configPda,
      universalNft: universalNftPda,
      mint: mint.publicKey,
      metadata: metadataPda,
      // ... other accounts
    })
    .signers([mint])
    .rpc();

  console.log("NFT minted:", tx);
  return { mint, universalNftPda };
}
```

### Step 4: Verify NFT

```typescript
async function verifyNft(universalNftPda: PublicKey) {
  const nft = await program.account.universalNft.fetch(universalNftPda);
  
  console.log("NFT Details:");
  console.log("Name:", nft.name);
  console.log("Symbol:", nft.symbol);
  console.log("Owner:", nft.owner.toString());
  console.log("Token ID:", nft.originTokenId);
}
```

## Tutorial 2: Cross-Chain Transfer

### Objective
Learn how to transfer an NFT from Solana to another blockchain.

### Step 1: Prepare for Transfer

```typescript
async function prepareTransfer(mint: Keypair, universalNftPda: PublicKey) {
  // Check NFT status
  const nft = await program.account.universalNft.fetch(universalNftPda);
  
  if (nft.isLocked) {
    throw new Error("NFT is already locked for transfer");
  }
  
  console.log("NFT ready for transfer");
  return nft;
}
```

### Step 2: Execute Cross-Chain Transfer

```typescript
async function transferToEthereum(
  mint: Keypair, 
  universalNftPda: PublicKey,
  ethereumRecipient: string
) {
  const destinationChainId = 5; // Ethereum Goerli
  const recipient = Array.from(Buffer.from(ethereumRecipient.slice(2), 'hex'));
  const gasLimit = 100000;

  // Get current nonce
  const config = await program.account.programConfig.fetch(configPda);
  const nonce = config.nonce.add(new anchor.BN(1));

  const [transferPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("transfer"),
      mint.publicKey.toBuffer(),
      nonce.toBuffer("le", 8),
    ],
    program.programId
  );

  const tx = await program.methods
    .burnAndTransfer(
      new anchor.BN(destinationChainId),
      recipient,
      new anchor.BN(gasLimit)
    )
    .accounts({
      config: configPda,
      universalNft: universalNftPda,
      transfer: transferPda,
      mint: mint.publicKey,
      // ... other accounts
    })
    .rpc();

  console.log("Cross-chain transfer initiated:", tx);
  return transferPda;
}
```

### Step 3: Monitor Transfer Status

```typescript
async function monitorTransfer(transferPda: PublicKey) {
  let status;
  do {
    const transfer = await program.account.crossChainTransfer.fetch(transferPda);
    status = transfer.status;
    
    console.log("Transfer status:", status);
    
    if (status.hasOwnProperty('completed')) {
      console.log("✅ Transfer completed!");
      break;
    } else if (status.hasOwnProperty('reverted')) {
      console.log("❌ Transfer reverted!");
      break;
    }
    
    // Wait 10 seconds before checking again
    await new Promise(resolve => setTimeout(resolve, 10000));
  } while (true);
}
```

## Tutorial 3: Creating NFT Collections

### Objective
Learn how to create and manage NFT collections.

### Step 1: Create Collection

```typescript
async function createCollection() {
  const collectionMint = Keypair.generate();

  const [collectionPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("collection"), collectionMint.publicKey.toBuffer()],
    program.programId
  );

  const tx = await program.methods
    .createCollection(
      "My Universal Collection",
      "MUC",
      "https://example.com/collection-metadata.json",
      new anchor.BN(10000) // Max supply
    )
    .accounts({
      config: configPda,
      collection: collectionPda,
      mint: collectionMint.publicKey,
      // ... other accounts
    })
    .signers([collectionMint])
    .rpc();

  console.log("Collection created:", tx);
  return { collectionMint, collectionPda };
}
```

### Step 2: Mint NFT in Collection

```typescript
async function mintNftInCollection(collectionMint: PublicKey) {
  const nftMint = Keypair.generate();

  const tx = await program.methods
    .mintNft(
      "Collection NFT #1",
      "CNFT",
      "https://example.com/nft-1-metadata.json",
      collectionMint // Specify collection
    )
    .accounts({
      // ... accounts
    })
    .signers([nftMint])
    .rpc();

  console.log("Collection NFT minted:", tx);
}
```

### Step 3: Verify Collection Membership

```typescript
async function verifyCollectionMembership(
  universalNftPda: PublicKey,
  collectionPda: PublicKey,
  collectionMint: PublicKey
) {
  const tx = await program.methods
    .verifyCollection()
    .accounts({
      config: configPda,
      universalNft: universalNftPda,
      collection: collectionPda,
      collectionMint: collectionMint,
      collectionAuthority: provider.wallet.publicKey,
    })
    .rpc();

  console.log("Collection verified:", tx);
}
```

## Tutorial 4: Advanced Security Features

### Objective
Learn about security features and how to implement them.

### Step 1: Signature Verification

```typescript
async function verifyTssSignature(
  messageHash: number[],
  signature: number[],
  recoveryId: number
) {
  try {
    const tx = await program.methods
      .verifySignature(messageHash, signature, recoveryId)
      .accounts({
        config: configPda,
      })
      .rpc();

    console.log("Signature verified:", tx);
    return true;
  } catch (error) {
    console.log("Signature verification failed:", error.message);
    return false;
  }
}
```

### Step 2: Cross-Chain Message Validation

```typescript
async function validateCrossChainMessage(
  nonce: number,
  chainId: number,
  recipient: number[],
  amount: number,
  data: number[],
  signature: number[],
  recoveryId: number
) {
  const tx = await program.methods
    .verifyCrossChainMessage(
      new anchor.BN(nonce),
      new anchor.BN(chainId),
      recipient,
      new anchor.BN(amount),
      data,
      signature,
      recoveryId
    )
    .accounts({
      config: configPda,
    })
    .rpc();

  console.log("Cross-chain message validated:", tx);
}
```

### Step 3: Emergency Pause

```typescript
async function pauseProgram() {
  const tx = await program.methods
    .updateConfig(null, null, true) // Set paused to true
    .accounts({
      config: configPda,
      authority: provider.wallet.publicKey,
    })
    .rpc();

  console.log("Program paused:", tx);
}

async function unpauseProgram() {
  const tx = await program.methods
    .updateConfig(null, null, false) // Set paused to false
    .accounts({
      config: configPda,
      authority: provider.wallet.publicKey,
    })
    .rpc();

  console.log("Program unpaused:", tx);
}
```

## Tutorial 5: Frontend Integration

### Objective
Learn how to integrate the Universal NFT program with a web frontend.

### Step 1: Setup Wallet Connection

```typescript
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets';

const network = WalletAdapterNetwork.Devnet;
const endpoint = clusterApiUrl(network);
const wallets = [new PhantomWalletAdapter()];

function App() {
  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <UniversalNftApp />
      </WalletProvider>
    </ConnectionProvider>
  );
}
```

### Step 2: Create NFT Component

```tsx
import { useWallet, useConnection } from '@solana/wallet-adapter-react';
import { Program, AnchorProvider } from '@coral-xyz/anchor';

function MintNftComponent() {
  const { connection } = useConnection();
  const wallet = useWallet();

  const mintNft = async () => {
    if (!wallet.connected) return;

    const provider = new AnchorProvider(connection, wallet, {});
    const program = new Program(idl, programId, provider);

    // Mint NFT logic here
    const tx = await program.methods
      .mintNft("Frontend NFT", "FNFT", "https://example.com/meta.json", null)
      .accounts({
        // ... accounts
      })
      .rpc();

    console.log("NFT minted:", tx);
  };

  return (
    <button onClick={mintNft} disabled={!wallet.connected}>
      Mint Universal NFT
    </button>
  );
}
```

### Step 3: Display NFT Gallery

```tsx
function NftGallery({ owner }: { owner: PublicKey }) {
  const [nfts, setNfts] = useState([]);
  const { connection } = useConnection();

  useEffect(() => {
    async function fetchNfts() {
      // Fetch all NFTs owned by the user
      const accounts = await program.account.universalNft.all([
        {
          memcmp: {
            offset: 8 + 32 + 8 + 4 + 64, // Skip to owner field
            bytes: owner.toBase58(),
          },
        },
      ]);

      setNfts(accounts);
    }

    if (owner) {
      fetchNfts();
    }
  }, [owner]);

  return (
    <div className="nft-gallery">
      {nfts.map((nft) => (
        <NftCard key={nft.publicKey.toString()} nft={nft.account} />
      ))}
    </div>
  );
}
```

## Best Practices

### 1. Error Handling

```typescript
async function safeOperation() {
  try {
    const result = await program.methods.someOperation().rpc();
    return { success: true, data: result };
  } catch (error) {
    console.error("Operation failed:", error);
    
    // Handle specific error types
    if (error.message.includes("ProgramPaused")) {
      return { success: false, error: "Program is currently paused" };
    } else if (error.message.includes("Unauthorized")) {
      return { success: false, error: "Insufficient permissions" };
    }
    
    return { success: false, error: "Unknown error occurred" };
  }
}
```

### 2. Gas Optimization

```typescript
// Pre-calculate PDAs to avoid recomputation
const pdaCache = new Map();

function getCachedPda(seeds: Buffer[], programId: PublicKey): PublicKey {
  const key = seeds.map(s => s.toString()).join(',');
  
  if (!pdaCache.has(key)) {
    const [pda] = PublicKey.findProgramAddressSync(seeds, programId);
    pdaCache.set(key, pda);
  }
  
  return pdaCache.get(key);
}
```

### 3. Transaction Confirmation

```typescript
async function confirmTransaction(signature: string) {
  const latestBlockHash = await connection.getLatestBlockhash();
  
  await connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: signature,
  });
  
  console.log("Transaction confirmed:", signature);
}
```

### 4. Batch Operations

```typescript
async function batchMintNfts(nftData: Array<{name: string, symbol: string, uri: string}>) {
  const instructions = [];
  
  for (const data of nftData) {
    const mint = Keypair.generate();
    
    const instruction = await program.methods
      .mintNft(data.name, data.symbol, data.uri, null)
      .accounts({
        // ... accounts
      })
      .instruction();
    
    instructions.push(instruction);
  }
  
  // Send all instructions in a single transaction
  const transaction = new Transaction().add(...instructions);
  const signature = await provider.sendAndConfirm(transaction);
  
  console.log("Batch mint completed:", signature);
}
```

## Troubleshooting

### Common Issues

1. **"Account does not exist"**
   - Ensure PDAs are derived correctly
   - Check if program is initialized

2. **"Insufficient funds"**
   - Ensure wallet has enough SOL for rent and transaction fees
   - Request airdrop on devnet: `solana airdrop 2`

3. **"Program is paused"**
   - Contact program authority to unpause
   - Check program configuration

4. **"Invalid signature"**
   - Verify TSS authority is correct
   - Check signature format and recovery ID

### Debug Mode

```typescript
// Enable debug mode for detailed logs
process.env.ANCHOR_LOG = "true";

// Use simulation for testing
const simulation = await connection.simulateTransaction(transaction);
console.log("Simulation result:", simulation);
```

## Next Steps

After completing these tutorials, you should:

1. Deploy your program to devnet
2. Test cross-chain functionality with ZetaChain
3. Build a complete frontend application
4. Prepare for mainnet deployment
5. Contribute to the ecosystem

For more advanced topics, see:
- [API Reference](./API.md)
- [Cross-Chain Guide](./CROSS_CHAIN.md)
- [Security Documentation](./SECURITY.md)