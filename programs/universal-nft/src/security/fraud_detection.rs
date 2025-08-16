use anchor_lang::prelude::*;
use crate::errors::UniversalNftError;
use std::collections::HashMap;

/// Advanced ML-Inspired Fraud Detection System
/// Detects suspicious patterns in cross-chain NFT operations
#[account]
#[derive(InitSpace)]
pub struct FraudDetectionEngine {
    /// Current risk score (0-1000)
    pub risk_score: u16,
    /// Number of suspicious patterns detected
    pub suspicious_patterns: u64,
    /// Total operations analyzed
    pub total_operations: u64,
    /// Last analysis timestamp
    pub last_analysis: i64,
    /// Configuration parameters
    pub config: FraudConfig,
    /// Authority for manual overrides
    pub authority: Pubkey,
    /// Recent operation signatures for pattern detection
    pub recent_operations: [OperationSignature; 20],
    /// Current position in circular buffer
    pub operation_index: u8,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct FraudConfig {
    /// Risk threshold for blocking operations
    pub risk_threshold: u16,
    /// Time window for pattern analysis (seconds)
    pub analysis_window: i64,
    /// Velocity threshold (operations per minute)
    pub velocity_threshold: u16,
    /// Minimum reputation score required
    pub min_reputation: u16,
    /// Geographic risk multiplier
    pub geo_risk_multiplier: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default)]
pub struct OperationSignature {
    /// Operation type
    pub op_type: u8,
    /// Timestamp
    pub timestamp: i64,
    /// Source chain ID
    pub source_chain: u64,
    /// Destination chain ID  
    pub destination_chain: u64,
    /// Value hash (for amount/value patterns)
    pub value_hash: u32,
    /// User address hash
    pub user_hash: u32,
    /// Risk score for this operation
    pub risk_score: u16,
}

impl Default for FraudConfig {
    fn default() -> Self {
        Self {
            risk_threshold: 750,      // 75% risk threshold
            analysis_window: 3600,    // 1 hour
            velocity_threshold: 10,   // 10 ops per minute
            min_reputation: 500,      // 50% minimum reputation
            geo_risk_multiplier: 150, // 1.5x for high-risk regions
        }
    }
}

impl FraudDetectionEngine {
    pub const INIT_SPACE: usize = 
        2 +     // risk_score
        8 +     // suspicious_patterns
        8 +     // total_operations
        8 +     // last_analysis
        2 * 5 + // config (5 u16s)
        32 +    // authority
        (2 + 8 + 8 + 8 + 4 + 4 + 2) * 20 + // recent_operations array
        1 +     // operation_index
        1;      // bump

    /// Initialize fraud detection engine
    pub fn initialize(&mut self, authority: Pubkey, config: Option<FraudConfig>, bump: u8) {
        self.risk_score = 0;
        self.suspicious_patterns = 0;
        self.total_operations = 0;
        self.last_analysis = Clock::get().unwrap().unix_timestamp;
        self.config = config.unwrap_or_default();
        self.authority = authority;
        self.recent_operations = [OperationSignature::default(); 20];
        self.operation_index = 0;
        self.bump = bump;
    }

    /// Analyze operation for fraud indicators
    pub fn analyze_operation(&mut self, operation: &OperationAnalysisInput) -> Result<FraudAnalysisResult> {
        let now = Clock::get()?.unix_timestamp;
        
        // Create operation signature
        let signature = OperationSignature {
            op_type: operation.operation_type as u8,
            timestamp: now,
            source_chain: operation.source_chain_id,
            destination_chain: operation.destination_chain_id,
            value_hash: self.hash_value(operation.value),
            user_hash: self.hash_address(&operation.user_address),
            risk_score: 0, // Will be calculated
        };

        // Add to recent operations (circular buffer)
        self.recent_operations[self.operation_index as usize] = signature;
        self.operation_index = (self.operation_index + 1) % 20;
        self.total_operations = self.total_operations.saturating_add(1);

        // Perform comprehensive fraud analysis
        let risk_score = self.calculate_comprehensive_risk_score(operation, now)?;
        
        // Update global risk score with exponential moving average
        self.risk_score = self.update_risk_score(risk_score);
        
        // Check for suspicious patterns
        let suspicious_patterns = self.detect_patterns()?;
        self.suspicious_patterns = self.suspicious_patterns.saturating_add(suspicious_patterns as u64);

        let result = FraudAnalysisResult {
            risk_score,
            is_suspicious: risk_score > self.config.risk_threshold,
            detected_patterns: suspicious_patterns,
            recommendation: self.get_recommendation(risk_score),
            confidence: self.calculate_confidence(),
        };

        // Log significant findings
        if result.is_suspicious {
            msg!("🚨 FRAUD ALERT: High-risk operation detected (score: {})", risk_score);
        }

        self.last_analysis = now;
        Ok(result)
    }

    /// Calculate comprehensive risk score using multiple factors
    fn calculate_comprehensive_risk_score(&self, operation: &OperationAnalysisInput, now: i64) -> Result<u16> {
        let mut risk_factors = Vec::new();

        // 1. Velocity Analysis (frequency-based risk)
        let velocity_risk = self.analyze_velocity(now)?;
        risk_factors.push(("velocity", velocity_risk));

        // 2. Chain Pair Risk Analysis
        let chain_risk = self.analyze_chain_pair_risk(operation.source_chain_id, operation.destination_chain_id);
        risk_factors.push(("chain_pair", chain_risk));

        // 3. Value Pattern Analysis
        let value_risk = self.analyze_value_patterns(operation.value);
        risk_factors.push(("value_pattern", value_risk));

        // 4. Time-based Analysis (unusual hours, etc.)
        let temporal_risk = self.analyze_temporal_patterns(now);
        risk_factors.push(("temporal", temporal_risk));

        // 5. User Behavior Analysis
        let behavior_risk = self.analyze_user_behavior(&operation.user_address, now)?;
        risk_factors.push(("behavior", behavior_risk));

        // 6. Cross-Chain Route Analysis
        let route_risk = self.analyze_route_risk(operation)?;
        risk_factors.push(("route", route_risk));

        // 7. Reputation-based Risk
        let reputation_risk = self.calculate_reputation_risk(operation.user_reputation);
        risk_factors.push(("reputation", reputation_risk));

        // Weighted risk calculation
        let total_risk = self.calculate_weighted_risk(&risk_factors);
        
        msg!("🔍 Risk Analysis: velocity={}, chain={}, value={}, temporal={}, behavior={}, route={}, reputation={} -> total={}",
            velocity_risk, chain_risk, value_risk, temporal_risk, behavior_risk, route_risk, reputation_risk, total_risk);

        Ok(total_risk.min(1000))
    }

    /// Analyze transaction velocity for suspicious patterns
    fn analyze_velocity(&self, now: i64) -> Result<u16> {
        let window_start = now - 60; // 1 minute window
        let recent_count = self.recent_operations
            .iter()
            .filter(|op| op.timestamp > window_start && op.timestamp > 0)
            .count();

        let velocity = recent_count as u16;
        
        if velocity > self.config.velocity_threshold {
            Ok(((velocity - self.config.velocity_threshold) * 50).min(500))
        } else {
            Ok(0)
        }
    }

    /// Analyze risk based on chain pair
    fn analyze_chain_pair_risk(&self, source: u64, destination: u64) -> u16 {
        // High-risk chain pairs (based on known attack patterns)
        let high_risk_chains = [99999, 88888, 77777]; // Example suspicious chain IDs
        
        let source_risk = if high_risk_chains.contains(&source) { 200 } else { 0 };
        let dest_risk = if high_risk_chains.contains(&destination) { 200 } else { 0 };
        
        // Unusual chain combinations
        let combination_risk = match (source, destination) {
            (900, 1) | (1, 900) => 50,  // Solana <-> Ethereum (normal)
            (900, 56) | (56, 900) => 50, // Solana <-> BSC (normal)
            (900, 7000) | (7000, 900) => 30, // Solana <-> ZetaChain (normal)
            _ => 100, // Unusual combinations
        };

        (source_risk + dest_risk + combination_risk).min(500)
    }

    /// Analyze value patterns for suspicious amounts
    fn analyze_value_patterns(&self, value: u64) -> u16 {
        // Round number detection (often used in attacks)
        let round_number_risk = if value % 1000000 == 0 && value > 0 { 100 } else { 0 };
        
        // Extremely high values
        let high_value_risk = if value > 1000000000000 { 200 } else { 0 }; // > 1T units
        
        // Suspicious exact amounts
        let exact_amount_risk = if value == 1337 || value == 69420 { 150 } else { 0 };

        (round_number_risk + high_value_risk + exact_amount_risk).min(300)
    }

    /// Analyze temporal patterns
    fn analyze_temporal_patterns(&self, timestamp: i64) -> u16 {
        // Convert to hours (UTC)
        let hour = ((timestamp % 86400) / 3600) as u8;
        
        // Higher risk during unusual hours (2-6 AM UTC when most users sleep)
        match hour {
            2..=5 => 100,
            6..=7 | 23..=1 => 50,
            _ => 0,
        }
    }

    /// Analyze user behavior patterns
    fn analyze_user_behavior(&self, user_address: &[u8], _now: i64) -> Result<u16> {
        let user_hash = self.hash_address(user_address);
        
        // Count recent operations by this user
        let user_ops = self.recent_operations
            .iter()
            .filter(|op| op.user_hash == user_hash && op.timestamp > 0)
            .count();

        // Rapid repeated operations by same user
        if user_ops > 5 {
            Ok(((user_ops - 5) * 50) as u16)
        } else {
            Ok(0)
        }
    }

    /// Analyze cross-chain route complexity
    fn analyze_route_risk(&self, operation: &OperationAnalysisInput) -> Result<u16> {
        // Complex routing through multiple chains increases risk
        let route_complexity = operation.route_hops.unwrap_or(1);
        
        match route_complexity {
            1 => 0,      // Direct transfer
            2 => 50,     // One intermediate
            3 => 150,    // Two intermediates  
            4..=u8::MAX => 300, // Highly complex routing
        }
    }

    /// Calculate reputation-based risk
    fn calculate_reputation_risk(&self, reputation: Option<u16>) -> u16 {
        match reputation {
            Some(rep) if rep >= self.config.min_reputation => 0,
            Some(rep) => (self.config.min_reputation - rep).min(400),
            None => 200, // Unknown reputation
        }
    }

    /// Calculate weighted risk from multiple factors
    fn calculate_weighted_risk(&self, factors: &[(&str, u16)]) -> u16 {
        let weights = [
            ("velocity", 25),     // 25% weight
            ("chain_pair", 20),   // 20% weight
            ("value_pattern", 15), // 15% weight
            ("temporal", 10),     // 10% weight
            ("behavior", 15),     // 15% weight
            ("route", 10),        // 10% weight
            ("reputation", 5),    // 5% weight
        ];

        let mut weighted_sum = 0u32;
        let mut total_weight = 0u32;

        for (factor_name, risk) in factors {
            if let Some((_, weight)) = weights.iter().find(|(name, _)| name == factor_name) {
                weighted_sum += (*risk as u32) * (*weight as u32);
                total_weight += *weight as u32;
            }
        }

        if total_weight > 0 {
            (weighted_sum / total_weight) as u16
        } else {
            0
        }
    }

    /// Detect suspicious patterns in recent operations
    fn detect_patterns(&self) -> Result<u16> {
        let mut patterns = 0u16;

        // Pattern 1: Rapid-fire operations
        let rapid_fire = self.detect_rapid_fire_pattern()?;
        if rapid_fire { patterns += 1; }

        // Pattern 2: Circular transfers (A->B->A)
        let circular = self.detect_circular_pattern()?;
        if circular { patterns += 1; }

        // Pattern 3: Value splitting/combining
        let value_manipulation = self.detect_value_manipulation_pattern()?;
        if value_manipulation { patterns += 1; }

        // Pattern 4: Chain hopping
        let chain_hopping = self.detect_chain_hopping_pattern()?;
        if chain_hopping { patterns += 1; }

        Ok(patterns)
    }

    fn detect_rapid_fire_pattern(&self) -> Result<bool> {
        let now = Clock::get()?.unix_timestamp;
        let recent_ops = self.recent_operations
            .iter()
            .filter(|op| op.timestamp > now - 60 && op.timestamp > 0)
            .count();
        
        Ok(recent_ops >= 10) // 10+ operations in 1 minute
    }

    fn detect_circular_pattern(&self) -> Result<bool> {
        // Look for A->B->A patterns in chain transfers
        for i in 0..18 {
            let op1 = &self.recent_operations[i];
            let op2 = &self.recent_operations[i + 1];
            let op3 = &self.recent_operations[i + 2];
            
            if op1.timestamp > 0 && op2.timestamp > 0 && op3.timestamp > 0 {
                if op1.source_chain == op3.destination_chain &&
                   op1.destination_chain == op3.source_chain &&
                   op1.user_hash == op2.user_hash && op2.user_hash == op3.user_hash {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    fn detect_value_manipulation_pattern(&self) -> Result<bool> {
        // Detect splitting large amounts into smaller ones
        let recent_values: Vec<u32> = self.recent_operations
            .iter()
            .filter(|op| op.timestamp > 0)
            .map(|op| op.value_hash)
            .collect();

        // Simple heuristic: many operations with similar value hashes
        let mut value_counts = std::collections::HashMap::new();
        for value in recent_values {
            *value_counts.entry(value).or_insert(0) += 1;
        }

        Ok(value_counts.values().any(|&count| count >= 5))
    }

    fn detect_chain_hopping_pattern(&self) -> Result<bool> {
        // Detect excessive chain switching
        let chain_switches = self.recent_operations
            .windows(2)
            .filter(|window| {
                window[0].timestamp > 0 && window[1].timestamp > 0 &&
                window[0].destination_chain != window[1].source_chain
            })
            .count();

        Ok(chain_switches >= 5)
    }

    // Helper methods
    fn hash_value(&self, value: u64) -> u32 {
        // Simple hash for value patterns
        ((value >> 32) as u32) ^ (value as u32)
    }

    fn hash_address(&self, address: &[u8]) -> u32 {
        // Simple hash for address patterns
        let mut hash = 0u32;
        for (i, &byte) in address.iter().take(4).enumerate() {
            hash ^= (byte as u32) << (i * 8);
        }
        hash
    }

    fn update_risk_score(&self, new_risk: u16) -> u16 {
        // Exponential moving average: 0.7 * old + 0.3 * new
        let alpha = 30; // 30% for new value
        ((self.risk_score as u32 * (100 - alpha) + new_risk as u32 * alpha) / 100) as u16
    }

    fn get_recommendation(&self, risk_score: u16) -> FraudRecommendation {
        match risk_score {
            0..=200 => FraudRecommendation::Allow,
            201..=500 => FraudRecommendation::Monitor,
            501..=750 => FraudRecommendation::RequireAdditionalVerification,
            751..=900 => FraudRecommendation::Delay,
            _ => FraudRecommendation::Block,
        }
    }

    fn calculate_confidence(&self) -> u8 {
        // Confidence increases with number of analyzed operations
        let operations_factor = (self.total_operations.min(100) * 80 / 100) as u8;
        let pattern_factor = (self.suspicious_patterns.min(10) * 20 / 10) as u8;
        (operations_factor + pattern_factor).min(100)
    }
}

pub struct OperationAnalysisInput {
    pub operation_type: OperationType,
    pub source_chain_id: u64,
    pub destination_chain_id: u64,
    pub value: u64,
    pub user_address: Vec<u8>,
    pub user_reputation: Option<u16>,
    pub route_hops: Option<u8>,
}

pub struct FraudAnalysisResult {
    pub risk_score: u16,
    pub is_suspicious: bool,
    pub detected_patterns: u16,
    pub recommendation: FraudRecommendation,
    pub confidence: u8,
}

#[derive(Clone, Copy)]
pub enum OperationType {
    CrossChainTransfer = 1,
    LocalTransfer = 2,
    Mint = 3,
    Burn = 4,
}

#[derive(Clone, Copy)]
pub enum FraudRecommendation {
    Allow,
    Monitor,
    RequireAdditionalVerification,
    Delay,
    Block,
}