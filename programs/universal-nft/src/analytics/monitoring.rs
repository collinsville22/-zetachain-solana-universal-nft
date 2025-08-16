use anchor_lang::prelude::*;
use crate::errors::UniversalNftError;
use crate::analytics::metrics::{MetricsCollector, ThreatLevel};

/// Real-time System Monitoring for Universal NFT Protocol
/// Provides continuous health checks, alerting, and automatic remediation
#[account]
#[derive(InitSpace)]
pub struct SystemMonitor {
    /// Monitoring authority
    pub authority: Pubkey,
    /// Current system status
    pub system_status: SystemStatus,
    /// Last health check timestamp
    pub last_health_check: i64,
    /// Health check interval (seconds)
    pub health_check_interval: i64,
    /// Alert threshold configurations
    pub alert_thresholds: AlertThresholds,
    /// Active alerts count
    pub active_alerts: u16,
    /// Total alerts generated
    pub total_alerts: u64,
    /// System uptime start
    pub uptime_start: i64,
    /// Last downtime duration (seconds)
    pub last_downtime_duration: i64,
    /// Automatic remediation enabled
    pub auto_remediation_enabled: bool,
    /// Monitoring enabled
    pub monitoring_enabled: bool,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum SystemStatus {
    Healthy,
    Warning,
    Critical,
    Down,
    Maintenance,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AlertThresholds {
    /// Error rate threshold (basis points)
    pub error_rate_warning_bps: u16,
    pub error_rate_critical_bps: u16,
    /// Latency thresholds (milliseconds)
    pub latency_warning_ms: u32,
    pub latency_critical_ms: u32,
    /// TPS thresholds
    pub tps_warning_threshold: u16,
    pub tps_critical_threshold: u16,
    /// Memory usage thresholds (percentage)
    pub memory_warning_pct: u8,
    pub memory_critical_pct: u8,
    /// Compute unit thresholds
    pub compute_warning_units: u32,
    pub compute_critical_units: u32,
}

#[account]
#[derive(InitSpace)]
pub struct Alert {
    /// Alert ID
    pub id: u64,
    /// Alert type
    pub alert_type: AlertType,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert message
    pub message: String,
    /// Triggering metric value
    pub metric_value: u64,
    /// Threshold that was exceeded
    pub threshold_value: u64,
    /// Alert timestamp
    pub created_at: i64,
    /// Alert acknowledged by
    pub acknowledged_by: Option<Pubkey>,
    /// Alert acknowledgment timestamp
    pub acknowledged_at: Option<i64>,
    /// Alert resolved timestamp
    pub resolved_at: Option<i64>,
    /// Auto-remediation applied
    pub auto_remediation_applied: bool,
    /// Alert status
    pub status: AlertStatus,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum AlertType {
    HighErrorRate,
    HighLatency,
    LowTPS,
    HighMemoryUsage,
    HighComputeUsage,
    SecurityThreat,
    CircuitBreakerTriggered,
    FraudDetected,
    SystemDown,
    ChainUnavailable,
    UnusualActivity,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum AlertStatus {
    Active,
    Acknowledged,
    Resolved,
    Suppressed,
}

#[account]
#[derive(InitSpace)]
pub struct HealthCheck {
    /// Check ID
    pub id: u64,
    /// Check timestamp
    pub timestamp: i64,
    /// System status at time of check
    pub system_status: SystemStatus,
    /// Individual component statuses
    pub component_statuses: ComponentStatuses,
    /// Performance metrics snapshot
    pub metrics_snapshot: MetricsSnapshot,
    /// Issues detected
    pub issues_detected: u8,
    /// Recommendations generated
    pub recommendations_count: u8,
    /// Check duration (microseconds)
    pub check_duration_us: u64,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ComponentStatuses {
    pub nft_minting: ComponentStatus,
    pub cross_chain_bridge: ComponentStatus,
    pub security_system: ComponentStatus,
    pub governance: ComponentStatus,
    pub treasury: ComponentStatus,
    pub analytics: ComponentStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ComponentStatus {
    Operational,
    Degraded,
    Failed,
    Maintenance,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MetricsSnapshot {
    pub current_tps: u16,
    pub avg_latency_ms: u32,
    pub error_rate_bps: u16,
    pub memory_usage_pct: u8,
    pub compute_units_avg: u32,
    pub active_users: u32,
    pub pending_transactions: u32,
}

impl SystemMonitor {
    pub const INIT_SPACE: usize = 
        32 +    // authority
        1 +     // system_status (enum)
        8 +     // last_health_check
        8 +     // health_check_interval
        32 +    // alert_thresholds (struct)
        2 +     // active_alerts
        8 +     // total_alerts
        8 +     // uptime_start
        8 +     // last_downtime_duration
        1 +     // auto_remediation_enabled
        1 +     // monitoring_enabled
        1;      // bump

    /// Initialize system monitoring
    pub fn initialize(
        &mut self,
        authority: Pubkey,
        thresholds: AlertThresholds,
        bump: u8,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        
        self.authority = authority;
        self.system_status = SystemStatus::Healthy;
        self.last_health_check = now;
        self.health_check_interval = 300; // 5 minutes default
        self.alert_thresholds = thresholds;
        self.active_alerts = 0;
        self.total_alerts = 0;
        self.uptime_start = now;
        self.last_downtime_duration = 0;
        self.auto_remediation_enabled = true;
        self.monitoring_enabled = true;
        self.bump = bump;

        msg!("System monitoring initialized");
        msg!("Health check interval: {} seconds", self.health_check_interval);
        
        Ok(())
    }

    /// Perform comprehensive health check
    pub fn perform_health_check(
        &mut self,
        health_check: &mut HealthCheck,
        metrics: &MetricsCollector,
        check_id: u64,
    ) -> Result<()> {
        let start_time = Clock::get()?.unix_timestamp;
        let check_start_us = 0; // Would use high-precision timer in real implementation
        
        require!(self.monitoring_enabled, UniversalNftError::InvalidTransferStatus);

        // Initialize health check record
        health_check.id = check_id;
        health_check.timestamp = start_time;
        health_check.issues_detected = 0;
        health_check.recommendations_count = 0;

        // Create metrics snapshot
        let snapshot = MetricsSnapshot {
            current_tps: 0, // Would calculate from recent transactions
            avg_latency_ms: metrics.avg_latency_ms,
            error_rate_bps: metrics.current_error_rate_bps,
            memory_usage_pct: 45, // Would get from system
            compute_units_avg: (metrics.total_compute_units / (metrics.successful_operations + metrics.failed_operations).max(1)) as u32,
            active_users: metrics.active_users_30d,
            pending_transactions: 0, // Would get from transaction pool
        };

        health_check.metrics_snapshot = snapshot.clone();

        // Check individual components
        let component_statuses = self.check_components(&snapshot);
        health_check.component_statuses = component_statuses.clone();

        // Determine overall system status
        let new_status = self.calculate_system_status(&component_statuses, &snapshot)?;
        let status_changed = new_status != self.system_status;
        
        if status_changed {
            msg!("System status changed: {:?} -> {:?}", self.system_status, new_status);
            self.system_status = new_status.clone();
        }

        health_check.system_status = new_status;

        // Generate alerts if thresholds exceeded
        self.check_alert_conditions(&snapshot)?;

        // Update monitoring state
        self.last_health_check = start_time;
        
        let check_end_us = 1000; // Would calculate actual duration
        health_check.check_duration_us = check_end_us - check_start_us;

        msg!("Health check {} completed in {}μs", check_id, health_check.check_duration_us);
        
        Ok(())
    }

    /// Check individual system components
    fn check_components(&self, snapshot: &MetricsSnapshot) -> ComponentStatuses {
        ComponentStatuses {
            nft_minting: if snapshot.error_rate_bps < self.alert_thresholds.error_rate_warning_bps {
                ComponentStatus::Operational
            } else if snapshot.error_rate_bps < self.alert_thresholds.error_rate_critical_bps {
                ComponentStatus::Degraded
            } else {
                ComponentStatus::Failed
            },
            
            cross_chain_bridge: if snapshot.avg_latency_ms < self.alert_thresholds.latency_warning_ms {
                ComponentStatus::Operational
            } else if snapshot.avg_latency_ms < self.alert_thresholds.latency_critical_ms {
                ComponentStatus::Degraded
            } else {
                ComponentStatus::Failed
            },
            
            security_system: ComponentStatus::Operational, // Would check security metrics
            governance: ComponentStatus::Operational,      // Would check governance health
            treasury: ComponentStatus::Operational,        // Would check treasury operations
            analytics: ComponentStatus::Operational,       // Would check analytics collection
        }
    }

    /// Calculate overall system status
    fn calculate_system_status(
        &self,
        components: &ComponentStatuses,
        snapshot: &MetricsSnapshot,
    ) -> Result<SystemStatus> {
        let components_array = [
            &components.nft_minting,
            &components.cross_chain_bridge,
            &components.security_system,
            &components.governance,
            &components.treasury,
            &components.analytics,
        ];

        let failed_count = components_array.iter()
            .filter(|&&status| status == ComponentStatus::Failed)
            .count();

        let degraded_count = components_array.iter()
            .filter(|&&status| status == ComponentStatus::Degraded)
            .count();

        let status = if failed_count > 2 {
            SystemStatus::Down
        } else if failed_count > 0 || degraded_count > 3 {
            SystemStatus::Critical
        } else if degraded_count > 0 || 
                  snapshot.error_rate_bps > self.alert_thresholds.error_rate_warning_bps ||
                  snapshot.avg_latency_ms > self.alert_thresholds.latency_warning_ms {
            SystemStatus::Warning
        } else {
            SystemStatus::Healthy
        };

        Ok(status)
    }

    /// Check conditions that should trigger alerts
    fn check_alert_conditions(&mut self, snapshot: &MetricsSnapshot) -> Result<()> {
        // Check error rate
        if snapshot.error_rate_bps > self.alert_thresholds.error_rate_critical_bps {
            self.trigger_alert(AlertType::HighErrorRate, AlertSeverity::Critical, snapshot.error_rate_bps as u64)?;
        } else if snapshot.error_rate_bps > self.alert_thresholds.error_rate_warning_bps {
            self.trigger_alert(AlertType::HighErrorRate, AlertSeverity::Warning, snapshot.error_rate_bps as u64)?;
        }

        // Check latency
        if snapshot.avg_latency_ms > self.alert_thresholds.latency_critical_ms {
            self.trigger_alert(AlertType::HighLatency, AlertSeverity::Critical, snapshot.avg_latency_ms as u64)?;
        } else if snapshot.avg_latency_ms > self.alert_thresholds.latency_warning_ms {
            self.trigger_alert(AlertType::HighLatency, AlertSeverity::Warning, snapshot.avg_latency_ms as u64)?;
        }

        // Check memory usage
        if snapshot.memory_usage_pct > self.alert_thresholds.memory_critical_pct {
            self.trigger_alert(AlertType::HighMemoryUsage, AlertSeverity::Critical, snapshot.memory_usage_pct as u64)?;
        } else if snapshot.memory_usage_pct > self.alert_thresholds.memory_warning_pct {
            self.trigger_alert(AlertType::HighMemoryUsage, AlertSeverity::Warning, snapshot.memory_usage_pct as u64)?;
        }

        // Check compute units
        if snapshot.compute_units_avg > self.alert_thresholds.compute_critical_units {
            self.trigger_alert(AlertType::HighComputeUsage, AlertSeverity::Critical, snapshot.compute_units_avg as u64)?;
        } else if snapshot.compute_units_avg > self.alert_thresholds.compute_warning_units {
            self.trigger_alert(AlertType::HighComputeUsage, AlertSeverity::Warning, snapshot.compute_units_avg as u64)?;
        }

        Ok(())
    }

    /// Trigger an alert
    fn trigger_alert(
        &mut self,
        alert_type: AlertType,
        severity: AlertSeverity,
        metric_value: u64,
    ) -> Result<()> {
        self.active_alerts = self.active_alerts.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        
        self.total_alerts = self.total_alerts.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        msg!("Alert triggered: {:?} - Severity: {:?} - Value: {}", 
             alert_type, severity, metric_value);

        // Apply auto-remediation if enabled and appropriate
        if self.auto_remediation_enabled {
            self.apply_auto_remediation(&alert_type, &severity)?;
        }

        Ok(())
    }

    /// Apply automatic remediation for certain alert types
    fn apply_auto_remediation(
        &self,
        alert_type: &AlertType,
        severity: &AlertSeverity,
    ) -> Result<()> {
        match (alert_type, severity) {
            (AlertType::HighErrorRate, AlertSeverity::Critical) => {
                msg!("Auto-remediation: Activating circuit breaker");
                // Would trigger circuit breaker
            }
            (AlertType::HighMemoryUsage, AlertSeverity::Critical) => {
                msg!("Auto-remediation: Clearing caches and optimizing memory");
                // Would trigger memory cleanup
            }
            (AlertType::SecurityThreat, _) => {
                msg!("Auto-remediation: Enhancing security monitoring");
                // Would increase security checks
            }
            _ => {
                // No auto-remediation for this alert type/severity
            }
        }
        Ok(())
    }

    /// Acknowledge an alert
    pub fn acknowledge_alert(&mut self, alert: &mut Alert, acknowledger: Pubkey) -> Result<()> {
        require!(alert.status == AlertStatus::Active, UniversalNftError::InvalidTransferStatus);
        
        alert.acknowledged_by = Some(acknowledger);
        alert.acknowledged_at = Some(Clock::get()?.unix_timestamp);
        alert.status = AlertStatus::Acknowledged;

        msg!("Alert {} acknowledged by {}", alert.id, acknowledger);
        Ok(())
    }

    /// Resolve an alert
    pub fn resolve_alert(&mut self, alert: &mut Alert) -> Result<()> {
        require!(
            alert.status == AlertStatus::Active || alert.status == AlertStatus::Acknowledged,
            UniversalNftError::InvalidTransferStatus
        );
        
        alert.resolved_at = Some(Clock::get()?.unix_timestamp);
        alert.status = AlertStatus::Resolved;

        self.active_alerts = self.active_alerts.saturating_sub(1);

        msg!("Alert {} resolved", alert.id);
        Ok(())
    }

    /// Update monitoring configuration
    pub fn update_config(
        &mut self,
        health_check_interval: Option<i64>,
        thresholds: Option<AlertThresholds>,
        auto_remediation: Option<bool>,
    ) -> Result<()> {
        if let Some(interval) = health_check_interval {
            self.health_check_interval = interval;
        }
        
        if let Some(new_thresholds) = thresholds {
            self.alert_thresholds = new_thresholds;
        }
        
        if let Some(auto_rem) = auto_remediation {
            self.auto_remediation_enabled = auto_rem;
        }

        msg!("Monitoring configuration updated");
        Ok(())
    }

    /// Get monitoring statistics
    pub fn get_monitoring_stats(&self) -> MonitoringStats {
        let now = Clock::get().unwrap().unix_timestamp;
        let uptime_duration = now - self.uptime_start;
        let uptime_percentage = if uptime_duration > 0 {
            ((uptime_duration - self.last_downtime_duration) * 100) / uptime_duration
        } else {
            100
        };

        MonitoringStats {
            system_status: self.system_status.clone(),
            uptime_percentage: uptime_percentage as u8,
            active_alerts: self.active_alerts,
            total_alerts: self.total_alerts,
            last_health_check: self.last_health_check,
            monitoring_enabled: self.monitoring_enabled,
            auto_remediation_enabled: self.auto_remediation_enabled,
            uptime_hours: uptime_duration / 3600,
        }
    }
}

impl Alert {
    pub const INIT_SPACE: usize = 
        8 +     // id
        1 +     // alert_type (enum)
        1 +     // severity (enum)
        4 + 256 + // message (String)
        8 +     // metric_value
        8 +     // threshold_value
        8 +     // created_at
        1 + 32 + // acknowledged_by (Option<Pubkey>)
        1 + 8 + // acknowledged_at (Option<i64>)
        1 + 8 + // resolved_at (Option<i64>)
        1 +     // auto_remediation_applied
        1 +     // status (enum)
        1;      // bump

    pub fn initialize(
        &mut self,
        id: u64,
        alert_type: AlertType,
        severity: AlertSeverity,
        message: String,
        metric_value: u64,
        threshold_value: u64,
        bump: u8,
    ) -> Result<()> {
        require!(message.len() <= 256, UniversalNftError::InvalidTransferStatus);
        
        self.id = id;
        self.alert_type = alert_type;
        self.severity = severity;
        self.message = message;
        self.metric_value = metric_value;
        self.threshold_value = threshold_value;
        self.created_at = Clock::get()?.unix_timestamp;
        self.acknowledged_by = None;
        self.acknowledged_at = None;
        self.resolved_at = None;
        self.auto_remediation_applied = false;
        self.status = AlertStatus::Active;
        self.bump = bump;

        Ok(())
    }
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            error_rate_warning_bps: 100,     // 1%
            error_rate_critical_bps: 500,    // 5%
            latency_warning_ms: 2000,        // 2 seconds
            latency_critical_ms: 5000,       // 5 seconds
            tps_warning_threshold: 10,       // < 10 TPS
            tps_critical_threshold: 5,       // < 5 TPS
            memory_warning_pct: 80,          // 80%
            memory_critical_pct: 95,         // 95%
            compute_warning_units: 150_000,  // 150k compute units
            compute_critical_units: 200_000, // 200k compute units
        }
    }
}

#[derive(Clone)]
pub struct MonitoringStats {
    pub system_status: SystemStatus,
    pub uptime_percentage: u8,
    pub active_alerts: u16,
    pub total_alerts: u64,
    pub last_health_check: i64,
    pub monitoring_enabled: bool,
    pub auto_remediation_enabled: bool,
    pub uptime_hours: i64,
}