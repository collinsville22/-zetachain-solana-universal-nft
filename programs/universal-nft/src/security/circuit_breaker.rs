use anchor_lang::prelude::*;
use crate::errors::UniversalNftError;

/// Advanced Circuit Breaker Pattern for Cross-Chain Operations
/// Implements automatic shutdowns when anomalies are detected
#[account]
#[derive(InitSpace)]
pub struct CircuitBreaker {
    /// Current circuit state
    pub state: CircuitState,
    /// Failure count in current window
    pub failure_count: u64,
    /// Success count in current window
    pub success_count: u64,
    /// Window start timestamp
    pub window_start: i64,
    /// Last state change timestamp
    pub last_state_change: i64,
    /// Configuration parameters
    pub config: CircuitConfig,
    /// Authority that can manually override
    pub authority: Pubkey,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Normal operation
    Closed,
    /// Partial restrictions - some operations allowed
    HalfOpen,
    /// Full shutdown - all operations blocked
    Open,
    /// Manual override by authority
    ManualOverride,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct CircuitConfig {
    /// Failure threshold to trigger circuit opening
    pub failure_threshold: u64,
    /// Time window for failure counting (seconds)
    pub failure_window: i64,
    /// Minimum time circuit stays open (seconds)
    pub min_open_duration: i64,
    /// Success threshold to close circuit from half-open
    pub success_threshold: u64,
    /// Maximum operations per window in half-open state
    pub half_open_limit: u64,
}

impl Default for CircuitConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            failure_window: 300,      // 5 minutes
            min_open_duration: 600,   // 10 minutes
            success_threshold: 3,
            half_open_limit: 10,
        }
    }
}

impl CircuitBreaker {
    pub const INIT_SPACE: usize = 
        1 +  // state
        8 +  // failure_count
        8 +  // success_count
        8 +  // window_start
        8 +  // last_state_change
        8 * 5 + // config (5 u64s)
        32 + // authority
        1;   // bump

    /// Initialize circuit breaker
    pub fn initialize(&mut self, authority: Pubkey, config: Option<CircuitConfig>, bump: u8) {
        self.state = CircuitState::Closed;
        self.failure_count = 0;
        self.success_count = 0;
        self.window_start = Clock::get().unwrap().unix_timestamp;
        self.last_state_change = self.window_start;
        self.config = config.unwrap_or_default();
        self.authority = authority;
        self.bump = bump;
    }

    /// Check if operation should be allowed
    pub fn check_operation_allowed(&mut self, operation_type: OperationType) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        
        // Update window if needed
        self.update_window(now)?;
        
        match self.state {
            CircuitState::Closed => {
                // Normal operation - check if we should open circuit
                if self.should_open_circuit(now) {
                    self.transition_to_open(now)?;
                    return Err(UniversalNftError::CircuitBreakerOpen.into());
                }
                Ok(())
            },
            CircuitState::HalfOpen => {
                // Limited operation - check limits
                let total_ops = self.failure_count + self.success_count;
                if total_ops >= self.config.half_open_limit {
                    return Err(UniversalNftError::CircuitBreakerRateLimit.into());
                }
                
                // Check if we should close or open based on recent performance
                if self.should_close_circuit() {
                    self.transition_to_closed(now)?;
                } else if self.should_open_circuit(now) {
                    self.transition_to_open(now)?;
                    return Err(UniversalNftError::CircuitBreakerOpen.into());
                }
                Ok(())
            },
            CircuitState::Open => {
                // Check if enough time has passed to try half-open
                if now - self.last_state_change >= self.config.min_open_duration {
                    self.transition_to_half_open(now)?;
                    Ok(())
                } else {
                    Err(UniversalNftError::CircuitBreakerOpen.into())
                }
            },
            CircuitState::ManualOverride => {
                // Manual override - allow all operations
                Ok(())
            }
        }
    }

    /// Record operation success
    pub fn record_success(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        self.update_window(now)?;
        
        self.success_count = self.success_count.saturating_add(1);
        
        // Check if we should transition state
        if self.state == CircuitState::HalfOpen && self.should_close_circuit() {
            self.transition_to_closed(now)?;
        }
        
        Ok(())
    }

    /// Record operation failure
    pub fn record_failure(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        self.update_window(now)?;
        
        self.failure_count = self.failure_count.saturating_add(1);
        
        // Check if we should open circuit
        if self.should_open_circuit(now) {
            self.transition_to_open(now)?;
        }
        
        Ok(())
    }

    /// Manual override by authority
    pub fn set_manual_override(&mut self, enabled: bool) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        
        if enabled {
            self.state = CircuitState::ManualOverride;
        } else {
            // Return to closed state when disabling override
            self.transition_to_closed(now)?;
        }
        
        Ok(())
    }

    /// Get current circuit health metrics
    pub fn get_health_metrics(&self) -> CircuitHealthMetrics {
        let total_ops = self.failure_count + self.success_count;
        let success_rate = if total_ops > 0 {
            (self.success_count as f64 / total_ops as f64) * 100.0
        } else {
            100.0
        };

        CircuitHealthMetrics {
            state: self.state,
            success_rate,
            total_operations: total_ops,
            failures_in_window: self.failure_count,
            time_in_current_state: Clock::get().unwrap().unix_timestamp - self.last_state_change,
        }
    }

    // Private helper methods
    fn update_window(&mut self, now: i64) -> Result<()> {
        if now - self.window_start >= self.config.failure_window {
            self.window_start = now;
            self.failure_count = 0;
            self.success_count = 0;
        }
        Ok(())
    }

    fn should_open_circuit(&self, _now: i64) -> bool {
        self.failure_count >= self.config.failure_threshold
    }

    fn should_close_circuit(&self) -> bool {
        self.success_count >= self.config.success_threshold && self.failure_count == 0
    }

    fn transition_to_open(&mut self, now: i64) -> Result<()> {
        self.state = CircuitState::Open;
        self.last_state_change = now;
        msg!("🚨 Circuit breaker OPENED - system protection activated");
        Ok(())
    }

    fn transition_to_half_open(&mut self, now: i64) -> Result<()> {
        self.state = CircuitState::HalfOpen;
        self.last_state_change = now;
        self.failure_count = 0;
        self.success_count = 0;
        msg!("⚠️ Circuit breaker HALF-OPEN - limited operations allowed");
        Ok(())
    }

    fn transition_to_closed(&mut self, now: i64) -> Result<()> {
        self.state = CircuitState::Closed;
        self.last_state_change = now;
        self.failure_count = 0;
        self.success_count = 0;
        msg!("✅ Circuit breaker CLOSED - normal operations resumed");
        Ok(())
    }
}

#[derive(Clone)]
pub struct CircuitHealthMetrics {
    pub state: CircuitState,
    pub success_rate: f64,
    pub total_operations: u64,
    pub failures_in_window: u64,
    pub time_in_current_state: i64,
}

#[derive(Clone, Copy)]
pub enum OperationType {
    CrossChainTransfer,
    NFTMinting,
    MetadataUpdate,
    SignatureVerification,
}

/// Enhanced error types for circuit breaker
impl From<CircuitState> for UniversalNftError {
    fn from(state: CircuitState) -> Self {
        match state {
            CircuitState::Open => UniversalNftError::CircuitBreakerOpen,
            CircuitState::HalfOpen => UniversalNftError::CircuitBreakerRateLimit,
            _ => UniversalNftError::InvalidCallOrigin,
        }
    }
}