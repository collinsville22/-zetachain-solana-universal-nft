use anchor_lang::prelude::*;
use crate::errors::UniversalNftError;

/// Universal Ecosystem Adapter for Cross-Chain NFT Protocol
/// Enables seamless integration with multiple blockchain ecosystems and protocols
#[account]
#[derive(InitSpace)]
pub struct EcosystemAdapter {
    /// Adapter authority
    pub authority: Pubkey,
    /// Supported ecosystem count
    pub supported_ecosystems: u16,
    /// Total integrations active
    pub active_integrations: u32,
    /// Total cross-ecosystem transactions
    pub total_cross_ecosystem_txs: u64,
    /// Total value locked across ecosystems
    pub total_value_locked: u64,
    /// Adapter version
    pub version: String,
    /// Last integration update
    pub last_update: i64,
    /// Integration health score (0-100)
    pub health_score: u8,
    /// Emergency mode enabled
    pub emergency_mode: bool,
    /// PDA bump
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct EcosystemIntegration {
    /// Integration ID
    pub integration_id: u64,
    /// Ecosystem type
    pub ecosystem_type: EcosystemType,
    /// Integration name
    pub name: String,
    /// Integration version
    pub version: String,
    /// Integration status
    pub status: IntegrationStatus,
    /// Supported features
    pub supported_features: Vec<IntegrationFeature>,
    /// Configuration parameters
    pub config: IntegrationConfig,
    /// Performance metrics
    pub metrics: IntegrationMetrics,
    /// Last health check timestamp
    pub last_health_check: i64,
    /// Integration created timestamp
    pub created_at: i64,
    /// SLA tier
    pub sla_tier: SLATier,
    /// Revenue sharing percentage (basis points)
    pub revenue_share_bps: u16,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum EcosystemType {
    /// Decentralized Finance
    DeFi,
    /// Gaming and Metaverse
    Gaming,
    /// NFT Marketplaces
    Marketplace,
    /// Traditional Finance
    TradFi,
    /// Social Platforms
    Social,
    /// Enterprise Solutions
    Enterprise,
    /// Mobile Applications
    Mobile,
    /// Cloud Infrastructure
    Cloud,
    /// Analytics and Data
    Analytics,
    /// Security and Compliance
    Security,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum IntegrationStatus {
    Active,
    Inactive,
    Maintenance,
    Deprecated,
    Beta,
    Pending,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum IntegrationFeature {
    CrossChainTransfer,
    Fractionalization,
    Lending,
    Staking,
    MarketplaceListings,
    PriceOracles,
    Analytics,
    Governance,
    Compliance,
    LiquidityProvision,
    YieldFarming,
    Gaming,
    Metaverse,
    SocialIdentity,
    PaymentProcessing,
    Custody,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct IntegrationConfig {
    /// Maximum daily transaction volume
    pub max_daily_volume: u64,
    /// Fee percentage (basis points)
    pub fee_bps: u16,
    /// Timeout settings (seconds)
    pub timeout_seconds: u32,
    /// Retry attempts
    pub max_retries: u8,
    /// Rate limiting (requests per minute)
    pub rate_limit_rpm: u32,
    /// Minimum stake required
    pub min_stake_amount: u64,
    /// Whitelist enabled
    pub whitelist_enabled: bool,
    /// KYC required
    pub kyc_required: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct IntegrationMetrics {
    /// Total transactions processed
    pub total_transactions: u64,
    /// Successful transactions
    pub successful_transactions: u64,
    /// Failed transactions
    pub failed_transactions: u64,
    /// Average processing time (ms)
    pub avg_processing_time_ms: u32,
    /// Total fees generated
    pub total_fees_generated: u64,
    /// Uptime percentage (basis points)
    pub uptime_bps: u16,
    /// Last 24h volume
    pub volume_24h: u64,
    /// Peak TPS achieved
    pub peak_tps: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum SLATier {
    Basic,      // 99% uptime
    Standard,   // 99.5% uptime
    Premium,    // 99.9% uptime
    Enterprise, // 99.99% uptime
}

#[account]
#[derive(InitSpace)]
pub struct PartnershipAgreement {
    /// Agreement ID
    pub agreement_id: u64,
    /// Partner organization
    pub partner: Pubkey,
    /// Partnership type
    pub partnership_type: PartnershipType,
    /// Agreement terms
    pub terms: PartnershipTerms,
    /// Agreement status
    pub status: AgreementStatus,
    /// Revenue sharing model
    pub revenue_model: RevenueModel,
    /// Performance requirements
    pub performance_requirements: PerformanceRequirements,
    /// Agreement start date
    pub start_date: i64,
    /// Agreement end date
    pub end_date: i64,
    /// Auto-renewal enabled
    pub auto_renewal: bool,
    /// Agreement signed timestamp
    pub signed_at: i64,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum PartnershipType {
    Technology,
    Strategic,
    Revenue,
    Marketing,
    Development,
    Distribution,
    Investment,
    Academic,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PartnershipTerms {
    /// Revenue share percentage (basis points)
    pub revenue_share_bps: u16,
    /// Minimum volume commitment
    pub min_volume_commitment: u64,
    /// Exclusivity terms
    pub exclusivity_regions: Vec<String>,
    /// Technical support included
    pub technical_support: bool,
    /// Marketing co-investment required
    pub marketing_contribution_bps: u16,
    /// Data sharing permissions
    pub data_sharing_level: DataSharingLevel,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum AgreementStatus {
    Draft,
    Negotiating,
    Pending,
    Active,
    Suspended,
    Terminated,
    Expired,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RevenueModel {
    /// Base fee structure
    pub base_fee_bps: u16,
    /// Volume-based tiers
    pub volume_tiers: Vec<VolumeTier>,
    /// Performance bonuses
    pub performance_bonuses: Vec<PerformanceBonus>,
    /// Payment frequency
    pub payment_frequency: PaymentFrequency,
    /// Minimum payout threshold
    pub min_payout_threshold: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct VolumeTier {
    /// Minimum volume for tier
    pub min_volume: u64,
    /// Fee percentage for tier (basis points)
    pub fee_bps: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PerformanceBonus {
    /// Performance metric
    pub metric: PerformanceMetric,
    /// Threshold value
    pub threshold: u64,
    /// Bonus percentage (basis points)
    pub bonus_bps: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum PerformanceMetric {
    Uptime,
    Volume,
    UserAcquisition,
    RetentionRate,
    ErrorRate,
    LatencyP95,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum PaymentFrequency {
    Weekly,
    Monthly,
    Quarterly,
    Annually,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PerformanceRequirements {
    /// Minimum uptime (basis points)
    pub min_uptime_bps: u16,
    /// Maximum response time (ms)
    pub max_response_time_ms: u32,
    /// Minimum throughput (TPS)
    pub min_throughput_tps: u16,
    /// Maximum error rate (basis points)
    pub max_error_rate_bps: u16,
    /// Security compliance level
    pub security_compliance: SecurityCompliance,
    /// Data retention period (days)
    pub data_retention_days: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum DataSharingLevel {
    None,
    Aggregate,
    Anonymized,
    Full,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum SecurityCompliance {
    Basic,
    Standard,
    SOC2,
    ISO27001,
    PCI_DSS,
    GDPR,
    Custom,
}

impl EcosystemAdapter {
    pub const INIT_SPACE: usize = 
        32 +    // authority
        2 +     // supported_ecosystems
        4 +     // active_integrations
        8 +     // total_cross_ecosystem_txs
        8 +     // total_value_locked
        4 + 32 + // version (String)
        8 +     // last_update
        1 +     // health_score
        1 +     // emergency_mode
        1;      // bump

    /// Initialize ecosystem adapter
    pub fn initialize(
        &mut self,
        authority: Pubkey,
        version: String,
        bump: u8,
    ) -> Result<()> {
        require!(version.len() <= 32, UniversalNftError::InvalidTransferStatus);
        
        let now = Clock::get()?.unix_timestamp;
        
        self.authority = authority;
        self.supported_ecosystems = 0;
        self.active_integrations = 0;
        self.total_cross_ecosystem_txs = 0;
        self.total_value_locked = 0;
        self.version = version.clone();
        self.last_update = now;
        self.health_score = 100;
        self.emergency_mode = false;
        self.bump = bump;

        msg!("Ecosystem adapter initialized");
        msg!("Version: {}", version);

        Ok(())
    }

    /// Register new ecosystem integration
    pub fn register_integration(
        &mut self,
        integration: &mut EcosystemIntegration,
        integration_id: u64,
        ecosystem_type: EcosystemType,
        name: String,
        version: String,
        features: Vec<IntegrationFeature>,
        config: IntegrationConfig,
        sla_tier: SLATier,
        revenue_share_bps: u16,
    ) -> Result<()> {
        require!(name.len() <= 64, UniversalNftError::InvalidTransferStatus);
        require!(version.len() <= 16, UniversalNftError::InvalidTransferStatus);
        require!(revenue_share_bps <= 10000, UniversalNftError::InvalidTransferStatus);

        let now = Clock::get()?.unix_timestamp;

        // Initialize integration
        integration.integration_id = integration_id;
        integration.ecosystem_type = ecosystem_type;
        integration.name = name.clone();
        integration.version = version;
        integration.status = IntegrationStatus::Pending;
        integration.supported_features = features;
        integration.config = config;
        integration.metrics = IntegrationMetrics {
            total_transactions: 0,
            successful_transactions: 0,
            failed_transactions: 0,
            avg_processing_time_ms: 0,
            total_fees_generated: 0,
            uptime_bps: 10000, // Start at 100%
            volume_24h: 0,
            peak_tps: 0,
        };
        integration.last_health_check = now;
        integration.created_at = now;
        integration.sla_tier = sla_tier;
        integration.revenue_share_bps = revenue_share_bps;

        // Update adapter statistics
        self.supported_ecosystems = self.supported_ecosystems.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        self.last_update = now;

        msg!("Integration registered: {} (ID: {})", name, integration_id);
        Ok(())
    }

    /// Activate integration after validation
    pub fn activate_integration(
        &mut self,
        integration: &mut EcosystemIntegration,
    ) -> Result<()> {
        require!(integration.status == IntegrationStatus::Pending, UniversalNftError::InvalidTransferStatus);

        integration.status = IntegrationStatus::Active;
        self.active_integrations = self.active_integrations.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        msg!("Integration activated: {}", integration.name);
        Ok(())
    }

    /// Record cross-ecosystem transaction
    pub fn record_cross_ecosystem_transaction(
        &mut self,
        integration: &mut EcosystemIntegration,
        transaction_value: u64,
        processing_time_ms: u32,
        success: bool,
    ) -> Result<()> {
        // Update adapter metrics
        self.total_cross_ecosystem_txs = self.total_cross_ecosystem_txs.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        if success {
            self.total_value_locked = self.total_value_locked.checked_add(transaction_value)
                .ok_or(UniversalNftError::ArithmeticOverflow)?;
        }

        // Update integration metrics
        integration.metrics.total_transactions = integration.metrics.total_transactions.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        if success {
            integration.metrics.successful_transactions = integration.metrics.successful_transactions.checked_add(1)
                .ok_or(UniversalNftError::ArithmeticOverflow)?;
        } else {
            integration.metrics.failed_transactions = integration.metrics.failed_transactions.checked_add(1)
                .ok_or(UniversalNftError::ArithmeticOverflow)?;
        }

        // Update average processing time
        let total_txs = integration.metrics.total_transactions;
        integration.metrics.avg_processing_time_ms = 
            ((integration.metrics.avg_processing_time_ms as u64 * (total_txs - 1)) + processing_time_ms as u64) as u32 / total_txs as u32;

        // Update 24h volume
        integration.metrics.volume_24h = integration.metrics.volume_24h.checked_add(transaction_value)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        Ok(())
    }

    /// Create partnership agreement
    pub fn create_partnership(
        &mut self,
        agreement: &mut PartnershipAgreement,
        agreement_id: u64,
        partner: Pubkey,
        partnership_type: PartnershipType,
        terms: PartnershipTerms,
        revenue_model: RevenueModel,
        performance_requirements: PerformanceRequirements,
        duration_days: i64,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        agreement.agreement_id = agreement_id;
        agreement.partner = partner;
        agreement.partnership_type = partnership_type;
        agreement.terms = terms;
        agreement.status = AgreementStatus::Draft;
        agreement.revenue_model = revenue_model;
        agreement.performance_requirements = performance_requirements;
        agreement.start_date = now;
        agreement.end_date = now + (duration_days * 24 * 3600);
        agreement.auto_renewal = false;
        agreement.signed_at = 0;

        msg!("Partnership agreement {} created with {}", agreement_id, partner);
        Ok(())
    }

    /// Sign partnership agreement
    pub fn sign_partnership(
        &mut self,
        agreement: &mut PartnershipAgreement,
    ) -> Result<()> {
        require!(agreement.status == AgreementStatus::Draft, UniversalNftError::InvalidTransferStatus);

        let now = Clock::get()?.unix_timestamp;
        agreement.status = AgreementStatus::Active;
        agreement.signed_at = now;

        msg!("Partnership agreement {} signed", agreement.agreement_id);
        Ok(())
    }

    /// Perform health check on integration
    pub fn health_check_integration(
        &mut self,
        integration: &mut EcosystemIntegration,
    ) -> Result<u8> {
        let now = Clock::get()?.unix_timestamp;
        
        // Calculate health score based on metrics
        let success_rate = if integration.metrics.total_transactions > 0 {
            (integration.metrics.successful_transactions * 100) / integration.metrics.total_transactions
        } else {
            100
        };

        let uptime_score = (integration.metrics.uptime_bps / 100) as u64;
        let performance_score = if integration.metrics.avg_processing_time_ms < 1000 {
            100
        } else if integration.metrics.avg_processing_time_ms < 5000 {
            80
        } else {
            50
        };

        let health_score = ((success_rate + uptime_score + performance_score) / 3) as u8;
        
        integration.last_health_check = now;

        // Update integration status based on health
        if health_score < 50 {
            integration.status = IntegrationStatus::Maintenance;
        } else if integration.status == IntegrationStatus::Maintenance && health_score > 80 {
            integration.status = IntegrationStatus::Active;
        }

        msg!("Health check for {}: {}%", integration.name, health_score);
        Ok(health_score)
    }

    /// Get ecosystem statistics
    pub fn get_ecosystem_stats(&self) -> EcosystemStats {
        EcosystemStats {
            supported_ecosystems: self.supported_ecosystems,
            active_integrations: self.active_integrations,
            total_cross_ecosystem_txs: self.total_cross_ecosystem_txs,
            total_value_locked: self.total_value_locked,
            health_score: self.health_score,
            emergency_mode: self.emergency_mode,
            last_update: self.last_update,
            version: self.version.clone(),
        }
    }
}

#[derive(Clone)]
pub struct EcosystemStats {
    pub supported_ecosystems: u16,
    pub active_integrations: u32,
    pub total_cross_ecosystem_txs: u64,
    pub total_value_locked: u64,
    pub health_score: u8,
    pub emergency_mode: bool,
    pub last_update: i64,
    pub version: String,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            max_daily_volume: 1_000_000_000_000, // 1M SOL equivalent
            fee_bps: 100,                         // 1%
            timeout_seconds: 30,
            max_retries: 3,
            rate_limit_rpm: 1000,
            min_stake_amount: 100_000_000_000, // 100 SOL
            whitelist_enabled: false,
            kyc_required: false,
        }
    }
}