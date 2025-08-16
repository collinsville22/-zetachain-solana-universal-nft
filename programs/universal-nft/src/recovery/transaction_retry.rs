use anchor_lang::prelude::*;
use crate::errors::UniversalNftError;

/// Advanced Transaction Retry System with Intelligent Backoff
/// Handles failed transactions with sophisticated retry logic and optimization
#[account]
#[derive(InitSpace)]
pub struct TransactionRetryManager {
    /// Retry manager authority
    pub authority: Pubkey,
    /// Total retry attempts across all transactions
    pub total_retry_attempts: u64,
    /// Successful retries
    pub successful_retries: u64,
    /// Failed retries (exhausted all attempts)
    pub failed_retries: u64,
    /// Currently active retry sessions
    pub active_retry_sessions: u16,
    /// Maximum concurrent retry sessions
    pub max_concurrent_sessions: u16,
    /// Default retry configuration
    pub default_config: RetryConfig,
    /// Adaptive retry enabled (learns from patterns)
    pub adaptive_retry_enabled: bool,
    /// Last retry attempt timestamp
    pub last_retry_attempt: i64,
    /// PDA bump
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct RetrySession {
    /// Session ID
    pub session_id: u64,
    /// Original transaction that failed
    pub original_tx_signature: String,
    /// Retry configuration for this session
    pub retry_config: RetryConfig,
    /// Current attempt number
    pub current_attempt: u8,
    /// Session status
    pub status: RetrySessionStatus,
    /// Failure reasons encountered
    pub failure_reasons: Vec<RetryFailureReason>,
    /// Session start timestamp
    pub started_at: i64,
    /// Last attempt timestamp
    pub last_attempt_at: i64,
    /// Next retry scheduled timestamp
    pub next_retry_at: i64,
    /// Total time spent on retries
    pub total_retry_time: u64,
    /// Compute units consumed across all attempts
    pub total_compute_units: u64,
    /// Fees spent on retry attempts
    pub total_fees_spent: u64,
    /// Final successful transaction signature
    pub successful_tx_signature: Option<String>,
    /// Optimization applied during retries
    pub optimizations_applied: Vec<RetryOptimization>,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u8,
    /// Initial delay between attempts (seconds)
    pub initial_delay_seconds: u16,
    /// Backoff multiplier (basis points, 10000 = 1.0x)
    pub backoff_multiplier_bps: u16,
    /// Maximum delay between attempts (seconds)
    pub max_delay_seconds: u32,
    /// Jitter percentage to add randomness (basis points)
    pub jitter_percentage_bps: u16,
    /// Compute unit adjustment per attempt (percentage)
    pub compute_unit_adjustment_pct: i16,
    /// Priority fee adjustment per attempt (percentage)
    pub priority_fee_adjustment_pct: i16,
    /// Enable adaptive adjustments based on network conditions
    pub adaptive_adjustments: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum RetrySessionStatus {
    Scheduled,
    InProgress,
    Successful,
    Failed,
    Cancelled,
    Paused,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum RetryFailureReason {
    NetworkTimeout,
    InsufficientComputeUnits,
    InsufficientPriorityFee,
    BlockhashExpired,
    AccountNotFound,
    InsufficientFunds,
    SimulationFailed,
    NodeOverloaded,
    UnknownError,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RetryOptimization {
    /// Optimization type applied
    pub optimization_type: OptimizationType,
    /// Value before optimization
    pub before_value: u64,
    /// Value after optimization
    pub after_value: u64,
    /// Attempt number when applied
    pub applied_at_attempt: u8,
    /// Whether optimization was successful
    pub was_successful: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum OptimizationType {
    ComputeUnitIncrease,
    ComputeUnitDecrease,
    PriorityFeeIncrease,
    PriorityFeeDecrease,
    BlockhashRefresh,
    AccountPreloading,
    InstructionBatching,
    EndpointSwitch,
}

/// Network condition analyzer for adaptive retry logic
pub struct NetworkConditionAnalyzer;

impl NetworkConditionAnalyzer {
    /// Analyze current network conditions
    pub fn analyze_conditions() -> NetworkConditions {
        // In real implementation, would query actual network metrics
        NetworkConditions {
            congestion_level: CongestionLevel::Medium,
            average_confirmation_time_ms: 2500,
            current_base_fee: 5000,
            suggested_priority_fee: 10000,
            recommended_compute_units: 200_000,
            network_stability_score: 85, // 0-100
        }
    }

    /// Calculate optimal retry parameters based on network conditions
    pub fn calculate_optimal_parameters(
        conditions: &NetworkConditions,
        failure_reason: &RetryFailureReason,
        attempt_number: u8,
    ) -> RetryParameters {
        let base_delay = match conditions.congestion_level {
            CongestionLevel::Low => 2,
            CongestionLevel::Medium => 5,
            CongestionLevel::High => 10,
            CongestionLevel::Critical => 20,
        };

        let delay_multiplier = match failure_reason {
            RetryFailureReason::NetworkTimeout => 2.0,
            RetryFailureReason::NodeOverloaded => 3.0,
            RetryFailureReason::BlockhashExpired => 0.5,
            _ => 1.5,
        };

        let compute_adjustment = match failure_reason {
            RetryFailureReason::InsufficientComputeUnits => 25, // +25%
            RetryFailureReason::SimulationFailed => 15,         // +15%
            _ => 0,
        };

        let priority_fee_adjustment = match failure_reason {
            RetryFailureReason::InsufficientPriorityFee => 50, // +50%
            RetryFailureReason::NodeOverloaded => 100,         // +100%
            _ => 10, // +10% default
        };

        RetryParameters {
            delay_seconds: ((base_delay as f64 * delay_multiplier) * (attempt_number as f64).powf(1.5)) as u32,
            compute_unit_adjustment_pct: compute_adjustment,
            priority_fee_adjustment_pct: priority_fee_adjustment,
            should_refresh_blockhash: failure_reason == &RetryFailureReason::BlockhashExpired,
            should_switch_endpoint: conditions.network_stability_score < 50,
        }
    }
}

#[derive(Clone)]
pub struct NetworkConditions {
    pub congestion_level: CongestionLevel,
    pub average_confirmation_time_ms: u32,
    pub current_base_fee: u64,
    pub suggested_priority_fee: u64,
    pub recommended_compute_units: u32,
    pub network_stability_score: u8,
}

#[derive(Clone, PartialEq)]
pub enum CongestionLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone)]
pub struct RetryParameters {
    pub delay_seconds: u32,
    pub compute_unit_adjustment_pct: i16,
    pub priority_fee_adjustment_pct: i16,
    pub should_refresh_blockhash: bool,
    pub should_switch_endpoint: bool,
}

impl TransactionRetryManager {
    pub const INIT_SPACE: usize = 
        32 +    // authority
        8 +     // total_retry_attempts
        8 +     // successful_retries
        8 +     // failed_retries
        2 +     // active_retry_sessions
        2 +     // max_concurrent_sessions
        32 +    // default_config (estimated)
        1 +     // adaptive_retry_enabled
        8 +     // last_retry_attempt
        1;      // bump

    /// Initialize transaction retry manager
    pub fn initialize(
        &mut self,
        authority: Pubkey,
        config: RetryConfig,
        bump: u8,
    ) -> Result<()> {
        self.authority = authority;
        self.total_retry_attempts = 0;
        self.successful_retries = 0;
        self.failed_retries = 0;
        self.active_retry_sessions = 0;
        self.max_concurrent_sessions = 20; // Allow up to 20 concurrent retry sessions
        self.default_config = config;
        self.adaptive_retry_enabled = true;
        self.last_retry_attempt = 0;
        self.bump = bump;

        msg!("Transaction retry manager initialized");
        msg!("Max concurrent sessions: {}", self.max_concurrent_sessions);
        msg!("Adaptive retry enabled: {}", self.adaptive_retry_enabled);

        Ok(())
    }

    /// Schedule a transaction for retry
    pub fn schedule_retry(
        &mut self,
        session: &mut RetrySession,
        session_id: u64,
        original_tx_signature: String,
        failure_reason: RetryFailureReason,
        custom_config: Option<RetryConfig>,
    ) -> Result<()> {
        require!(
            self.active_retry_sessions < self.max_concurrent_sessions,
            UniversalNftError::InvalidTransferStatus
        );

        let now = Clock::get()?.unix_timestamp;
        let config = custom_config.unwrap_or(self.default_config.clone());

        // Calculate initial retry delay
        let initial_delay = if self.adaptive_retry_enabled {
            let conditions = NetworkConditionAnalyzer::analyze_conditions();
            let params = NetworkConditionAnalyzer::calculate_optimal_parameters(
                &conditions, &failure_reason, 1
            );
            params.delay_seconds as i64
        } else {
            config.initial_delay_seconds as i64
        };

        // Initialize retry session
        session.session_id = session_id;
        session.original_tx_signature = original_tx_signature.clone();
        session.retry_config = config;
        session.current_attempt = 0;
        session.status = RetrySessionStatus::Scheduled;
        session.failure_reasons = vec![failure_reason];
        session.started_at = now;
        session.last_attempt_at = 0;
        session.next_retry_at = now + initial_delay;
        session.total_retry_time = 0;
        session.total_compute_units = 0;
        session.total_fees_spent = 0;
        session.successful_tx_signature = None;
        session.optimizations_applied = Vec::new();

        // Update manager state
        self.active_retry_sessions = self.active_retry_sessions.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        msg!("Retry session {} scheduled for transaction: {}", 
             session_id, original_tx_signature);
        msg!("Initial retry scheduled for: {}", session.next_retry_at);

        Ok(())
    }

    /// Execute a retry attempt
    pub fn execute_retry_attempt(
        &mut self,
        session: &mut RetrySession,
    ) -> Result<RetryAttemptResult> {
        let now = Clock::get()?.unix_timestamp;
        
        require!(session.status == RetrySessionStatus::Scheduled, UniversalNftError::InvalidTransferStatus);
        require!(now >= session.next_retry_at, UniversalNftError::InvalidTransferStatus);
        require!(session.current_attempt < session.retry_config.max_attempts, UniversalNftError::InvalidTransferStatus);

        session.current_attempt = session.current_attempt.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        session.status = RetrySessionStatus::InProgress;
        session.last_attempt_at = now;

        // Update global retry statistics
        self.total_retry_attempts = self.total_retry_attempts.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        self.last_retry_attempt = now;

        msg!("Executing retry attempt {} for session {}", 
             session.current_attempt, session.session_id);

        // Simulate retry attempt (in real implementation, would execute actual transaction)
        let attempt_result = self.simulate_retry_attempt(session)?;

        // Update session based on result
        match attempt_result.result {
            AttemptResult::Success => {
                session.status = RetrySessionStatus::Successful;
                session.successful_tx_signature = Some(attempt_result.tx_signature.clone());
                self.successful_retries = self.successful_retries.checked_add(1)
                    .ok_or(UniversalNftError::ArithmeticOverflow)?;
                self.active_retry_sessions = self.active_retry_sessions.saturating_sub(1);
                
                msg!("Retry session {} successful after {} attempts", 
                     session.session_id, session.current_attempt);
            }
            AttemptResult::Failed => {
                if let Some(reason) = attempt_result.failure_reason {
                    session.failure_reasons.push(reason.clone());
                }

                if session.current_attempt >= session.retry_config.max_attempts {
                    session.status = RetrySessionStatus::Failed;
                    self.failed_retries = self.failed_retries.checked_add(1)
                        .ok_or(UniversalNftError::ArithmeticOverflow)?;
                    self.active_retry_sessions = self.active_retry_sessions.saturating_sub(1);
                    
                    msg!("Retry session {} failed after {} attempts", 
                         session.session_id, session.current_attempt);
                } else {
                    // Schedule next retry attempt
                    self.schedule_next_retry(session, &attempt_result.failure_reason)?;
                }
            }
        }

        // Update session metrics
        session.total_compute_units = session.total_compute_units.checked_add(attempt_result.compute_units_used as u64)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        session.total_fees_spent = session.total_fees_spent.checked_add(attempt_result.fees_spent)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        if let Some(optimization) = attempt_result.optimization_applied {
            session.optimizations_applied.push(optimization);
        }

        Ok(attempt_result)
    }

    /// Schedule the next retry attempt
    fn schedule_next_retry(
        &mut self,
        session: &mut RetrySession,
        failure_reason: &Option<RetryFailureReason>,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        
        let delay = if self.adaptive_retry_enabled && failure_reason.is_some() {
            let conditions = NetworkConditionAnalyzer::analyze_conditions();
            let params = NetworkConditionAnalyzer::calculate_optimal_parameters(
                &conditions, failure_reason.as_ref().unwrap(), session.current_attempt
            );
            params.delay_seconds as i64
        } else {
            self.calculate_exponential_backoff_delay(session)
        };

        session.next_retry_at = now + delay;
        session.status = RetrySessionStatus::Scheduled;

        msg!("Next retry for session {} scheduled at {}", 
             session.session_id, session.next_retry_at);

        Ok(())
    }

    /// Calculate exponential backoff delay with jitter
    fn calculate_exponential_backoff_delay(&self, session: &RetrySession) -> i64 {
        let base_delay = session.retry_config.initial_delay_seconds as f64;
        let multiplier = session.retry_config.backoff_multiplier_bps as f64 / 10000.0;
        let attempt = session.current_attempt as f64;
        
        let exponential_delay = base_delay * multiplier.powf(attempt - 1.0);
        let max_delay = session.retry_config.max_delay_seconds as f64;
        let capped_delay = exponential_delay.min(max_delay);
        
        // Add jitter to prevent thundering herd
        let jitter_range = capped_delay * (session.retry_config.jitter_percentage_bps as f64 / 10000.0);
        let jitter = (session.session_id % 1000) as f64 / 1000.0 * jitter_range;
        
        (capped_delay + jitter) as i64
    }

    /// Simulate a retry attempt (replace with actual implementation)
    fn simulate_retry_attempt(&self, session: &RetrySession) -> Result<RetryAttemptResult> {
        // Simulate network conditions and retry logic
        let success_probability = match session.current_attempt {
            1 => 0.3, // 30% success on first retry
            2 => 0.6, // 60% success on second retry
            3 => 0.8, // 80% success on third retry
            _ => 0.9, // 90% success on subsequent retries
        };

        let random_factor = (session.session_id % 100) as f64 / 100.0;
        let is_successful = random_factor < success_probability;

        let result = if is_successful {
            RetryAttemptResult {
                result: AttemptResult::Success,
                tx_signature: format!("retry_success_{}", session.session_id),
                failure_reason: None,
                compute_units_used: 180_000,
                fees_spent: 5000,
                optimization_applied: Some(RetryOptimization {
                    optimization_type: OptimizationType::ComputeUnitIncrease,
                    before_value: 150_000,
                    after_value: 180_000,
                    applied_at_attempt: session.current_attempt,
                    was_successful: true,
                }),
            }
        } else {
            let failure_reasons = [
                RetryFailureReason::NetworkTimeout,
                RetryFailureReason::InsufficientComputeUnits,
                RetryFailureReason::NodeOverloaded,
                RetryFailureReason::BlockhashExpired,
            ];
            let failure_idx = (session.session_id + session.current_attempt as u64) % failure_reasons.len() as u64;
            
            RetryAttemptResult {
                result: AttemptResult::Failed,
                tx_signature: String::new(),
                failure_reason: Some(failure_reasons[failure_idx as usize].clone()),
                compute_units_used: 50_000, // Partial execution
                fees_spent: 2000,
                optimization_applied: None,
            }
        };

        Ok(result)
    }

    /// Cancel a retry session
    pub fn cancel_retry_session(&mut self, session: &mut RetrySession) -> Result<()> {
        require!(
            session.status == RetrySessionStatus::Scheduled || 
            session.status == RetrySessionStatus::Paused,
            UniversalNftError::InvalidTransferStatus
        );

        session.status = RetrySessionStatus::Cancelled;
        self.active_retry_sessions = self.active_retry_sessions.saturating_sub(1);

        msg!("Retry session {} cancelled", session.session_id);
        Ok(())
    }

    /// Update retry configuration
    pub fn update_retry_config(&mut self, new_config: RetryConfig) -> Result<()> {
        self.default_config = new_config;
        msg!("Retry configuration updated");
        Ok(())
    }

    /// Get retry statistics
    pub fn get_retry_stats(&self) -> RetryStats {
        let total_attempts = self.successful_retries + self.failed_retries;
        let success_rate = if total_attempts > 0 {
            (self.successful_retries * 10000) / total_attempts
        } else {
            10000
        };

        RetryStats {
            total_retry_attempts: self.total_retry_attempts,
            successful_retries: self.successful_retries,
            failed_retries: self.failed_retries,
            active_sessions: self.active_retry_sessions,
            success_rate_bps: success_rate as u16,
            adaptive_retry_enabled: self.adaptive_retry_enabled,
            max_concurrent_sessions: self.max_concurrent_sessions,
            last_retry_attempt: self.last_retry_attempt,
        }
    }
}

#[derive(Clone)]
pub struct RetryAttemptResult {
    pub result: AttemptResult,
    pub tx_signature: String,
    pub failure_reason: Option<RetryFailureReason>,
    pub compute_units_used: u32,
    pub fees_spent: u64,
    pub optimization_applied: Option<RetryOptimization>,
}

#[derive(Clone, PartialEq)]
pub enum AttemptResult {
    Success,
    Failed,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 5,
            initial_delay_seconds: 2,
            backoff_multiplier_bps: 20000, // 2.0x
            max_delay_seconds: 60,
            jitter_percentage_bps: 1000, // 10%
            compute_unit_adjustment_pct: 20,
            priority_fee_adjustment_pct: 50,
            adaptive_adjustments: true,
        }
    }
}

#[derive(Clone)]
pub struct RetryStats {
    pub total_retry_attempts: u64,
    pub successful_retries: u64,
    pub failed_retries: u64,
    pub active_sessions: u16,
    pub success_rate_bps: u16,
    pub adaptive_retry_enabled: bool,
    pub max_concurrent_sessions: u16,
    pub last_retry_attempt: i64,
}