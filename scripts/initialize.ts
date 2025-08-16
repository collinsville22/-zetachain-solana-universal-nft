import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider } from "@coral-xyz/anchor";
import { UniversalNft } from "../target/types/universal_nft";
import { PublicKey, Keypair, SystemProgram, Connection, clusterApiUrl } from "@solana/web3.js";
import fs from "fs";
import path from "path";

// Configuration
const PROGRAM_ID = new PublicKey("EiGgwyFXtqcNEutPaUe94J9c9sPaPnDWj64sFcD7W9sz");
const GATEWAY_PROGRAM_ID = new PublicKey("ZETAjseVjuFsJ3pSrPjS3k2pZ9s3q4Yrp6Cz6h5VLnN"); // Replace with actual ZetaChain gateway

interface NetworkConfig {
  rpcUrl: string;
  gatewayAuthority: PublicKey;
  tssAuthority?: PublicKey;
}

const NETWORK_CONFIGS: Record<string, NetworkConfig> = {
  devnet: {
    rpcUrl: clusterApiUrl("devnet"),
    gatewayAuthority: new PublicKey("ZETAjseVjuFsJ3pSrPjS3k2pZ9s3q4Yrp6Cz6h5VLnN"),
    tssAuthority: new PublicKey("TSS2jseVjuFsJ3pSrPjS3k2pZ9s3q4Yrp6Cz6h5VLnN"),
  },
  mainnet: {
    rpcUrl: clusterApiUrl("mainnet-beta"),
    gatewayAuthority: new PublicKey("ZETA1jseVjuFsJ3pSrPjS3k2pZ9s3q4Yrp6Cz6h5VLnN"),
    tssAuthority: new PublicKey("TSS1jseVjuFsJ3pSrPjS3k2pZ9s3q4Yrp6Cz6h5VLnN"),
  },
};

async function initializeProgram(network: string = "devnet") {
  console.log(`🚀 Initializing Universal NFT Program on ${network.toUpperCase()}`);
  console.log("========================================================");

  // Get network configuration
  const config = NETWORK_CONFIGS[network];
  if (!config) {
    throw new Error(`Unsupported network: ${network}`);
  }

  // Setup connection and provider
  const connection = new Connection(config.rpcUrl, "confirmed");
  
  // Load wallet
  const walletPath = path.join(process.env.HOME || "", ".config", "solana", "id.json");
  if (!fs.existsSync(walletPath)) {
    throw new Error(`Wallet not found at ${walletPath}. Please generate a keypair first.`);
  }
  
  const walletKeypair = Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync(walletPath, "utf8")))
  );
  
  const wallet = new anchor.Wallet(walletKeypair);
  const provider = new AnchorProvider(connection, wallet, {
    commitment: "confirmed",
    preflightCommitment: "confirmed",
  });
  
  // Load program
  const idl = JSON.parse(
    fs.readFileSync(path.join(__dirname, "../target/idl/universal_nft.json"), "utf8")
  );
  const program = new Program(idl, PROGRAM_ID, provider) as Program<UniversalNft>;

  console.log(`📍 Network: ${network}`);
  console.log(`🔗 RPC: ${config.rpcUrl}`);
  console.log(`👛 Wallet: ${wallet.publicKey.toString()}`);
  console.log(`📋 Program ID: ${PROGRAM_ID.toString()}`);
  console.log("");

  // Check wallet balance
  const balance = await connection.getBalance(wallet.publicKey);
  console.log(`💰 Wallet balance: ${balance / anchor.web3.LAMPORTS_PER_SOL} SOL`);
  
  if (balance < 0.1 * anchor.web3.LAMPORTS_PER_SOL) {
    console.warn("⚠️  Warning: Low wallet balance. You may need more SOL for initialization.");
  }

  // Find program config PDA
  const [configPda, configBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    PROGRAM_ID
  );

  console.log(`🔧 Config PDA: ${configPda.toString()}`);

  // Check if already initialized
  try {
    const configAccount = await program.account.programConfig.fetch(configPda);
    console.log("ℹ️  Program already initialized!");
    console.log(`   Authority: ${configAccount.authority.toString()}`);
    console.log(`   Gateway Authority: ${configAccount.gatewayAuthority.toString()}`);
    console.log(`   TSS Authority: ${configAccount.tssAuthority.toString()}`);
    console.log(`   Nonce: ${configAccount.nonce.toString()}`);
    console.log(`   Paused: ${configAccount.isPaused}`);
    
    // Ask if user wants to update configuration
    const readline = require("readline").createInterface({
      input: process.stdin,
      output: process.stdout,
    });
    
    const updateConfig = await new Promise<boolean>((resolve) => {
      readline.question("Do you want to update the configuration? (y/N): ", (answer) => {
        readline.close();
        resolve(answer.toLowerCase() === "y" || answer.toLowerCase() === "yes");
      });
    });
    
    if (updateConfig) {
      await updateProgramConfig(program, configPda, config);
    }
    
    return;
  } catch (error) {
    if (error.message.includes("Account does not exist")) {
      console.log("🔄 Program not initialized. Proceeding with initialization...");
    } else {
      throw error;
    }
  }

  // Initialize the program
  try {
    console.log("🔄 Initializing program...");
    
    const tx = await program.methods
      .initialize(config.gatewayAuthority)
      .accounts({
        config: configPda,
        authority: wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log(`✅ Program initialized successfully!`);
    console.log(`📝 Transaction: ${tx}`);

    // Verify initialization
    const configAccount = await program.account.programConfig.fetch(configPda);
    console.log("\n📋 Configuration Summary:");
    console.log(`   Authority: ${configAccount.authority.toString()}`);
    console.log(`   Gateway Authority: ${configAccount.gatewayAuthority.toString()}`);
    console.log(`   TSS Authority: ${configAccount.tssAuthority.toString()}`);
    console.log(`   Nonce: ${configAccount.nonce.toString()}`);
    console.log(`   Paused: ${configAccount.isPaused}`);

    // Set TSS authority if provided
    if (config.tssAuthority) {
      console.log("\n🔄 Setting TSS authority...");
      
      const updateTx = await program.methods
        .updateConfig(null, config.tssAuthority, null)
        .accounts({
          config: configPda,
          authority: wallet.publicKey,
        })
        .rpc();

      console.log(`✅ TSS authority set successfully!`);
      console.log(`📝 Transaction: ${updateTx}`);
    }

    // Save deployment info
    saveDeploymentInfo(network, {
      programId: PROGRAM_ID.toString(),
      configPda: configPda.toString(),
      authority: wallet.publicKey.toString(),
      gatewayAuthority: config.gatewayAuthority.toString(),
      tssAuthority: config.tssAuthority?.toString(),
      network,
      deploymentDate: new Date().toISOString(),
    });

    console.log("\n🎉 Initialization completed successfully!");
    console.log("📄 Deployment info saved to deployment.json");
    
  } catch (error) {
    console.error("❌ Initialization failed:", error);
    throw error;
  }
}

async function updateProgramConfig(
  program: Program<UniversalNft>,
  configPda: PublicKey,
  config: NetworkConfig
) {
  console.log("🔄 Updating program configuration...");
  
  try {
    const tx = await program.methods
      .updateConfig(
        config.gatewayAuthority,
        config.tssAuthority || null,
        null // Don't change pause status
      )
      .accounts({
        config: configPda,
        authority: program.provider.publicKey,
      })
      .rpc();

    console.log(`✅ Configuration updated successfully!`);
    console.log(`📝 Transaction: ${tx}`);

    // Verify update
    const configAccount = await program.account.programConfig.fetch(configPda);
    console.log("\n📋 Updated Configuration:");
    console.log(`   Gateway Authority: ${configAccount.gatewayAuthority.toString()}`);
    console.log(`   TSS Authority: ${configAccount.tssAuthority.toString()}`);

  } catch (error) {
    console.error("❌ Configuration update failed:", error);
    throw error;
  }
}

function saveDeploymentInfo(network: string, info: any) {
  const deploymentPath = path.join(__dirname, "..", "deployment.json");
  
  let deploymentData: any = {};
  if (fs.existsSync(deploymentPath)) {
    deploymentData = JSON.parse(fs.readFileSync(deploymentPath, "utf8"));
  }
  
  deploymentData[network] = info;
  
  fs.writeFileSync(deploymentPath, JSON.stringify(deploymentData, null, 2));
}

async function verifyDeployment(network: string = "devnet") {
  console.log(`🔍 Verifying deployment on ${network.toUpperCase()}`);
  console.log("==========================================");

  const config = NETWORK_CONFIGS[network];
  const connection = new Connection(config.rpcUrl, "confirmed");

  // Load deployment info
  const deploymentPath = path.join(__dirname, "..", "deployment.json");
  if (!fs.existsSync(deploymentPath)) {
    throw new Error("Deployment info not found. Please initialize first.");
  }

  const deploymentData = JSON.parse(fs.readFileSync(deploymentPath, "utf8"));
  const networkData = deploymentData[network];
  
  if (!networkData) {
    throw new Error(`No deployment data found for ${network}`);
  }

  console.log("📋 Deployment Information:");
  console.log(`   Program ID: ${networkData.programId}`);
  console.log(`   Config PDA: ${networkData.configPda}`);
  console.log(`   Authority: ${networkData.authority}`);
  console.log(`   Deployment Date: ${networkData.deploymentDate}`);

  // Verify program exists
  const programInfo = await connection.getAccountInfo(new PublicKey(networkData.programId));
  if (!programInfo) {
    throw new Error("Program account not found");
  }

  console.log(`✅ Program account verified (${programInfo.data.length} bytes)`);

  // Verify config account
  const configInfo = await connection.getAccountInfo(new PublicKey(networkData.configPda));
  if (!configInfo) {
    throw new Error("Config account not found");
  }

  console.log(`✅ Config account verified`);

  // Load program and verify configuration
  const walletPath = path.join(process.env.HOME || "", ".config", "solana", "id.json");
  const walletKeypair = Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync(walletPath, "utf8")))
  );
  const wallet = new anchor.Wallet(walletKeypair);
  const provider = new AnchorProvider(connection, wallet, { commitment: "confirmed" });
  
  const idl = JSON.parse(
    fs.readFileSync(path.join(__dirname, "../target/idl/universal_nft.json"), "utf8")
  );
  const program = new Program(idl, new PublicKey(networkData.programId), provider) as Program<UniversalNft>;

  const configAccount = await program.account.programConfig.fetch(new PublicKey(networkData.configPda));
  
  console.log("\n📋 Current Configuration:");
  console.log(`   Authority: ${configAccount.authority.toString()}`);
  console.log(`   Gateway Authority: ${configAccount.gatewayAuthority.toString()}`);
  console.log(`   TSS Authority: ${configAccount.tssAuthority.toString()}`);
  console.log(`   Nonce: ${configAccount.nonce.toString()}`);
  console.log(`   Paused: ${configAccount.isPaused}`);

  console.log("\n✅ Deployment verification completed successfully!");
}

// CLI interface
async function main() {
  const args = process.argv.slice(2);
  const command = args[0];
  const network = args[1] || "devnet";

  try {
    switch (command) {
      case "init":
      case "initialize":
        await initializeProgram(network);
        break;
      case "verify":
        await verifyDeployment(network);
        break;
      case "help":
      default:
        console.log("ZetaChain Solana Universal NFT Initialization Script");
        console.log("===================================================");
        console.log("");
        console.log("Usage:");
        console.log("  npm run initialize [network]     - Initialize the program");
        console.log("  npm run verify [network]         - Verify deployment");
        console.log("");
        console.log("Networks:");
        console.log("  devnet (default)");
        console.log("  mainnet");
        console.log("");
        console.log("Examples:");
        console.log("  npm run initialize devnet");
        console.log("  npm run verify mainnet");
        break;
    }
  } catch (error) {
    console.error("❌ Error:", error.message);
    process.exit(1);
  }
}

if (require.main === module) {
  main();
}

export { initializeProgram, verifyDeployment };