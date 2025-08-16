use anchor_lang::prelude::*;
use crate::errors::UniversalNftError;

/// Enterprise Solutions Module for Universal NFT Protocol
/// Provides enterprise-grade features for institutional adoption
#[account]
#[derive(InitSpace)]
pub struct EnterpriseManager {
    /// Enterprise manager authority
    pub authority: Pubkey,
    /// Total enterprise clients
    pub total_enterprise_clients: u32,
    /// Total enterprise volume (in lamports)
    pub total_enterprise_volume: u64,
    /// Enterprise tier configurations
    pub tier_configs: [EnterpriseTier; 4],
    /// Compliance framework version
    pub compliance_version: String,
    /// Last compliance update
    pub last_compliance_update: i64,
    /// Enterprise features enabled
    pub enterprise_features_enabled: bool,
    /// PDA bump
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct EnterpriseClient {
    /// Client organization identifier
    pub client_id: u64,
    /// Client organization public key
    pub organization: Pubkey,
    /// Client tier level
    pub tier: EnterpriseClientTier,
    /// Enterprise configuration
    pub config: EnterpriseConfig,
    /// Compliance status
    pub compliance_status: ComplianceStatus,
    /// Client metrics
    pub metrics: EnterpriseMetrics,
    /// Service level agreement
    pub sla: ServiceLevelAgreement,
    /// Account created timestamp
    pub created_at: i64,
    /// Last activity timestamp
    pub last_activity: i64,
    /// Contract end date
    pub contract_end_date: i64,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EnterpriseTier {
    /// Tier name
    pub name: String,
    /// Monthly fee (in lamports)
    pub monthly_fee: u64,
    /// Transaction limit per month
    pub monthly_tx_limit: u32,
    /// Volume limit per month (in lamports)
    pub monthly_volume_limit: u64,
    /// Support level
    pub support_level: SupportLevel,
    /// Features included
    pub included_features: Vec<EnterpriseFeature>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum EnterpriseClientTier {
    Startup,    // $1K/month
    Growth,     // $10K/month
    Enterprise, // $50K/month
    Fortune500, // $250K/month
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EnterpriseConfig {
    /// Custom branding enabled
    pub custom_branding: bool,
    /// Dedicated infrastructure
    pub dedicated_infrastructure: bool,
    /// Advanced analytics enabled
    pub advanced_analytics: bool,
    /// Priority support enabled
    pub priority_support: bool,
    /// Custom fee structure
    pub custom_fee_bps: Option<u16>,
    /// Compliance requirements
    pub compliance_requirements: Vec<ComplianceRequirement>,
    /// API rate limits
    pub api_rate_limits: ApiLimits,
    /// Security configurations
    pub security_config: SecurityConfig,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum SupportLevel {
    Community,
    Business,
    Premium,
    Dedicated,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum EnterpriseFeature {
    AdvancedAnalytics,
    CustomBranding,
    PrioritySupport,
    DedicatedInfrastructure,
    ComplianceReporting,
    BulkOperations,
    APIAccess,
    WebhookSupport,
    CustomIntegration,
    TrainingProgram,
    OnboardingSupport,
    TechnicalAccountManager,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ComplianceStatus {
    /// Overall compliance score (0-100)
    pub compliance_score: u8,
    /// KYC completion status
    pub kyc_completed: bool,
    /// AML verification status
    pub aml_verified: bool,
    /// GDPR compliance status
    pub gdpr_compliant: bool,
    /// SOX compliance (for public companies)
    pub sox_compliant: bool,
    /// Last compliance audit date
    pub last_audit_date: i64,
    /// Next audit due date
    pub next_audit_due: i64,
    /// Compliance officer contact
    pub compliance_officer: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ComplianceRequirement {
    KYC,
    AML,
    GDPR,
    SOX,
    PCI_DSS,
    ISO27001,
    HIPAA,
    Custom(String),
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EnterpriseMetrics {
    /// Total transactions processed
    pub total_transactions: u64,
    /// Total volume processed (lamports)
    pub total_volume: u64,
    /// Monthly active users
    pub monthly_active_users: u32,
    /// Average transaction value
    pub avg_transaction_value: u64,
    /// Success rate (basis points)
    pub success_rate_bps: u16,
    /// Average processing time (ms)
    pub avg_processing_time_ms: u32,
    /// Cost savings compared to traditional methods
    pub cost_savings_percentage: u16,
    /// ROI measurement
    pub roi_percentage: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ServiceLevelAgreement {
    /// Uptime guarantee (basis points)
    pub uptime_guarantee_bps: u16,
    /// Maximum response time (ms)
    pub max_response_time_ms: u32,
    /// Support response time (minutes)
    pub support_response_time_minutes: u16,
    /// Resolution time SLA (hours)
    pub resolution_time_hours: u16,
    /// Penalties for SLA breaches
    pub breach_penalties: Vec<SLAPenalty>,
    /// Credits for outages
    pub outage_credits_bps: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SLAPenalty {
    /// Breach type
    pub breach_type: SLABreachType,
    /// Penalty percentage of monthly fee
    pub penalty_percentage: u16,
    /// Maximum penalty cap
    pub max_penalty_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum SLABreachType {
    UptimeBreach,
    ResponseTimeBreach,
    SupportResponseBreach,
    SecurityBreach,
    DataBreach,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ApiLimits {
    /// Requests per minute
    pub requests_per_minute: u32,
    /// Requests per hour
    pub requests_per_hour: u32,
    /// Requests per day
    pub requests_per_day: u32,
    /// Burst limit
    pub burst_limit: u32,
    /// Rate limit exemptions
    pub exemptions: Vec<String>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SecurityConfig {
    /// IP whitelist enabled
    pub ip_whitelist_enabled: bool,
    /// Multi-factor authentication required
    pub mfa_required: bool,
    /// API key rotation frequency (days)
    pub api_key_rotation_days: u16,
    /// Session timeout (minutes)
    pub session_timeout_minutes: u16,
    /// Advanced threat protection
    pub advanced_threat_protection: bool,
    /// Audit logging level
    pub audit_logging_level: AuditLevel,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum AuditLevel {
    Basic,
    Enhanced,
    Comprehensive,
    Forensic,
}

impl EnterpriseManager {
    pub const INIT_SPACE: usize = 
        32 +     // authority
        4 +      // total_enterprise_clients
        8 +      // total_enterprise_volume
        1024 +   // tier_configs (estimated)
        4 + 32 + // compliance_version
        8 +      // last_compliance_update
        1 +      // enterprise_features_enabled
        1;       // bump

    /// Initialize enterprise manager
    pub fn initialize(&mut self, authority: Pubkey, bump: u8) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        
        self.authority = authority;
        self.total_enterprise_clients = 0;
        self.total_enterprise_volume = 0;
        self.tier_configs = Self::default_tier_configs();
        self.compliance_version = "v1.0".to_string();
        self.last_compliance_update = now;
        self.enterprise_features_enabled = true;
        self.bump = bump;

        msg!("Enterprise manager initialized");
        Ok(())
    }

    /// Onboard new enterprise client
    pub fn onboard_enterprise_client(
        &mut self,
        client: &mut EnterpriseClient,
        client_id: u64,
        organization: Pubkey,
        tier: EnterpriseClientTier,
        contract_duration_months: u16,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        
        // Initialize client configuration based on tier
        let config = Self::create_tier_config(&tier);
        let sla = Self::create_tier_sla(&tier);

        client.client_id = client_id;
        client.organization = organization;
        client.tier = tier;
        client.config = config;
        client.compliance_status = ComplianceStatus {
            compliance_score: 0,
            kyc_completed: false,
            aml_verified: false,
            gdpr_compliant: false,
            sox_compliant: false,
            last_audit_date: 0,
            next_audit_due: now + (365 * 24 * 3600), // 1 year
            compliance_officer: organization, // Placeholder
        };
        client.metrics = EnterpriseMetrics {
            total_transactions: 0,
            total_volume: 0,
            monthly_active_users: 0,
            avg_transaction_value: 0,
            success_rate_bps: 10000, // Start at 100%
            avg_processing_time_ms: 0,
            cost_savings_percentage: 0,
            roi_percentage: 0,
        };
        client.sla = sla;
        client.created_at = now;
        client.last_activity = now;
        client.contract_end_date = now + (contract_duration_months as i64 * 30 * 24 * 3600);

        // Update manager statistics
        self.total_enterprise_clients = self.total_enterprise_clients.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        msg!("Enterprise client {} onboarded with tier: {:?}", client_id, tier);
        Ok(())
    }

    /// Process enterprise transaction
    pub fn process_enterprise_transaction(
        &mut self,
        client: &mut EnterpriseClient,
        transaction_value: u64,
        processing_time_ms: u32,
        success: bool,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        // Update client metrics
        client.metrics.total_transactions = client.metrics.total_transactions.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        if success {
            client.metrics.total_volume = client.metrics.total_volume.checked_add(transaction_value)
                .ok_or(UniversalNftError::ArithmeticOverflow)?;
        }

        // Update success rate
        let total_txs = client.metrics.total_transactions;
        if success {
            client.metrics.success_rate_bps = ((client.metrics.success_rate_bps as u64 * (total_txs - 1) + 10000) / total_txs) as u16;
        } else {
            client.metrics.success_rate_bps = ((client.metrics.success_rate_bps as u64 * (total_txs - 1)) / total_txs) as u16;
        }

        // Update average processing time
        client.metrics.avg_processing_time_ms = 
            ((client.metrics.avg_processing_time_ms as u64 * (total_txs - 1)) + processing_time_ms as u64) as u32 / total_txs as u32;

        // Update average transaction value
        if success {
            let successful_volume = (client.metrics.total_volume as f64 * client.metrics.success_rate_bps as f64 / 10000.0) as u64;
            let successful_txs = (total_txs as f64 * client.metrics.success_rate_bps as f64 / 10000.0) as u64;
            if successful_txs > 0 {
                client.metrics.avg_transaction_value = successful_volume / successful_txs;
            }
        }

        client.last_activity = now;

        // Update manager totals
        if success {
            self.total_enterprise_volume = self.total_enterprise_volume.checked_add(transaction_value)
                .ok_or(UniversalNftError::ArithmeticOverflow)?;
        }

        Ok(())
    }

    /// Update compliance status
    pub fn update_compliance_status(
        &mut self,
        client: &mut EnterpriseClient,
        requirement: ComplianceRequirement,
        status: bool,
    ) -> Result<()> {
        match requirement {
            ComplianceRequirement::KYC => client.compliance_status.kyc_completed = status,
            ComplianceRequirement::AML => client.compliance_status.aml_verified = status,
            ComplianceRequirement::GDPR => client.compliance_status.gdpr_compliant = status,
            ComplianceRequirement::SOX => client.compliance_status.sox_compliant = status,
            _ => {} // Handle other requirements
        }

        // Recalculate compliance score
        let mut score = 0u8;
        if client.compliance_status.kyc_completed { score += 25; }
        if client.compliance_status.aml_verified { score += 25; }
        if client.compliance_status.gdpr_compliant { score += 25; }
        if client.compliance_status.sox_compliant { score += 25; }
        
        client.compliance_status.compliance_score = score;

        msg!("Compliance updated for client {}: {:?} = {}", 
             client.client_id, requirement, status);
        Ok(())
    }

    /// Generate enterprise report
    pub fn generate_enterprise_report(&self, client: &EnterpriseClient) -> EnterpriseReport {
        let now = Clock::get().unwrap().unix_timestamp;
        let days_active = ((now - client.created_at) / (24 * 3600)).max(1);
        
        EnterpriseReport {
            client_id: client.client_id,
            reporting_period_days: days_active,
            total_transactions: client.metrics.total_transactions,
            total_volume: client.metrics.total_volume,
            success_rate_bps: client.metrics.success_rate_bps,
            avg_processing_time_ms: client.metrics.avg_processing_time_ms,
            compliance_score: client.compliance_status.compliance_score,
            cost_savings: client.metrics.cost_savings_percentage,
            roi_percentage: client.metrics.roi_percentage,
            sla_breaches: 0, // Would calculate actual breaches
            uptime_achieved_bps: 9999, // 99.99% example
            generated_at: now,
        }
    }

    // Helper methods for tier configuration

    fn default_tier_configs() -> [EnterpriseTier; 4] {
        [
            EnterpriseTier {
                name: "Startup".to_string(),
                monthly_fee: 1_000_000_000_000, // 1K SOL
                monthly_tx_limit: 10_000,
                monthly_volume_limit: 1_000_000_000_000_000, // 1M SOL
                support_level: SupportLevel::Business,
                included_features: vec![
                    EnterpriseFeature::APIAccess,
                    EnterpriseFeature::ComplianceReporting,
                ],
            },
            EnterpriseTier {
                name: "Growth".to_string(),
                monthly_fee: 10_000_000_000_000, // 10K SOL
                monthly_tx_limit: 100_000,
                monthly_volume_limit: 10_000_000_000_000_000, // 10M SOL
                support_level: SupportLevel::Premium,
                included_features: vec![
                    EnterpriseFeature::APIAccess,
                    EnterpriseFeature::ComplianceReporting,
                    EnterpriseFeature::AdvancedAnalytics,
                    EnterpriseFeature::PrioritySupport,
                ],
            },
            EnterpriseTier {
                name: "Enterprise".to_string(),
                monthly_fee: 50_000_000_000_000, // 50K SOL
                monthly_tx_limit: 1_000_000,
                monthly_volume_limit: 100_000_000_000_000_000, // 100M SOL
                support_level: SupportLevel::Dedicated,
                included_features: vec![
                    EnterpriseFeature::APIAccess,
                    EnterpriseFeature::ComplianceReporting,
                    EnterpriseFeature::AdvancedAnalytics,
                    EnterpriseFeature::PrioritySupport,
                    EnterpriseFeature::CustomBranding,
                    EnterpriseFeature::DedicatedInfrastructure,
                ],
            },
            EnterpriseTier {
                name: "Fortune500".to_string(),
                monthly_fee: 250_000_000_000_000, // 250K SOL
                monthly_tx_limit: u32::MAX, // Unlimited
                monthly_volume_limit: u64::MAX, // Unlimited
                support_level: SupportLevel::Dedicated,
                included_features: vec![
                    EnterpriseFeature::APIAccess,
                    EnterpriseFeature::ComplianceReporting,
                    EnterpriseFeature::AdvancedAnalytics,
                    EnterpriseFeature::PrioritySupport,
                    EnterpriseFeature::CustomBranding,
                    EnterpriseFeature::DedicatedInfrastructure,
                    EnterpriseFeature::TechnicalAccountManager,
                    EnterpriseFeature::CustomIntegration,
                ],
            },
        ]
    }

    fn create_tier_config(tier: &EnterpriseClientTier) -> EnterpriseConfig {
        match tier {
            EnterpriseClientTier::Startup => EnterpriseConfig {
                custom_branding: false,
                dedicated_infrastructure: false,
                advanced_analytics: false,
                priority_support: false,
                custom_fee_bps: None,
                compliance_requirements: vec![ComplianceRequirement::KYC],
                api_rate_limits: ApiLimits {
                    requests_per_minute: 100,
                    requests_per_hour: 5000,
                    requests_per_day: 100000,
                    burst_limit: 200,
                    exemptions: vec![],
                },
                security_config: SecurityConfig {
                    ip_whitelist_enabled: false,
                    mfa_required: true,
                    api_key_rotation_days: 90,
                    session_timeout_minutes: 60,
                    advanced_threat_protection: false,
                    audit_logging_level: AuditLevel::Basic,
                },
            },
            EnterpriseClientTier::Growth => EnterpriseConfig {
                custom_branding: false,
                dedicated_infrastructure: false,
                advanced_analytics: true,
                priority_support: true,
                custom_fee_bps: Some(75), // 0.75%
                compliance_requirements: vec![ComplianceRequirement::KYC, ComplianceRequirement::AML],
                api_rate_limits: ApiLimits {
                    requests_per_minute: 500,
                    requests_per_hour: 25000,
                    requests_per_day: 500000,
                    burst_limit: 1000,
                    exemptions: vec![],
                },
                security_config: SecurityConfig {
                    ip_whitelist_enabled: true,
                    mfa_required: true,
                    api_key_rotation_days: 60,
                    session_timeout_minutes: 120,
                    advanced_threat_protection: true,
                    audit_logging_level: AuditLevel::Enhanced,
                },
            },
            EnterpriseClientTier::Enterprise => EnterpriseConfig {
                custom_branding: true,
                dedicated_infrastructure: true,
                advanced_analytics: true,
                priority_support: true,
                custom_fee_bps: Some(50), // 0.5%
                compliance_requirements: vec![
                    ComplianceRequirement::KYC, 
                    ComplianceRequirement::AML,
                    ComplianceRequirement::GDPR,
                ],
                api_rate_limits: ApiLimits {
                    requests_per_minute: 2000,
                    requests_per_hour: 100000,
                    requests_per_day: 2000000,
                    burst_limit: 5000,
                    exemptions: vec!["bulk_operations".to_string()],
                },
                security_config: SecurityConfig {
                    ip_whitelist_enabled: true,
                    mfa_required: true,
                    api_key_rotation_days: 30,
                    session_timeout_minutes: 240,
                    advanced_threat_protection: true,
                    audit_logging_level: AuditLevel::Comprehensive,
                },
            },
            EnterpriseClientTier::Fortune500 => EnterpriseConfig {
                custom_branding: true,
                dedicated_infrastructure: true,
                advanced_analytics: true,
                priority_support: true,
                custom_fee_bps: Some(25), // 0.25%
                compliance_requirements: vec![
                    ComplianceRequirement::KYC, 
                    ComplianceRequirement::AML,
                    ComplianceRequirement::GDPR,
                    ComplianceRequirement::SOX,
                ],
                api_rate_limits: ApiLimits {
                    requests_per_minute: u32::MAX,
                    requests_per_hour: u32::MAX,
                    requests_per_day: u32::MAX,
                    burst_limit: u32::MAX,
                    exemptions: vec!["all".to_string()],
                },
                security_config: SecurityConfig {
                    ip_whitelist_enabled: true,
                    mfa_required: true,
                    api_key_rotation_days: 14,
                    session_timeout_minutes: 480,
                    advanced_threat_protection: true,
                    audit_logging_level: AuditLevel::Forensic,
                },
            },
        }
    }

    fn create_tier_sla(tier: &EnterpriseClientTier) -> ServiceLevelAgreement {
        match tier {
            EnterpriseClientTier::Startup => ServiceLevelAgreement {
                uptime_guarantee_bps: 9900, // 99%
                max_response_time_ms: 5000,
                support_response_time_minutes: 240, // 4 hours
                resolution_time_hours: 48,
                breach_penalties: vec![],
                outage_credits_bps: 100, // 1%
            },
            EnterpriseClientTier::Growth => ServiceLevelAgreement {
                uptime_guarantee_bps: 9950, // 99.5%
                max_response_time_ms: 2000,
                support_response_time_minutes: 120, // 2 hours
                resolution_time_hours: 24,
                breach_penalties: vec![SLAPenalty {
                    breach_type: SLABreachType::UptimeBreach,
                    penalty_percentage: 10,
                    max_penalty_amount: 1_000_000_000_000, // 1K SOL
                }],
                outage_credits_bps: 200, // 2%
            },
            EnterpriseClientTier::Enterprise => ServiceLevelAgreement {
                uptime_guarantee_bps: 9990, // 99.9%
                max_response_time_ms: 1000,
                support_response_time_minutes: 60, // 1 hour
                resolution_time_hours: 12,
                breach_penalties: vec![
                    SLAPenalty {
                        breach_type: SLABreachType::UptimeBreach,
                        penalty_percentage: 20,
                        max_penalty_amount: 10_000_000_000_000, // 10K SOL
                    },
                    SLAPenalty {
                        breach_type: SLABreachType::ResponseTimeBreach,
                        penalty_percentage: 10,
                        max_penalty_amount: 5_000_000_000_000, // 5K SOL
                    },
                ],
                outage_credits_bps: 300, // 3%
            },
            EnterpriseClientTier::Fortune500 => ServiceLevelAgreement {
                uptime_guarantee_bps: 9999, // 99.99%
                max_response_time_ms: 500,
                support_response_time_minutes: 15, // 15 minutes
                resolution_time_hours: 4,
                breach_penalties: vec![
                    SLAPenalty {
                        breach_type: SLABreachType::UptimeBreach,
                        penalty_percentage: 50,
                        max_penalty_amount: 50_000_000_000_000, // 50K SOL
                    },
                    SLAPenalty {
                        breach_type: SLABreachType::ResponseTimeBreach,
                        penalty_percentage: 25,
                        max_penalty_amount: 25_000_000_000_000, // 25K SOL
                    },
                    SLAPenalty {
                        breach_type: SLABreachType::SupportResponseBreach,
                        penalty_percentage: 15,
                        max_penalty_amount: 10_000_000_000_000, // 10K SOL
                    },
                ],
                outage_credits_bps: 500, // 5%
            },
        }
    }
}

#[derive(Clone)]
pub struct EnterpriseReport {
    pub client_id: u64,
    pub reporting_period_days: i64,
    pub total_transactions: u64,
    pub total_volume: u64,
    pub success_rate_bps: u16,
    pub avg_processing_time_ms: u32,
    pub compliance_score: u8,
    pub cost_savings: u16,
    pub roi_percentage: u16,
    pub sla_breaches: u16,
    pub uptime_achieved_bps: u16,
    pub generated_at: i64,
}