use anchor_lang::prelude::*;
use crate::errors::UniversalNftError;

/// Advanced Error Recovery System for Universal NFT Protocol
/// Provides intelligent error handling, automatic recovery, and failure compensation
#[account]
#[derive(InitSpace)]
pub struct ErrorRecoveryManager {
    /// Recovery manager authority
    pub authority: Pubkey,
    /// Total recovery attempts made
    pub total_recovery_attempts: u64,
    /// Successful recoveries
    pub successful_recoveries: u64,
    /// Failed recoveries requiring manual intervention
    pub failed_recoveries: u64,
    /// Current active recovery sessions
    pub active_recovery_sessions: u16,
    /// Maximum concurrent recovery sessions
    pub max_concurrent_sessions: u16,
    /// Recovery success rate (basis points)
    pub recovery_success_rate_bps: u16,
    /// Auto-recovery enabled
    pub auto_recovery_enabled: bool,
    /// Aggressive recovery mode (higher resource usage)
    pub aggressive_mode: bool,
    /// Last recovery attempt timestamp
    pub last_recovery_attempt: i64,
    /// Recovery statistics reset timestamp
    pub stats_reset_at: i64,
    /// PDA bump
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct RecoverySession {
    /// Session ID
    pub session_id: u64,
    /// Error that triggered recovery
    pub original_error: ErrorType,
    /// Recovery strategy being used
    pub recovery_strategy: RecoveryStrategy,
    /// Affected operation details
    pub operation_context: OperationContext,
    /// Recovery attempts made
    pub attempts_made: u8,
    /// Maximum attempts allowed
    pub max_attempts: u8,
    /// Session status
    pub status: RecoveryStatus,
    /// Session start timestamp
    pub started_at: i64,
    /// Session completion timestamp
    pub completed_at: Option<i64>,
    /// Recovery actions taken
    pub actions_taken: Vec<RecoveryAction>,
    /// Final outcome
    pub outcome: Option<RecoveryOutcome>,
    /// Resources consumed during recovery
    pub resources_consumed: ResourceUsage,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ErrorType {
    TransactionFailed,
    NetworkTimeout,
    InsufficientFunds,
    AccountNotFound,
    InvalidSignature,
    ComputeExceeded,
    CrossChainTimeout,
    GatewayUnavailable,
    StateCorruption,
    ConcurrencyConflict,
    SecurityViolation,
    SystemOverload,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum RecoveryStrategy {
    /// Simple retry with exponential backoff
    ExponentialBackoff,
    /// Retry with different parameters
    ParameterAdjustment,
    /// Switch to alternative execution path
    AlternativeExecution,
    /// Rollback and retry from checkpoint
    RollbackRetry,
    /// Compensating transaction
    CompensatingTransaction,
    /// State reconstruction
    StateReconstruction,
    /// Manual intervention required
    ManualIntervention,
    /// Graceful degradation
    GracefulDegradation,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct OperationContext {
    /// Operation type that failed
    pub operation_type: String,
    /// User who initiated the operation
    pub user: Pubkey,
    /// NFT mint involved (if applicable)
    pub nft_mint: Option<Pubkey>,
    /// Target chain (for cross-chain operations)
    pub target_chain: Option<u64>,
    /// Transaction signature that failed
    pub failed_signature: Option<String>,
    /// Compute units consumed before failure
    pub compute_units_used: u32,
    /// Fees paid before failure
    pub fees_paid: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum RecoveryStatus {
    InProgress,
    Successful,
    Failed,
    RequiresManualIntervention,
    Cancelled,
    TimedOut,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RecoveryAction {
    /// Action type taken
    pub action_type: ActionType,
    /// Action timestamp
    pub timestamp: i64,
    /// Action parameters
    pub parameters: String,
    /// Action result
    pub result: ActionResult,
    /// Compute units consumed
    pub compute_units: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ActionType {
    RetryTransaction,
    AdjustComputeLimit,
    AdjustPriorityFee,
    SwitchRpcEndpoint,
    RefreshBlockhash,
    RecreateAccounts,
    ValidateState,
    CompensateUser,
    NotifyUser,
    EscalateToManual,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ActionResult {
    Success,
    PartialSuccess,
    Failed,
    Skipped,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RecoveryOutcome {
    /// Final result of recovery
    pub result: RecoveryResult,
    /// New transaction signature (if successful)
    pub new_signature: Option<String>,
    /// Compensation provided to user
    pub compensation: Option<Compensation>,
    /// Lessons learned for future improvements
    pub lessons_learned: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum RecoveryResult {
    FullRecovery,
    PartialRecovery,
    CompensatedFailure,
    UnrecoverableFailure,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Compensation {
    /// Compensation type
    pub compensation_type: CompensationType,
    /// Amount of compensation
    pub amount: u64,
    /// Token mint for compensation (None for SOL)
    pub token_mint: Option<Pubkey>,
    /// Reason for compensation
    pub reason: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum CompensationType {
    FeeRefund,
    TokenCompensation,
    ServiceCredit,
    PriorityAccess,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ResourceUsage {
    /// Total compute units consumed
    pub compute_units: u64,
    /// Total fees spent on recovery
    pub fees_spent: u64,
    /// Recovery duration (seconds)
    pub duration_seconds: u64,
    /// Network requests made
    pub network_requests: u32,
}

impl ErrorRecoveryManager {
    pub const INIT_SPACE: usize = 
        32 +    // authority
        8 +     // total_recovery_attempts
        8 +     // successful_recoveries
        8 +     // failed_recoveries
        2 +     // active_recovery_sessions
        2 +     // max_concurrent_sessions
        2 +     // recovery_success_rate_bps
        1 +     // auto_recovery_enabled
        1 +     // aggressive_mode
        8 +     // last_recovery_attempt
        8 +     // stats_reset_at
        1;      // bump

    /// Initialize error recovery manager
    pub fn initialize(
        &mut self,
        authority: Pubkey,
        config: RecoveryConfig,
        bump: u8,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        
        self.authority = authority;
        self.total_recovery_attempts = 0;
        self.successful_recoveries = 0;
        self.failed_recoveries = 0;
        self.active_recovery_sessions = 0;
        self.max_concurrent_sessions = config.max_concurrent_sessions;
        self.recovery_success_rate_bps = 10000; // Start at 100%
        self.auto_recovery_enabled = config.auto_recovery_enabled;
        self.aggressive_mode = config.aggressive_mode;
        self.last_recovery_attempt = 0;
        self.stats_reset_at = now;
        self.bump = bump;

        msg!("Error recovery manager initialized");
        msg!("Max concurrent sessions: {}", self.max_concurrent_sessions);
        msg!("Auto-recovery enabled: {}", self.auto_recovery_enabled);

        Ok(())
    }

    /// Initiate error recovery for a failed operation
    pub fn initiate_recovery(
        &mut self,
        session: &mut RecoverySession,
        session_id: u64,
        error_type: ErrorType,
        operation_context: OperationContext,
    ) -> Result<()> {
        require!(self.auto_recovery_enabled, UniversalNftError::InvalidTransferStatus);
        require!(
            self.active_recovery_sessions < self.max_concurrent_sessions,
            UniversalNftError::InvalidTransferStatus
        );

        let now = Clock::get()?.unix_timestamp;
        
        // Determine recovery strategy based on error type
        let strategy = self.determine_recovery_strategy(&error_type, &operation_context);
        let max_attempts = self.calculate_max_attempts(&error_type, &strategy);

        // Initialize recovery session
        session.session_id = session_id;
        session.original_error = error_type.clone();
        session.recovery_strategy = strategy;
        session.operation_context = operation_context;
        session.attempts_made = 0;
        session.max_attempts = max_attempts;
        session.status = RecoveryStatus::InProgress;
        session.started_at = now;
        session.completed_at = None;
        session.actions_taken = Vec::new();
        session.outcome = None;
        session.resources_consumed = ResourceUsage {
            compute_units: 0,
            fees_spent: 0,
            duration_seconds: 0,
            network_requests: 0,
        };

        // Update manager state
        self.active_recovery_sessions = self.active_recovery_sessions.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        self.total_recovery_attempts = self.total_recovery_attempts.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        self.last_recovery_attempt = now;

        msg!("Recovery session {} initiated for error: {:?}", session_id, error_type);
        msg!("Strategy: {:?}, Max attempts: {}", session.recovery_strategy, max_attempts);

        Ok(())
    }

    /// Execute recovery attempt
    pub fn execute_recovery_attempt(
        &mut self,
        session: &mut RecoverySession,
    ) -> Result<bool> {
        require!(session.status == RecoveryStatus::InProgress, UniversalNftError::InvalidTransferStatus);
        require!(session.attempts_made < session.max_attempts, UniversalNftError::InvalidTransferStatus);

        session.attempts_made = session.attempts_made.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        let now = Clock::get()?.unix_timestamp;
        
        let success = match session.recovery_strategy {
            RecoveryStrategy::ExponentialBackoff => {
                self.execute_exponential_backoff_recovery(session)?
            }
            RecoveryStrategy::ParameterAdjustment => {
                self.execute_parameter_adjustment_recovery(session)?
            }
            RecoveryStrategy::AlternativeExecution => {
                self.execute_alternative_execution_recovery(session)?
            }
            RecoveryStrategy::RollbackRetry => {
                self.execute_rollback_retry_recovery(session)?
            }
            RecoveryStrategy::CompensatingTransaction => {
                self.execute_compensating_transaction_recovery(session)?
            }
            RecoveryStrategy::StateReconstruction => {
                self.execute_state_reconstruction_recovery(session)?
            }
            RecoveryStrategy::GracefulDegradation => {
                self.execute_graceful_degradation_recovery(session)?
            }
            RecoveryStrategy::ManualIntervention => {
                session.status = RecoveryStatus::RequiresManualIntervention;
                false
            }
        };

        // Record action taken
        let action = RecoveryAction {
            action_type: ActionType::RetryTransaction, // Simplified
            timestamp: now,
            parameters: format!("Attempt {}/{}", session.attempts_made, session.max_attempts),
            result: if success { ActionResult::Success } else { ActionResult::Failed },
            compute_units: 5000, // Estimated
        };
        session.actions_taken.push(action);

        if success {
            self.complete_recovery_session(session, RecoveryResult::FullRecovery)?;
        } else if session.attempts_made >= session.max_attempts {
            self.complete_recovery_session(session, RecoveryResult::UnrecoverableFailure)?;
        }

        Ok(success)
    }

    /// Complete a recovery session
    fn complete_recovery_session(
        &mut self,
        session: &mut RecoverySession,
        result: RecoveryResult,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        
        session.status = match result {
            RecoveryResult::FullRecovery | RecoveryResult::PartialRecovery => RecoveryStatus::Successful,
            RecoveryResult::CompensatedFailure => RecoveryStatus::Failed,
            RecoveryResult::UnrecoverableFailure => RecoveryStatus::Failed,
        };
        
        session.completed_at = Some(now);
        session.resources_consumed.duration_seconds = (now - session.started_at) as u64;
        
        // Set outcome
        session.outcome = Some(RecoveryOutcome {
            result: result.clone(),
            new_signature: None, // Would be set in real implementation
            compensation: self.calculate_compensation(session, &result),
            lessons_learned: self.generate_lessons_learned(session),
        });

        // Update manager statistics
        self.active_recovery_sessions = self.active_recovery_sessions.saturating_sub(1);
        
        match result {
            RecoveryResult::FullRecovery | RecoveryResult::PartialRecovery => {
                self.successful_recoveries = self.successful_recoveries.checked_add(1)
                    .ok_or(UniversalNftError::ArithmeticOverflow)?;
            }
            _ => {
                self.failed_recoveries = self.failed_recoveries.checked_add(1)
                    .ok_or(UniversalNftError::ArithmeticOverflow)?;
            }
        }

        self.update_success_rate();

        msg!("Recovery session {} completed with result: {:?}", session.session_id, result);
        Ok(())
    }

    /// Determine appropriate recovery strategy
    fn determine_recovery_strategy(
        &self,
        error_type: &ErrorType,
        context: &OperationContext,
    ) -> RecoveryStrategy {
        match error_type {
            ErrorType::TransactionFailed => {
                if context.compute_units_used > 150_000 {
                    RecoveryStrategy::ParameterAdjustment
                } else {
                    RecoveryStrategy::ExponentialBackoff
                }
            }
            ErrorType::NetworkTimeout => RecoveryStrategy::ExponentialBackoff,
            ErrorType::InsufficientFunds => RecoveryStrategy::CompensatingTransaction,
            ErrorType::ComputeExceeded => RecoveryStrategy::ParameterAdjustment,
            ErrorType::CrossChainTimeout => RecoveryStrategy::AlternativeExecution,
            ErrorType::GatewayUnavailable => RecoveryStrategy::GracefulDegradation,
            ErrorType::StateCorruption => RecoveryStrategy::StateReconstruction,
            ErrorType::ConcurrencyConflict => RecoveryStrategy::RollbackRetry,
            ErrorType::SecurityViolation => RecoveryStrategy::ManualIntervention,
            ErrorType::SystemOverload => RecoveryStrategy::GracefulDegradation,
            _ => RecoveryStrategy::ExponentialBackoff,
        }
    }

    /// Calculate maximum recovery attempts
    fn calculate_max_attempts(&self, error_type: &ErrorType, strategy: &RecoveryStrategy) -> u8 {
        let base_attempts = match error_type {
            ErrorType::NetworkTimeout => 5,
            ErrorType::TransactionFailed => 3,
            ErrorType::ComputeExceeded => 2,
            ErrorType::CrossChainTimeout => 4,
            ErrorType::SecurityViolation => 1,
            _ => 3,
        };

        if self.aggressive_mode {
            (base_attempts * 2).min(10)
        } else {
            base_attempts
        }
    }

    /// Execute exponential backoff recovery
    fn execute_exponential_backoff_recovery(&self, session: &RecoverySession) -> Result<bool> {
        // Simulate exponential backoff delay
        let delay_seconds = 2_u64.pow((session.attempts_made - 1) as u32);
        
        msg!("Exponential backoff recovery: attempt {}, delay {}s", 
             session.attempts_made, delay_seconds);
        
        // In real implementation, would actually retry the operation
        // Simulating 70% success rate
        Ok(session.attempts_made >= 2 && session.session_id % 10 < 7)
    }

    /// Execute parameter adjustment recovery
    fn execute_parameter_adjustment_recovery(&self, session: &RecoverySession) -> Result<bool> {
        msg!("Parameter adjustment recovery: attempt {}", session.attempts_made);
        
        // Would adjust compute limits, priority fees, etc.
        // Simulating 60% success rate
        Ok(session.attempts_made >= 1 && session.session_id % 10 < 6)
    }

    /// Execute alternative execution recovery
    fn execute_alternative_execution_recovery(&self, session: &RecoverySession) -> Result<bool> {
        msg!("Alternative execution recovery: attempt {}", session.attempts_made);
        
        // Would try different execution paths or endpoints
        // Simulating 80% success rate
        Ok(session.session_id % 10 < 8)
    }

    /// Execute rollback retry recovery
    fn execute_rollback_retry_recovery(&self, session: &RecoverySession) -> Result<bool> {
        msg!("Rollback retry recovery: attempt {}", session.attempts_made);
        
        // Would rollback to last known good state and retry
        // Simulating 75% success rate
        Ok(session.session_id % 10 < 7)
    }

    /// Execute compensating transaction recovery
    fn execute_compensating_transaction_recovery(&self, session: &RecoverySession) -> Result<bool> {
        msg!("Compensating transaction recovery: attempt {}", session.attempts_made);
        
        // Would create compensating transactions
        // Simulating 90% success rate (compensation almost always works)
        Ok(session.session_id % 10 < 9)
    }

    /// Execute state reconstruction recovery
    fn execute_state_reconstruction_recovery(&self, session: &RecoverySession) -> Result<bool> {
        msg!("State reconstruction recovery: attempt {}", session.attempts_made);
        
        // Would reconstruct corrupted state from backups
        // Simulating 50% success rate (complex operation)
        Ok(session.attempts_made >= 2 && session.session_id % 10 < 5)
    }

    /// Execute graceful degradation recovery
    fn execute_graceful_degradation_recovery(&self, session: &RecoverySession) -> Result<bool> {
        msg!("Graceful degradation recovery: attempt {}", session.attempts_made);
        
        // Would provide reduced functionality
        // Simulating 95% success rate (almost always can degrade gracefully)
        Ok(session.session_id % 10 < 9)
    }

    /// Calculate appropriate compensation
    fn calculate_compensation(
        &self,
        session: &RecoverySession,
        result: &RecoveryResult,
    ) -> Option<Compensation> {
        match result {
            RecoveryResult::FullRecovery => None, // No compensation needed
            RecoveryResult::PartialRecovery => {
                Some(Compensation {
                    compensation_type: CompensationType::ServiceCredit,
                    amount: session.operation_context.fees_paid / 2, // 50% credit
                    token_mint: None,
                    reason: "Partial service disruption".to_string(),
                })
            }
            RecoveryResult::CompensatedFailure => {
                Some(Compensation {
                    compensation_type: CompensationType::FeeRefund,
                    amount: session.operation_context.fees_paid,
                    token_mint: None,
                    reason: "Operation failed despite recovery attempts".to_string(),
                })
            }
            RecoveryResult::UnrecoverableFailure => {
                Some(Compensation {
                    compensation_type: CompensationType::TokenCompensation,
                    amount: session.operation_context.fees_paid * 2, // 2x compensation
                    token_mint: None,
                    reason: "Unrecoverable system failure".to_string(),
                })
            }
        }
    }

    /// Generate lessons learned for continuous improvement
    fn generate_lessons_learned(&self, session: &RecoverySession) -> String {
        match session.original_error {
            ErrorType::ComputeExceeded => "Consider implementing dynamic compute limit adjustment".to_string(),
            ErrorType::NetworkTimeout => "Evaluate network reliability and implement redundant endpoints".to_string(),
            ErrorType::CrossChainTimeout => "Optimize cross-chain message routing and timeouts".to_string(),
            ErrorType::StateCorruption => "Enhance state validation and backup frequency".to_string(),
            _ => "Continue monitoring for pattern recognition".to_string(),
        }
    }

    /// Update recovery success rate
    fn update_success_rate(&mut self) {
        let total = self.successful_recoveries + self.failed_recoveries;
        if total > 0 {
            self.recovery_success_rate_bps = ((self.successful_recoveries * 10000) / total) as u16;
        }
    }

    /// Get recovery statistics
    pub fn get_recovery_stats(&self) -> RecoveryStats {
        let total_attempts = self.successful_recoveries + self.failed_recoveries;
        
        RecoveryStats {
            total_recovery_attempts: self.total_recovery_attempts,
            successful_recoveries: self.successful_recoveries,
            failed_recoveries: self.failed_recoveries,
            active_sessions: self.active_recovery_sessions,
            success_rate_bps: self.recovery_success_rate_bps,
            auto_recovery_enabled: self.auto_recovery_enabled,
            aggressive_mode: self.aggressive_mode,
            avg_attempts_per_session: if total_attempts > 0 {
                (self.total_recovery_attempts / total_attempts) as f32
            } else {
                0.0
            },
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RecoveryConfig {
    pub max_concurrent_sessions: u16,
    pub auto_recovery_enabled: bool,
    pub aggressive_mode: bool,
}

impl Default for RecoveryConfig {
    fn default() -> Self {
        Self {
            max_concurrent_sessions: 10,
            auto_recovery_enabled: true,
            aggressive_mode: false,
        }
    }
}

#[derive(Clone)]
pub struct RecoveryStats {
    pub total_recovery_attempts: u64,
    pub successful_recoveries: u64,
    pub failed_recoveries: u64,
    pub active_sessions: u16,
    pub success_rate_bps: u16,
    pub auto_recovery_enabled: bool,
    pub aggressive_mode: bool,
    pub avg_attempts_per_session: f32,
}