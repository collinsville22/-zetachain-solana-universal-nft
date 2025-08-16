use anchor_lang::prelude::*;
use crate::errors::UniversalNftError;
use crate::governance::{
    dao::{UniversalNftDAO, GovernanceStake, VotingDelegation},
    proposals::{Proposal, Vote, VoteType, ProposalStatus}
};

/// Advanced Voting System for Universal NFT Governance
/// Supports delegation, quadratic voting, and time-weighted voting
#[account]
#[derive(InitSpace)]
pub struct VotingSession {
    /// Session identifier
    pub session_id: u64,
    /// Associated proposal
    pub proposal: Pubkey,
    /// Session start time
    pub start_time: i64,
    /// Session end time
    pub end_time: i64,
    /// Total unique voters
    pub unique_voters: u32,
    /// Total voting power exercised
    pub total_voting_power: u64,
    /// Voting method configuration
    pub voting_method: VotingMethod,
    /// Session status
    pub status: VotingSessionStatus,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum VotingMethod {
    /// Standard linear voting (1 token = 1 vote)
    Linear,
    /// Quadratic voting (vote cost increases quadratically)
    Quadratic,
    /// Time-weighted voting (earlier votes have more weight)
    TimeWeighted,
    /// Conviction voting (longer stake = more weight)
    Conviction,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum VotingSessionStatus {
    Active,
    Completed,
    Cancelled,
}

/// Advanced voting calculator
pub struct VotingCalculator;

impl VotingCalculator {
    /// Calculate voting power based on stake and method
    pub fn calculate_voting_power(
        stake: &GovernanceStake,
        method: &VotingMethod,
        vote_amount: u64,
        voting_start: i64,
        current_time: i64,
    ) -> Result<u64> {
        let base_power = stake.voting_power;
        
        match method {
            VotingMethod::Linear => {
                // Standard linear voting: min(vote_amount, available_power)
                Ok(vote_amount.min(base_power))
            }
            
            VotingMethod::Quadratic => {
                // Quadratic voting: cost = votes^2
                let max_votes = Self::calculate_max_quadratic_votes(base_power);
                let actual_votes = vote_amount.min(max_votes);
                let cost = actual_votes * actual_votes;
                
                require!(cost <= base_power, UniversalNftError::ArithmeticOverflow);
                Ok(actual_votes)
            }
            
            VotingMethod::TimeWeighted => {
                // Earlier votes get bonus weight
                let time_elapsed = current_time - voting_start;
                let voting_duration = 7 * 24 * 3600; // 7 days default
                
                if time_elapsed <= 0 {
                    return Ok(vote_amount.min(base_power));
                }
                
                // Decay factor: starts at 1.5x, decays to 1.0x
                let decay_factor = 150 - ((time_elapsed * 50) / voting_duration).min(50);
                let weighted_power = (base_power * decay_factor as u64) / 100;
                
                Ok(vote_amount.min(weighted_power))
            }
            
            VotingMethod::Conviction => {
                // Longer stake duration = more voting power
                let stake_duration = current_time - stake.staked_at;
                let conviction_multiplier = Self::calculate_conviction_multiplier(stake_duration);
                let enhanced_power = (base_power * conviction_multiplier as u64) / 100;
                
                Ok(vote_amount.min(enhanced_power))
            }
        }
    }

    /// Calculate maximum votes for quadratic voting
    fn calculate_max_quadratic_votes(voting_power: u64) -> u64 {
        // votes = sqrt(voting_power)
        let sqrt_approx = Self::integer_sqrt(voting_power);
        sqrt_approx
    }

    /// Calculate conviction multiplier based on stake duration
    fn calculate_conviction_multiplier(stake_duration: i64) -> u16 {
        match stake_duration {
            0..=604800 => 100,           // 1 week: 1.0x
            604801..=2592000 => 110,     // 1 month: 1.1x
            2592001..=7776000 => 125,    // 3 months: 1.25x
            7776001..=15552000 => 150,   // 6 months: 1.5x
            15552001..=31104000 => 175,  // 1 year: 1.75x
            _ => 200,                    // >1 year: 2.0x
        }
    }

    /// Integer square root approximation
    fn integer_sqrt(n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        
        let mut x = n;
        let mut y = (x + 1) / 2;
        
        while y < x {
            x = y;
            y = (x + n / x) / 2;
        }
        
        x
    }
}

/// Voting delegation manager
pub struct DelegationManager;

impl DelegationManager {
    /// Get total voting power including delegations
    pub fn get_total_voting_power(
        voter: Pubkey,
        stake: &GovernanceStake,
        delegations: &[VotingDelegation],
    ) -> u64 {
        let mut total_power = stake.voting_power;
        
        // Add delegated voting power
        for delegation in delegations {
            if delegation.delegate == voter && delegation.is_valid() {
                total_power = total_power.saturating_add(delegation.voting_power);
            }
        }
        
        total_power
    }

    /// Check if voter can vote with delegated power
    pub fn can_vote_with_delegation(
        voter: Pubkey,
        delegation: &VotingDelegation,
    ) -> bool {
        delegation.delegate == voter && 
        delegation.is_active && 
        delegation.is_valid()
    }

    /// Split voting power across multiple votes
    pub fn split_voting_power(
        total_power: u64,
        splits: &[(VoteType, u16)], // (vote_type, percentage)
    ) -> Result<Vec<(VoteType, u64)>> {
        let total_percentage: u16 = splits.iter().map(|(_, pct)| *pct).sum();
        require!(total_percentage <= 10000, UniversalNftError::ArithmeticOverflow); // 100.00%

        let mut result = Vec::new();
        let mut remaining_power = total_power;

        for (i, (vote_type, percentage)) in splits.iter().enumerate() {
            let power = if i == splits.len() - 1 {
                // Last split gets remaining power to avoid rounding errors
                remaining_power
            } else {
                (total_power * (*percentage as u64)) / 10000
            };
            
            remaining_power = remaining_power.saturating_sub(power);
            result.push((vote_type.clone(), power));
        }

        Ok(result)
    }
}

/// Voting security and fraud detection
pub struct VotingSecurityChecker;

impl VotingSecurityChecker {
    /// Check for suspicious voting patterns
    pub fn check_voting_patterns(
        voter: Pubkey,
        voting_power: u64,
        vote_type: &VoteType,
        proposal: &Proposal,
        recent_votes: &[Vote],
    ) -> Result<SecurityCheck> {
        let mut risk_score = 0u16;
        let mut warnings = Vec::new();

        // Check for whale voting (large single vote)
        if voting_power > proposal.total_votes / 2 {
            risk_score += 30;
            warnings.push("Large voting power concentration detected".to_string());
        }

        // Check for rapid voting patterns
        let recent_voter_votes: Vec<_> = recent_votes.iter()
            .filter(|v| v.voter == voter)
            .collect();
            
        if recent_voter_votes.len() > 10 {
            risk_score += 20;
            warnings.push("High frequency voting detected".to_string());
        }

        // Check for last-minute votes (within 1 hour of deadline)
        let now = Clock::get()?.unix_timestamp;
        if proposal.voting_end - now < 3600 {
            risk_score += 10;
            warnings.push("Last-minute voting detected".to_string());
        }

        // Check for coordinated voting patterns
        if Self::detect_coordinated_voting(voter, recent_votes)? {
            risk_score += 40;
            warnings.push("Potential coordinated voting detected".to_string());
        }

        let security_level = match risk_score {
            0..=20 => SecurityLevel::Low,
            21..=50 => SecurityLevel::Medium,
            51..=80 => SecurityLevel::High,
            _ => SecurityLevel::Critical,
        };

        Ok(SecurityCheck {
            risk_score,
            security_level,
            warnings,
            requires_review: risk_score > 50,
        })
    }

    /// Detect potential coordinated voting
    fn detect_coordinated_voting(voter: Pubkey, recent_votes: &[Vote]) -> Result<bool> {
        let now = Clock::get()?.unix_timestamp;
        let time_window = 300; // 5 minutes
        
        // Count votes in the same time window with similar patterns
        let mut suspicious_count = 0;
        let voter_vote_time = recent_votes.iter()
            .find(|v| v.voter == voter)
            .map(|v| v.voted_at)
            .unwrap_or(now);

        for vote in recent_votes {
            if vote.voter != voter && 
               (vote.voted_at - voter_vote_time).abs() < time_window &&
               vote.voting_power == recent_votes.iter()
                   .find(|v| v.voter == voter)
                   .map(|v| v.voting_power)
                   .unwrap_or(0) {
                suspicious_count += 1;
            }
        }

        Ok(suspicious_count > 5) // More than 5 similar votes in 5 minutes
    }
}

#[derive(Clone)]
pub struct SecurityCheck {
    pub risk_score: u16,
    pub security_level: SecurityLevel,
    pub warnings: Vec<String>,
    pub requires_review: bool,
}

#[derive(Clone, PartialEq)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Vote verification and validation
pub struct VoteValidator;

impl VoteValidator {
    /// Comprehensive vote validation
    pub fn validate_vote(
        voter: Pubkey,
        proposal: &Proposal,
        voting_power: u64,
        stake: &GovernanceStake,
        delegations: &[VotingDelegation],
    ) -> Result<VoteValidation> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check basic voting eligibility
        if !proposal.can_vote(voter) {
            errors.push("Voting period has ended or proposal is not active".to_string());
        }

        // Check voting power availability
        let total_available_power = DelegationManager::get_total_voting_power(
            voter, stake, delegations
        );
        
        if voting_power > total_available_power {
            errors.push("Insufficient voting power".to_string());
        }

        // Check minimum voting power requirements
        if voting_power < 1000 { // Minimum 1000 tokens
            warnings.push("Very small voting power - consider delegating".to_string());
        }

        // Check stake lock requirements
        if !stake.can_unstake() && stake.lock_duration < 604800 { // Less than 1 week
            warnings.push("Short stake duration may affect voting weight".to_string());
        }

        let is_valid = errors.is_empty();

        Ok(VoteValidation {
            is_valid,
            errors,
            warnings,
            effective_voting_power: voting_power.min(total_available_power),
            delegation_count: delegations.len(),
        })
    }
}

#[derive(Clone)]
pub struct VoteValidation {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub effective_voting_power: u64,
    pub delegation_count: usize,
}

impl VotingSession {
    pub const INIT_SPACE: usize = 
        8 +     // session_id
        32 +    // proposal
        8 +     // start_time
        8 +     // end_time
        4 +     // unique_voters
        8 +     // total_voting_power
        1 +     // voting_method (enum)
        1 +     // status (enum)
        1;      // bump

    pub fn initialize(
        &mut self,
        session_id: u64,
        proposal: Pubkey,
        voting_method: VotingMethod,
        bump: u8,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        
        self.session_id = session_id;
        self.proposal = proposal;
        self.start_time = now;
        self.end_time = now + (7 * 24 * 3600); // 7 days default
        self.unique_voters = 0;
        self.total_voting_power = 0;
        self.voting_method = voting_method;
        self.status = VotingSessionStatus::Active;
        self.bump = bump;

        msg!("Voting session {} initialized", session_id);
        Ok(())
    }

    pub fn record_vote(&mut self, voting_power: u64, is_new_voter: bool) -> Result<()> {
        if is_new_voter {
            self.unique_voters = self.unique_voters.checked_add(1)
                .ok_or(UniversalNftError::ArithmeticOverflow)?;
        }
        
        self.total_voting_power = self.total_voting_power.checked_add(voting_power)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        Ok(())
    }

    pub fn finalize_session(&mut self) -> Result<()> {
        self.status = VotingSessionStatus::Completed;
        msg!("Voting session {} completed with {} voters and {} total voting power", 
             self.session_id, self.unique_voters, self.total_voting_power);
        Ok(())
    }
}