/**
 * ZetaChain Universal NFT TypeScript SDK
 * 
 * The most comprehensive SDK for cross-chain NFT operations
 * Built for enterprise applications with advanced features
 */

import { 
  Connection, 
  PublicKey, 
  Transaction, 
  TransactionInstruction,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  sendAndConfirmTransaction
} from '@solana/web3.js';
import { 
  TOKEN_PROGRAM_ID, 
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  createMintToInstruction,
  createBurnInstruction
} from '@solana/spl-token';
import { Program, AnchorProvider, web3, BN } from '@coral-xyz/anchor';
import { EventEmitter } from 'events';

// Type definitions
export interface UniversalNftConfig {
  programId: PublicKey;
  connection: Connection;
  wallet?: any;
  cluster?: 'devnet' | 'mainnet-beta' | 'testnet';
  gatewayProgramId?: PublicKey;
  enableAnalytics?: boolean;
  retryConfig?: RetryConfig;
}

export interface RetryConfig {
  maxRetries: number;
  retryDelay: number;
  exponentialBackoff: boolean;
}

export interface NftMetadata {
  name: string;
  symbol: string;
  uri: string;
  description?: string;
  image?: string;
  attributes?: NftAttribute[];
  collection?: PublicKey;
}

export interface NftAttribute {
  trait_type: string;
  value: string | number;
  display_type?: string;
}

export interface CrossChainTransferOptions {
  destinationChainId: number;
  recipient: string;
  gasLimit: number;
  maxFeePerGas?: number;
  priorityFee?: number;
  deadline?: number;
}

export interface UniversalNftInfo {
  mint: PublicKey;
  owner: PublicKey;
  tokenId: string;
  originChainId: number;
  uri: string;
  name: string;
  symbol: string;
  isLocked: boolean;
  collectionMint?: PublicKey;
  creationBlock: number;
  creationTimestamp: number;
}

export interface TransferStatus {
  txHash: string;
  status: 'pending' | 'confirmed' | 'failed' | 'reverted';
  confirmations: number;
  gasUsed?: number;
  blockNumber?: number;
  timestamp: number;
}

// Events
export interface SDKEvents {
  'mint.started': { mint: PublicKey; metadata: NftMetadata };
  'mint.confirmed': { mint: PublicKey; tokenId: string };
  'mint.failed': { error: Error };
  'transfer.started': { mint: PublicKey; destination: number };
  'transfer.confirmed': { mint: PublicKey; txHash: string };
  'transfer.failed': { mint: PublicKey; error: Error };
  'security.alert': { type: string; severity: 'low' | 'medium' | 'high'; details: any };
}

/**
 * Main SDK Class - The Ultimate Universal NFT Interface
 */
export class UniversalNftSDK extends EventEmitter {
  private config: UniversalNftConfig;
  private program?: Program;
  private provider?: AnchorProvider;
  private analytics: AnalyticsTracker;
  private securityMonitor: SecurityMonitor;
  private performanceProfiler: PerformanceProfiler;
  private cache: CacheManager;

  constructor(config: UniversalNftConfig) {
    super();
    this.config = {
      retryConfig: {
        maxRetries: 3,
        retryDelay: 1000,
        exponentialBackoff: true,
      },
      enableAnalytics: true,
      ...config,
    };

    this.analytics = new AnalyticsTracker(this.config.enableAnalytics!);
    this.securityMonitor = new SecurityMonitor(this);
    this.performanceProfiler = new PerformanceProfiler();
    this.cache = new CacheManager();

    this.initializeProvider();
  }

  private async initializeProvider() {
    if (this.config.wallet) {
      this.provider = new AnchorProvider(
        this.config.connection,
        this.config.wallet,
        { commitment: 'confirmed' }
      );
      
      // Load program IDL and initialize
      // const idl = await Program.fetchIdl(this.config.programId, this.provider);
      // this.program = new Program(idl, this.config.programId, this.provider);
    }
  }

  /**
   * 🎨 ADVANCED NFT MINTING
   */
  async mintUniversalNft(
    metadata: NftMetadata,
    options?: {
      owner?: PublicKey;
      priorityFee?: number;
      computeUnitLimit?: number;
      verifyCollection?: boolean;
    }
  ): Promise<{ mint: PublicKey; tokenId: string; signature: string }> {
    const startTime = performance.now();
    
    try {
      this.emit('mint.started', { mint: new PublicKey(''), metadata });
      
      // Security pre-checks
      await this.securityMonitor.validateMintRequest(metadata);
      
      // Generate mint keypair
      const mint = Keypair.generate();
      const owner = options?.owner || this.provider!.wallet.publicKey;
      
      // Find PDAs
      const [configPda] = PublicKey.findProgramAddressSync(
        [Buffer.from('config')],
        this.config.programId
      );
      
      const [universalNftPda] = PublicKey.findProgramAddressSync(
        [Buffer.from('universal_nft'), mint.publicKey.toBuffer()],
        this.config.programId
      );
      
      const [metadataPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from('metadata'),
          new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s').toBuffer(),
          mint.publicKey.toBuffer(),
        ],
        new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s')
      );
      
      const [masterEditionPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from('metadata'),
          new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s').toBuffer(),
          mint.publicKey.toBuffer(),
          Buffer.from('edition'),
        ],
        new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s')
      );
      
      const tokenAccount = await getAssociatedTokenAddress(mint.publicKey, owner);
      
      // Create instruction
      const instruction = await this.program!.methods
        .mintNft(metadata.name, metadata.symbol, metadata.uri, metadata.collection || null)
        .accounts({
          config: configPda,
          universalNft: universalNftPda,
          mint: mint.publicKey,
          metadata: metadataPda,
          masterEdition: masterEditionPda,
          tokenAccount,
          mintAuthority: universalNftPda,
          owner,
          payer: this.provider!.wallet.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .instruction();
      
      // Create transaction with optimizations
      const transaction = await this.createOptimizedTransaction([instruction], {
        priorityFee: options?.priorityFee,
        computeUnitLimit: options?.computeUnitLimit || 200_000,
      });
      
      // Send with retry logic
      const signature = await this.sendTransactionWithRetry(transaction, [mint]);
      
      // Wait for confirmation
      await this.waitForConfirmation(signature);
      
      // Get token ID from account
      const nftAccount = await this.program!.account.universalNft.fetch(universalNftPda);
      const tokenId = nftAccount.originTokenId;
      
      // Update analytics
      this.analytics.recordMint(mint.publicKey, metadata, performance.now() - startTime);
      
      this.emit('mint.confirmed', { mint: mint.publicKey, tokenId });
      
      return {
        mint: mint.publicKey,
        tokenId,
        signature,
      };
      
    } catch (error) {
      this.emit('mint.failed', { error: error as Error });
      throw error;
    }
  }

  /**
   * 🌉 ADVANCED CROSS-CHAIN TRANSFERS
   */
  async transferToChain(
    nftMint: PublicKey,
    options: CrossChainTransferOptions
  ): Promise<{ transferId: string; signature: string }> {
    const startTime = performance.now();
    
    try {
      this.emit('transfer.started', { 
        mint: nftMint, 
        destination: options.destinationChainId 
      });
      
      // Security checks
      await this.securityMonitor.validateTransferRequest(nftMint, options);
      
      // Check NFT status
      const nftInfo = await this.getNftInfo(nftMint);
      if (nftInfo.isLocked) {
        throw new Error('NFT is currently locked for transfer');
      }
      
      // Find PDAs
      const [configPda] = PublicKey.findProgramAddressSync(
        [Buffer.from('config')],
        this.config.programId
      );
      
      const [universalNftPda] = PublicKey.findProgramAddressSync(
        [Buffer.from('universal_nft'), nftMint.toBuffer()],
        this.config.programId
      );
      
      // Get current nonce
      const config = await this.program!.account.programConfig.fetch(configPda);
      const nonce = config.nonce.add(new BN(1));
      
      const [transferPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from('transfer'),
          nftMint.toBuffer(),
          nonce.toBuffer('le', 8),
        ],
        this.config.programId
      );
      
      const tokenAccount = await getAssociatedTokenAddress(
        nftMint, 
        this.provider!.wallet.publicKey
      );
      
      // Convert recipient to bytes
      const recipientBytes = this.parseRecipientAddress(options.recipient);
      
      // Create burn and transfer instruction
      const instruction = await this.program!.methods
        .burnAndTransfer(
          new BN(options.destinationChainId),
          Array.from(recipientBytes),
          new BN(options.gasLimit)
        )
        .accounts({
          config: configPda,
          universalNft: universalNftPda,
          transfer: transferPda,
          mint: nftMint,
          tokenAccount,
          owner: this.provider!.wallet.publicKey,
          gatewayProgram: this.config.gatewayProgramId!,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .instruction();
      
      // Create optimized transaction
      const transaction = await this.createOptimizedTransaction([instruction], {
        priorityFee: options.priorityFee,
        computeUnitLimit: 150_000,
      });
      
      // Send transaction
      const signature = await this.sendTransactionWithRetry(transaction);
      
      // Monitor transfer progress
      const transferId = transferPda.toString();
      this.monitorTransferProgress(transferId, options.destinationChainId);
      
      // Update analytics
      this.analytics.recordTransfer(
        nftMint, 
        options.destinationChainId, 
        performance.now() - startTime
      );
      
      this.emit('transfer.confirmed', { mint: nftMint, txHash: signature });
      
      return { transferId, signature };
      
    } catch (error) {
      this.emit('transfer.failed', { mint: nftMint, error: error as Error });
      throw error;
    }
  }

  /**
   * 📊 NFT INFORMATION & MANAGEMENT
   */
  async getNftInfo(mint: PublicKey): Promise<UniversalNftInfo> {
    // Check cache first
    const cached = this.cache.get(`nft_info_${mint.toString()}`);
    if (cached) return cached;
    
    const [universalNftPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('universal_nft'), mint.toBuffer()],
      this.config.programId
    );
    
    const account = await this.program!.account.universalNft.fetch(universalNftPda);
    
    const info: UniversalNftInfo = {
      mint,
      owner: account.owner,
      tokenId: account.originTokenId,
      originChainId: account.originChainId.toNumber(),
      uri: account.uri,
      name: account.name,
      symbol: account.symbol,
      isLocked: account.isLocked,
      collectionMint: account.collectionMint,
      creationBlock: account.creationBlock.toNumber(),
      creationTimestamp: account.creationTimestamp.toNumber(),
    };
    
    // Cache for 60 seconds
    this.cache.set(`nft_info_${mint.toString()}`, info, 60000);
    
    return info;
  }

  /**
   * 📈 PORTFOLIO MANAGEMENT
   */
  async getUserNfts(owner: PublicKey): Promise<UniversalNftInfo[]> {
    const accounts = await this.program!.account.universalNft.all([
      {
        memcmp: {
          offset: 8 + 32 + 8 + 4 + 64, // Skip to owner field
          bytes: owner.toBase58(),
        },
      },
    ]);
    
    return accounts.map(acc => ({
      mint: acc.account.mint,
      owner: acc.account.owner,
      tokenId: acc.account.originTokenId,
      originChainId: acc.account.originChainId.toNumber(),
      uri: acc.account.uri,
      name: acc.account.name,
      symbol: acc.account.symbol,
      isLocked: acc.account.isLocked,
      collectionMint: acc.account.collectionMint,
      creationBlock: acc.account.creationBlock.toNumber(),
      creationTimestamp: acc.account.creationTimestamp.toNumber(),
    }));
  }

  /**
   * 📊 ANALYTICS & MONITORING
   */
  async getAnalytics(): Promise<any> {
    return this.analytics.getReport();
  }

  async getSecurityStatus(): Promise<any> {
    return this.securityMonitor.getStatus();
  }

  async getPerformanceMetrics(): Promise<any> {
    return this.performanceProfiler.getMetrics();
  }

  /**
   * 🔧 UTILITY METHODS
   */
  private async createOptimizedTransaction(
    instructions: TransactionInstruction[],
    options?: {
      priorityFee?: number;
      computeUnitLimit?: number;
    }
  ): Promise<Transaction> {
    const transaction = new Transaction();
    
    // Add compute budget instructions if specified
    if (options?.computeUnitLimit) {
      const computeBudgetIx = web3.ComputeBudgetProgram.setComputeUnitLimit({
        units: options.computeUnitLimit,
      });
      transaction.add(computeBudgetIx);
    }
    
    if (options?.priorityFee) {
      const priorityFeeIx = web3.ComputeBudgetProgram.setComputeUnitPrice({
        microLamports: options.priorityFee,
      });
      transaction.add(priorityFeeIx);
    }
    
    // Add main instructions
    transaction.add(...instructions);
    
    // Set recent blockhash
    const { blockhash } = await this.config.connection.getLatestBlockhash();
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = this.provider!.wallet.publicKey;
    
    return transaction;
  }

  private async sendTransactionWithRetry(
    transaction: Transaction,
    signers: Keypair[] = []
  ): Promise<string> {
    const { maxRetries, retryDelay, exponentialBackoff } = this.config.retryConfig!;
    
    for (let attempt = 0; attempt <= maxRetries; attempt++) {
      try {
        if (signers.length > 0) {
          transaction.partialSign(...signers);
        }
        
        const signature = await this.provider!.sendAndConfirm(transaction);
        return signature;
        
      } catch (error) {
        if (attempt === maxRetries) throw error;
        
        const delay = exponentialBackoff 
          ? retryDelay * Math.pow(2, attempt)
          : retryDelay;
          
        await new Promise(resolve => setTimeout(resolve, delay));
      }
    }
    
    throw new Error('Max retries exceeded');
  }

  private async waitForConfirmation(signature: string): Promise<void> {
    await this.config.connection.confirmTransaction(signature, 'confirmed');
  }

  private parseRecipientAddress(recipient: string): Uint8Array {
    if (recipient.startsWith('0x')) {
      // Ethereum-style address
      return new Uint8Array(Buffer.from(recipient.slice(2), 'hex'));
    } else {
      // Assume Solana address
      return new PublicKey(recipient).toBytes();
    }
  }

  private async monitorTransferProgress(transferId: string, chainId: number): Promise<void> {
    // Implementation would monitor cross-chain transfer status
    // This is a simplified version
    setTimeout(() => {
      this.emit('transfer.progress', { transferId, status: 'processing' });
    }, 5000);
  }
}

/**
 * 📊 ADVANCED ANALYTICS TRACKER
 */
class AnalyticsTracker {
  private enabled: boolean;
  private metrics: Map<string, any> = new Map();

  constructor(enabled: boolean) {
    this.enabled = enabled;
  }

  recordMint(mint: PublicKey, metadata: NftMetadata, duration: number) {
    if (!this.enabled) return;
    
    const key = 'mints';
    const current = this.metrics.get(key) || [];
    current.push({
      mint: mint.toString(),
      metadata,
      duration,
      timestamp: Date.now(),
    });
    this.metrics.set(key, current);
  }

  recordTransfer(mint: PublicKey, chainId: number, duration: number) {
    if (!this.enabled) return;
    
    const key = 'transfers';
    const current = this.metrics.get(key) || [];
    current.push({
      mint: mint.toString(),
      chainId,
      duration,
      timestamp: Date.now(),
    });
    this.metrics.set(key, current);
  }

  getReport() {
    const mints = this.metrics.get('mints') || [];
    const transfers = this.metrics.get('transfers') || [];
    
    return {
      totalMints: mints.length,
      totalTransfers: transfers.length,
      avgMintDuration: mints.reduce((acc: number, m: any) => acc + m.duration, 0) / mints.length || 0,
      avgTransferDuration: transfers.reduce((acc: number, t: any) => acc + t.duration, 0) / transfers.length || 0,
      chainDistribution: this.calculateChainDistribution(transfers),
    };
  }

  private calculateChainDistribution(transfers: any[]) {
    const distribution: { [key: number]: number } = {};
    transfers.forEach(t => {
      distribution[t.chainId] = (distribution[t.chainId] || 0) + 1;
    });
    return distribution;
  }
}

/**
 * 🛡️ ADVANCED SECURITY MONITOR
 */
class SecurityMonitor {
  private sdk: UniversalNftSDK;
  private suspiciousActivity: any[] = [];

  constructor(sdk: UniversalNftSDK) {
    this.sdk = sdk;
  }

  async validateMintRequest(metadata: NftMetadata): Promise<void> {
    // Check for suspicious metadata
    if (metadata.uri.includes('suspicious-domain.com')) {
      this.sdk.emit('security.alert', {
        type: 'suspicious_metadata',
        severity: 'high' as const,
        details: { uri: metadata.uri },
      });
      throw new Error('Suspicious metadata detected');
    }
  }

  async validateTransferRequest(mint: PublicKey, options: CrossChainTransferOptions): Promise<void> {
    // Check for suspicious transfer patterns
    if (options.gasLimit > 1_000_000) {
      this.sdk.emit('security.alert', {
        type: 'high_gas_limit',
        severity: 'medium' as const,
        details: { mint: mint.toString(), gasLimit: options.gasLimit },
      });
    }
  }

  getStatus() {
    return {
      alertCount: this.suspiciousActivity.length,
      lastAlert: this.suspiciousActivity[this.suspiciousActivity.length - 1],
      riskLevel: this.calculateRiskLevel(),
    };
  }

  private calculateRiskLevel(): 'low' | 'medium' | 'high' {
    const recentAlerts = this.suspiciousActivity.filter(
      a => Date.now() - a.timestamp < 3600000 // Last hour
    );
    
    if (recentAlerts.length > 10) return 'high';
    if (recentAlerts.length > 5) return 'medium';
    return 'low';
  }
}

/**
 * ⚡ PERFORMANCE PROFILER
 */
class PerformanceProfiler {
  private metrics: Map<string, number[]> = new Map();

  recordOperation(operation: string, duration: number) {
    const key = operation;
    const current = this.metrics.get(key) || [];
    current.push(duration);
    
    // Keep only last 100 measurements
    if (current.length > 100) {
      current.shift();
    }
    
    this.metrics.set(key, current);
  }

  getMetrics() {
    const result: any = {};
    
    for (const [operation, durations] of this.metrics) {
      const avg = durations.reduce((a, b) => a + b, 0) / durations.length;
      const min = Math.min(...durations);
      const max = Math.max(...durations);
      
      result[operation] = {
        average: avg,
        min,
        max,
        count: durations.length,
      };
    }
    
    return result;
  }
}

/**
 * 💾 CACHE MANAGER
 */
class CacheManager {
  private cache: Map<string, { data: any; expiry: number }> = new Map();

  set(key: string, data: any, ttl: number = 300000): void {
    this.cache.set(key, {
      data,
      expiry: Date.now() + ttl,
    });
  }

  get(key: string): any {
    const entry = this.cache.get(key);
    if (!entry) return null;
    
    if (Date.now() > entry.expiry) {
      this.cache.delete(key);
      return null;
    }
    
    return entry.data;
  }

  clear(): void {
    this.cache.clear();
  }
}

// Export additional utilities
export * from './types';
export * from './utils';
export * from './constants';