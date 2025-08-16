use anchor_lang::prelude::*;
use crate::errors::UniversalNftError;

/// Real-time Metrics Collection System for Universal NFT Protocol
/// Tracks all critical operations, performance, and usage patterns
#[account]
#[derive(InitSpace)]
pub struct MetricsCollector {
    /// Metrics collection authority
    pub authority: Pubkey,
    /// Total NFTs minted across all chains
    pub total_nfts_minted: u64,
    /// Total cross-chain transfers
    pub total_cross_chain_transfers: u64,
    /// Total successful operations
    pub successful_operations: u64,
    /// Total failed operations
    pub failed_operations: u64,
    /// Total compute units consumed
    pub total_compute_units: u64,
    /// Total fees collected (in lamports)
    pub total_fees_collected: u64,
    /// Active users count (last 30 days)
    pub active_users_30d: u32,
    /// Peak transactions per second achieved
    pub peak_tps: u16,
    /// Average transaction latency (milliseconds)
    pub avg_latency_ms: u32,
    /// Current error rate (basis points)
    pub current_error_rate_bps: u16,
    /// System uptime percentage (basis points)
    pub uptime_percentage_bps: u16,
    /// Last metrics update timestamp
    pub last_updated: i64,
    /// Metrics collection start time
    pub collection_start: i64,
    /// PDA bump
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct OperationMetrics {
    /// Operation type
    pub operation_type: OperationType,
    /// Total executions
    pub total_executions: u64,
    /// Successful executions
    pub successful_executions: u64,
    /// Failed executions
    pub failed_executions: u64,
    /// Average execution time (microseconds)
    pub avg_execution_time_us: u64,
    /// Peak execution time (microseconds)
    pub peak_execution_time_us: u64,
    /// Average compute units used
    pub avg_compute_units: u32,
    /// Peak compute units used
    pub peak_compute_units: u32,
    /// Total gas/fees consumed
    pub total_gas_consumed: u64,
    /// Last execution timestamp
    pub last_execution: i64,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum OperationType {
    MintNFT,
    BurnAndTransfer,
    OnCall,
    Initialize,
    UpdateMetadata,
    VerifySignature,
    FraudDetection,
    CircuitBreakerCheck,
    GovernanceVote,
    TreasuryOperation,
}

#[account]
#[derive(InitSpace)]
pub struct ChainMetrics {
    /// Source or destination chain ID
    pub chain_id: u64,
    /// Chain name
    pub chain_name: String,
    /// Total transfers to this chain
    pub transfers_to: u64,
    /// Total transfers from this chain
    pub transfers_from: u64,
    /// Average transfer time (seconds)
    pub avg_transfer_time_s: u32,
    /// Peak transfer time (seconds)
    pub peak_transfer_time_s: u32,
    /// Failed transfers count
    pub failed_transfers: u64,
    /// Total value transferred (in normalized units)
    pub total_value_transferred: u64,
    /// Last transfer timestamp
    pub last_transfer: i64,
    /// Chain status
    pub status: ChainStatus,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ChainStatus {
    Active,
    Degraded,
    Inactive,
    Maintenance,
}

#[account]
#[derive(InitSpace)]
pub struct UserMetrics {
    /// User public key
    pub user: Pubkey,
    /// Total NFTs minted by user
    pub nfts_minted: u32,
    /// Total cross-chain transfers initiated
    pub transfers_initiated: u32,
    /// Total transactions count
    pub total_transactions: u64,
    /// Total fees paid (in lamports)
    pub total_fees_paid: u64,
    /// First interaction timestamp
    pub first_interaction: i64,
    /// Last interaction timestamp
    pub last_interaction: i64,
    /// User tier based on activity
    pub user_tier: UserTier,
    /// Reputation score (0-1000)
    pub reputation_score: u16,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum UserTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}

#[account]
#[derive(InitSpace)]
pub struct SecurityMetrics {
    /// Total security events detected
    pub total_security_events: u64,
    /// Critical security events
    pub critical_events: u32,
    /// High severity events
    pub high_severity_events: u32,
    /// Medium severity events
    pub medium_severity_events: u32,
    /// Low severity events
    pub low_severity_events: u32,
    /// Circuit breaker activations
    pub circuit_breaker_activations: u32,
    /// Fraud detection triggers
    pub fraud_detection_triggers: u32,
    /// Suspicious transaction count
    pub suspicious_transactions: u64,
    /// Last security event timestamp
    pub last_security_event: i64,
    /// Current threat level
    pub threat_level: ThreatLevel,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl MetricsCollector {
    pub const INIT_SPACE: usize = 
        32 +    // authority
        8 +     // total_nfts_minted
        8 +     // total_cross_chain_transfers
        8 +     // successful_operations
        8 +     // failed_operations
        8 +     // total_compute_units
        8 +     // total_fees_collected
        4 +     // active_users_30d
        2 +     // peak_tps
        4 +     // avg_latency_ms
        2 +     // current_error_rate_bps
        2 +     // uptime_percentage_bps
        8 +     // last_updated
        8 +     // collection_start
        1;      // bump

    /// Initialize metrics collection
    pub fn initialize(&mut self, authority: Pubkey, bump: u8) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        
        self.authority = authority;
        self.total_nfts_minted = 0;
        self.total_cross_chain_transfers = 0;
        self.successful_operations = 0;
        self.failed_operations = 0;
        self.total_compute_units = 0;
        self.total_fees_collected = 0;
        self.active_users_30d = 0;
        self.peak_tps = 0;
        self.avg_latency_ms = 0;
        self.current_error_rate_bps = 0;
        self.uptime_percentage_bps = 10000; // 100%
        self.last_updated = now;
        self.collection_start = now;
        self.bump = bump;

        msg!("Metrics collection initialized");
        Ok(())
    }

    /// Record an NFT mint operation
    pub fn record_nft_mint(&mut self, compute_units: u32, latency_us: u64, fees: u64) -> Result<()> {
        self.total_nfts_minted = self.total_nfts_minted.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        
        self.successful_operations = self.successful_operations.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        
        self.total_compute_units = self.total_compute_units.checked_add(compute_units as u64)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        
        self.total_fees_collected = self.total_fees_collected.checked_add(fees)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        self.update_latency(latency_us);
        self.last_updated = Clock::get()?.unix_timestamp;

        Ok(())
    }

    /// Record a cross-chain transfer
    pub fn record_cross_chain_transfer(&mut self, compute_units: u32, latency_us: u64, fees: u64) -> Result<()> {
        self.total_cross_chain_transfers = self.total_cross_chain_transfers.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        
        self.successful_operations = self.successful_operations.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        
        self.total_compute_units = self.total_compute_units.checked_add(compute_units as u64)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        
        self.total_fees_collected = self.total_fees_collected.checked_add(fees)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        self.update_latency(latency_us);
        self.last_updated = Clock::get()?.unix_timestamp;

        Ok(())
    }

    /// Record a failed operation
    pub fn record_failed_operation(&mut self, compute_units: u32) -> Result<()> {
        self.failed_operations = self.failed_operations.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        
        self.total_compute_units = self.total_compute_units.checked_add(compute_units as u64)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        self.update_error_rate();
        self.last_updated = Clock::get()?.unix_timestamp;

        Ok(())
    }

    /// Update TPS if current rate is higher
    pub fn update_peak_tps(&mut self, current_tps: u16) -> Result<()> {
        if current_tps > self.peak_tps {
            self.peak_tps = current_tps;
            msg!("New peak TPS recorded: {}", current_tps);
        }
        Ok(())
    }

    /// Update active users count
    pub fn update_active_users(&mut self, count: u32) -> Result<()> {
        self.active_users_30d = count;
        self.last_updated = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// Private helper to update latency
    fn update_latency(&mut self, latency_us: u64) {
        let latency_ms = (latency_us / 1000) as u32;
        
        // Simple moving average (can be improved with more sophisticated algorithms)
        let total_ops = self.successful_operations + self.failed_operations;
        if total_ops > 0 {
            self.avg_latency_ms = ((self.avg_latency_ms as u64 * (total_ops - 1)) + latency_ms as u64) as u32 / total_ops as u32;
        } else {
            self.avg_latency_ms = latency_ms;
        }
    }

    /// Private helper to update error rate
    fn update_error_rate(&mut self) {
        let total_ops = self.successful_operations + self.failed_operations;
        if total_ops > 0 {
            self.current_error_rate_bps = ((self.failed_operations * 10000) / total_ops) as u16;
        }
    }

    /// Get system health score (0-100)
    pub fn get_health_score(&self) -> u8 {
        let mut score = 100u8;

        // Deduct points for high error rate
        if self.current_error_rate_bps > 500 { // > 5%
            score = score.saturating_sub(30);
        } else if self.current_error_rate_bps > 100 { // > 1%
            score = score.saturating_sub(10);
        }

        // Deduct points for high latency
        if self.avg_latency_ms > 5000 { // > 5 seconds
            score = score.saturating_sub(20);
        } else if self.avg_latency_ms > 2000 { // > 2 seconds
            score = score.saturating_sub(10);
        }

        // Deduct points for low uptime
        if self.uptime_percentage_bps < 9500 { // < 95%
            score = score.saturating_sub(25);
        } else if self.uptime_percentage_bps < 9900 { // < 99%
            score = score.saturating_sub(10);
        }

        score
    }

    /// Get comprehensive metrics summary
    pub fn get_metrics_summary(&self) -> MetricsSummary {
        let total_ops = self.successful_operations + self.failed_operations;
        let success_rate = if total_ops > 0 {
            (self.successful_operations * 10000) / total_ops
        } else {
            10000
        };

        let now = Clock::get().unwrap().unix_timestamp;
        let uptime_hours = ((now - self.collection_start) / 3600).max(1);

        MetricsSummary {
            total_operations: total_ops,
            success_rate_bps: success_rate as u16,
            error_rate_bps: self.current_error_rate_bps,
            avg_latency_ms: self.avg_latency_ms,
            peak_tps: self.peak_tps,
            total_nfts: self.total_nfts_minted,
            total_transfers: self.total_cross_chain_transfers,
            total_fees: self.total_fees_collected,
            active_users: self.active_users_30d,
            uptime_percentage: self.uptime_percentage_bps,
            health_score: self.get_health_score(),
            collection_duration_hours: uptime_hours,
        }
    }
}

#[derive(Clone)]
pub struct MetricsSummary {
    pub total_operations: u64,
    pub success_rate_bps: u16,
    pub error_rate_bps: u16,
    pub avg_latency_ms: u32,
    pub peak_tps: u16,
    pub total_nfts: u64,
    pub total_transfers: u64,
    pub total_fees: u64,
    pub active_users: u32,
    pub uptime_percentage: u16,
    pub health_score: u8,
    pub collection_duration_hours: i64,
}

impl OperationMetrics {
    pub const INIT_SPACE: usize = 
        1 +     // operation_type (enum)
        8 +     // total_executions
        8 +     // successful_executions
        8 +     // failed_executions
        8 +     // avg_execution_time_us
        8 +     // peak_execution_time_us
        4 +     // avg_compute_units
        4 +     // peak_compute_units
        8 +     // total_gas_consumed
        8 +     // last_execution
        1;      // bump

    pub fn initialize(&mut self, operation_type: OperationType, bump: u8) {
        self.operation_type = operation_type;
        self.total_executions = 0;
        self.successful_executions = 0;
        self.failed_executions = 0;
        self.avg_execution_time_us = 0;
        self.peak_execution_time_us = 0;
        self.avg_compute_units = 0;
        self.peak_compute_units = 0;
        self.total_gas_consumed = 0;
        self.last_execution = 0;
        self.bump = bump;
    }

    pub fn record_execution(
        &mut self, 
        success: bool, 
        execution_time_us: u64, 
        compute_units: u32, 
        gas_consumed: u64
    ) -> Result<()> {
        self.total_executions = self.total_executions.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        if success {
            self.successful_executions = self.successful_executions.checked_add(1)
                .ok_or(UniversalNftError::ArithmeticOverflow)?;
        } else {
            self.failed_executions = self.failed_executions.checked_add(1)
                .ok_or(UniversalNftError::ArithmeticOverflow)?;
        }

        // Update timing metrics
        if execution_time_us > self.peak_execution_time_us {
            self.peak_execution_time_us = execution_time_us;
        }

        self.avg_execution_time_us = ((self.avg_execution_time_us * (self.total_executions - 1)) + execution_time_us) / self.total_executions;

        // Update compute metrics
        if compute_units > self.peak_compute_units {
            self.peak_compute_units = compute_units;
        }

        self.avg_compute_units = ((self.avg_compute_units * (self.total_executions as u32 - 1)) + compute_units) / self.total_executions as u32;

        self.total_gas_consumed = self.total_gas_consumed.checked_add(gas_consumed)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        self.last_execution = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn get_success_rate(&self) -> u16 {
        if self.total_executions > 0 {
            ((self.successful_executions * 10000) / self.total_executions) as u16
        } else {
            10000
        }
    }
}

impl UserMetrics {
    pub const INIT_SPACE: usize = 
        32 +    // user
        4 +     // nfts_minted
        4 +     // transfers_initiated
        8 +     // total_transactions
        8 +     // total_fees_paid
        8 +     // first_interaction
        8 +     // last_interaction
        1 +     // user_tier (enum)
        2 +     // reputation_score
        1;      // bump

    pub fn initialize(&mut self, user: Pubkey, bump: u8) {
        let now = Clock::get().unwrap().unix_timestamp;
        
        self.user = user;
        self.nfts_minted = 0;
        self.transfers_initiated = 0;
        self.total_transactions = 0;
        self.total_fees_paid = 0;
        self.first_interaction = now;
        self.last_interaction = now;
        self.user_tier = UserTier::Bronze;
        self.reputation_score = 100; // Start with base reputation
        self.bump = bump;
    }

    pub fn record_transaction(&mut self, fees_paid: u64) -> Result<()> {
        self.total_transactions = self.total_transactions.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        
        self.total_fees_paid = self.total_fees_paid.checked_add(fees_paid)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        
        self.last_interaction = Clock::get()?.unix_timestamp;
        self.update_tier();

        Ok(())
    }

    pub fn record_nft_mint(&mut self) -> Result<()> {
        self.nfts_minted = self.nfts_minted.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        
        self.update_reputation(5); // Mint adds reputation
        Ok(())
    }

    pub fn record_transfer(&mut self) -> Result<()> {
        self.transfers_initiated = self.transfers_initiated.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        
        self.update_reputation(3); // Transfer adds reputation
        Ok(())
    }

    fn update_tier(&mut self) {
        self.user_tier = match self.total_transactions {
            0..=9 => UserTier::Bronze,
            10..=49 => UserTier::Silver,
            50..=199 => UserTier::Gold,
            200..=999 => UserTier::Platinum,
            _ => UserTier::Diamond,
        };
    }

    fn update_reputation(&mut self, points: u16) {
        self.reputation_score = (self.reputation_score + points).min(1000);
    }
}