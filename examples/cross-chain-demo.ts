/**
 * ZetaChain Solana Universal NFT Cross-Chain Demo
 * 
 * This example demonstrates the complete cross-chain NFT flow:
 * 1. Mint NFT on Solana
 * 2. Transfer to Ethereum via ZetaChain
 * 3. Transfer back to Solana
 */

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { UniversalNft } from "../target/types/universal_nft";
import { 
  PublicKey, 
  Keypair, 
  SystemProgram, 
  SYSVAR_RENT_PUBKEY,
  Connection,
  clusterApiUrl 
} from "@solana/web3.js";
import { 
  TOKEN_PROGRAM_ID, 
  ASSOCIATED_TOKEN_PROGRAM_ID, 
  getAssociatedTokenAddress 
} from "@solana/spl-token";

// Chain IDs
const CHAIN_IDS = {
  SOLANA: 900,
  ZETACHAIN_TESTNET: 7001,
  ETHEREUM_GOERLI: 5,
  BSC_TESTNET: 97,
};

// Program and Gateway IDs (update with actual values)
const PROGRAM_ID = new PublicKey("EiGgwyFXtqcNEutPaUe94J9c9sPaPnDWj64sFcD7W9sz");
const GATEWAY_PROGRAM_ID = new PublicKey("ZETAjseVjuFsJ3pSrPjS3k2pZ9s3q4Yrp6Cz6h5VLnN");
const MPL_TOKEN_METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

class CrossChainDemo {
  private program: Program<UniversalNft>;
  private provider: anchor.AnchorProvider;
  private connection: Connection;

  constructor() {
    // Setup connection to Solana devnet
    this.connection = new Connection(clusterApiUrl("devnet"), "confirmed");
    
    // Setup wallet (you should replace this with your actual wallet)
    const wallet = anchor.Wallet.local();
    this.provider = new anchor.AnchorProvider(this.connection, wallet, {
      commitment: "confirmed",
    });
    
    // Load program
    this.program = anchor.workspace.UniversalNft as Program<UniversalNft>;
  }

  /**
   * Step 1: Mint a Universal NFT on Solana
   */
  async mintUniversalNft(
    owner: PublicKey,
    name: string,
    symbol: string,
    uri: string,
    collection?: PublicKey
  ): Promise<{
    mint: Keypair;
    universalNftPda: PublicKey;
    tokenAccount: PublicKey;
    signature: string;
  }> {
    console.log("🎨 Minting Universal NFT on Solana...");
    
    // Generate new mint keypair
    const mint = Keypair.generate();
    
    // Find PDAs
    const [configPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      this.program.programId
    );

    const [universalNftPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("universal_nft"), mint.publicKey.toBuffer()],
      this.program.programId
    );

    const [metadataPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.publicKey.toBuffer(),
      ],
      MPL_TOKEN_METADATA_PROGRAM_ID
    );

    const [masterEditionPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.publicKey.toBuffer(),
        Buffer.from("edition"),
      ],
      MPL_TOKEN_METADATA_PROGRAM_ID
    );

    const tokenAccount = await getAssociatedTokenAddress(mint.publicKey, owner);

    // Mint the NFT
    const signature = await this.program.methods
      .mintNft(name, symbol, uri, collection || null)
      .accounts({
        config: configPda,
        universalNft: universalNftPda,
        mint: mint.publicKey,
        metadata: metadataPda,
        masterEdition: masterEditionPda,
        tokenAccount: tokenAccount,
        mintAuthority: universalNftPda,
        owner: owner,
        payer: this.provider.wallet.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .signers([mint])
      .rpc();

    console.log(`✅ NFT minted successfully!`);
    console.log(`   Mint: ${mint.publicKey.toString()}`);
    console.log(`   Owner: ${owner.toString()}`);
    console.log(`   Transaction: ${signature}`);

    // Verify the mint
    const nftAccount = await this.program.account.universalNft.fetch(universalNftPda);
    console.log(`   Token ID: ${nftAccount.originTokenId}`);
    console.log(`   Origin Chain: ${nftAccount.originChainId}`);

    return {
      mint,
      universalNftPda,
      tokenAccount,
      signature,
    };
  }

  /**
   * Step 2: Transfer NFT from Solana to another chain
   */
  async transferToEthereum(
    mint: Keypair,
    universalNftPda: PublicKey,
    owner: Keypair,
    ethereumRecipient: string
  ): Promise<{
    transferPda: PublicKey;
    signature: string;
  }> {
    console.log("🌉 Transferring NFT from Solana to Ethereum...");
    
    // Validate Ethereum address format
    if (!ethereumRecipient.startsWith("0x") || ethereumRecipient.length !== 42) {
      throw new Error("Invalid Ethereum address format");
    }

    const destinationChainId = CHAIN_IDS.ETHEREUM_GOERLI;
    const recipient = Array.from(Buffer.from(ethereumRecipient.slice(2), 'hex'));
    const gasLimit = 100000;

    // Find PDAs
    const [configPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      this.program.programId
    );

    // Get current nonce for transfer PDA
    const config = await this.program.account.programConfig.fetch(configPda);
    const nonce = config.nonce.add(new anchor.BN(1));

    const [transferPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("transfer"),
        mint.publicKey.toBuffer(),
        nonce.toBuffer("le", 8),
      ],
      this.program.programId
    );

    const tokenAccount = await getAssociatedTokenAddress(mint.publicKey, owner.publicKey);

    // Execute burn and transfer
    const signature = await this.program.methods
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
        tokenAccount: tokenAccount,
        owner: owner.publicKey,
        gatewayProgram: GATEWAY_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([owner])
      .rpc();

    console.log(`✅ Cross-chain transfer initiated!`);
    console.log(`   Destination: Ethereum (Chain ID: ${destinationChainId})`);
    console.log(`   Recipient: ${ethereumRecipient}`);
    console.log(`   Transaction: ${signature}`);

    // Verify transfer state
    const transferAccount = await this.program.account.crossChainTransfer.fetch(transferPda);
    console.log(`   Transfer Status: ${JSON.stringify(transferAccount.status)}`);
    console.log(`   Nonce: ${transferAccount.nonce}`);

    return {
      transferPda,
      signature,
    };
  }

  /**
   * Step 3: Simulate receiving NFT from another chain
   */
  async simulateIncomingTransfer(
    tokenId: string,
    name: string,
    symbol: string,
    uri: string,
    recipient: PublicKey,
    sourceChainId: number = CHAIN_IDS.ETHEREUM_GOERLI
  ): Promise<string> {
    console.log("📥 Simulating incoming cross-chain transfer...");

    // Create cross-chain message
    const message = {
      mintNft: {
        tokenId,
        name,
        symbol,
        uri,
        recipient,
        collectionMint: null,
      }
    };

    // Serialize message (this would typically be done by the gateway)
    const serializedMessage = Buffer.from(JSON.stringify(message));
    const sender = Array.from(Buffer.alloc(20, 1)); // Mock Ethereum sender

    // Find config PDA
    const [configPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      this.program.programId
    );

    // This would typically be called by the ZetaChain gateway
    // For demo purposes, we're simulating the call
    console.log("⚠️  Note: This would typically be called by ZetaChain Gateway");
    console.log(`   Source Chain: ${sourceChainId}`);
    console.log(`   Token ID: ${tokenId}`);
    console.log(`   Recipient: ${recipient.toString()}`);

    // In a real scenario, the gateway would call on_call with proper authentication
    // Here we just log what would happen
    console.log("📝 Would execute on_call with:");
    console.log(`   - Sender: ${Buffer.from(sender).toString('hex')}`);
    console.log(`   - Source Chain ID: ${sourceChainId}`);
    console.log(`   - Message: ${serializedMessage.toString('hex')}`);

    return "simulated_transaction_hash";
  }

  /**
   * Monitor transfer status
   */
  async monitorTransfer(transferPda: PublicKey): Promise<void> {
    console.log("📊 Monitoring transfer status...");
    
    let attempts = 0;
    const maxAttempts = 30; // 5 minutes with 10-second intervals
    
    while (attempts < maxAttempts) {
      try {
        const transferAccount = await this.program.account.crossChainTransfer.fetch(transferPda);
        const status = JSON.stringify(transferAccount.status);
        
        console.log(`   Status: ${status} (attempt ${attempts + 1}/${maxAttempts})`);
        
        if (status.includes("Completed") || status.includes("Reverted")) {
          console.log(`✅ Transfer ${status.toLowerCase()}`);
          break;
        }
        
        // Wait 10 seconds before next check
        await new Promise(resolve => setTimeout(resolve, 10000));
        attempts++;
        
      } catch (error) {
        console.error(`❌ Error monitoring transfer: ${error.message}`);
        break;
      }
    }
    
    if (attempts >= maxAttempts) {
      console.log("⏰ Transfer monitoring timed out");
    }
  }

  /**
   * Display NFT information
   */
  async displayNftInfo(universalNftPda: PublicKey): Promise<void> {
    try {
      const nftAccount = await this.program.account.universalNft.fetch(universalNftPda);
      
      console.log("📋 NFT Information:");
      console.log(`   Name: ${nftAccount.name}`);
      console.log(`   Symbol: ${nftAccount.symbol}`);
      console.log(`   URI: ${nftAccount.uri}`);
      console.log(`   Owner: ${nftAccount.owner.toString()}`);
      console.log(`   Token ID: ${nftAccount.originTokenId}`);
      console.log(`   Origin Chain: ${nftAccount.originChainId}`);
      console.log(`   Mint: ${nftAccount.mint.toString()}`);
      console.log(`   Locked: ${nftAccount.isLocked}`);
      console.log(`   Created: ${new Date(nftAccount.creationTimestamp.toNumber() * 1000).toISOString()}`);
      
    } catch (error) {
      console.error(`❌ Error fetching NFT info: ${error.message}`);
    }
  }

  /**
   * Run complete cross-chain demo
   */
  async runDemo(): Promise<void> {
    console.log("🚀 Starting ZetaChain Solana Universal NFT Cross-Chain Demo");
    console.log("===========================================================");
    
    try {
      // Demo user
      const user = Keypair.generate();
      
      // Airdrop SOL for demo
      console.log("💰 Requesting airdrop for demo user...");
      const airdropSignature = await this.connection.requestAirdrop(
        user.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
      );
      await this.connection.confirmTransaction(airdropSignature);
      
      // Step 1: Mint NFT on Solana
      const { mint, universalNftPda, tokenAccount } = await this.mintUniversalNft(
        user.publicKey,
        "Cross-Chain Demo NFT",
        "CCDNFT",
        "https://example.com/demo-nft-metadata.json"
      );
      
      await this.displayNftInfo(universalNftPda);
      
      // Step 2: Transfer to Ethereum
      const ethereumRecipient = "0x742d35Cc6634C0532925a3b8D474C2f83C1b3d4C";
      const { transferPda } = await this.transferToEthereum(
        mint,
        universalNftPda,
        user,
        ethereumRecipient
      );
      
      // Step 3: Monitor transfer (in a real scenario, this would show actual progress)
      console.log("\n⏳ In a real scenario, you would now:");
      console.log("   1. Monitor the transfer on ZetaChain");
      console.log("   2. See the NFT appear on Ethereum");
      console.log("   3. Interact with it on Ethereum");
      console.log("   4. Transfer it back to Solana if desired");
      
      // Simulate monitoring
      setTimeout(async () => {
        await this.displayNftInfo(universalNftPda);
      }, 2000);
      
      console.log("\n🎉 Cross-chain demo completed successfully!");
      console.log("\n📚 Next Steps:");
      console.log("   - Deploy to devnet/mainnet");
      console.log("   - Integrate with ZetaChain gateway");
      console.log("   - Test with real cross-chain transfers");
      console.log("   - Build frontend application");
      
    } catch (error) {
      console.error("❌ Demo failed:", error);
      throw error;
    }
  }
}

// Example usage functions
export async function runBasicMintExample() {
  const demo = new CrossChainDemo();
  const user = Keypair.generate();
  
  // Airdrop for demo
  await demo.connection.requestAirdrop(user.publicKey, anchor.web3.LAMPORTS_PER_SOL);
  
  const result = await demo.mintUniversalNft(
    user.publicKey,
    "Example NFT",
    "EXAM",
    "https://example.com/metadata.json"
  );
  
  await demo.displayNftInfo(result.universalNftPda);
  return result;
}

export async function runCrossChainExample() {
  const demo = new CrossChainDemo();
  await demo.runDemo();
}

// CLI runner
if (require.main === module) {
  const demo = new CrossChainDemo();
  demo.runDemo()
    .then(() => {
      console.log("Demo completed successfully!");
      process.exit(0);
    })
    .catch((error) => {
      console.error("Demo failed:", error);
      process.exit(1);
    });
}