use anchor_lang::prelude::*;
use crate::errors::UniversalNftError;

/// Decentralized Autonomous Organization for Universal NFT Protocol Governance
/// Implements a sophisticated governance system with timelock, delegation, and treasury management
#[account]
#[derive(InitSpace)]
pub struct UniversalNftDAO {
    /// DAO authority (can be upgraded to a DAO program)
    pub authority: Pubkey,
    /// Treasury account holding protocol funds
    pub treasury: Pubkey,
    /// Governance token mint (for voting power)
    pub governance_token: Pubkey,
    /// Minimum tokens required to create proposals
    pub proposal_threshold: u64,
    /// Minimum voting period in seconds
    pub min_voting_period: i64,
    /// Maximum voting period in seconds
    pub max_voting_period: i64,
    /// Quorum threshold (percentage of total supply needed)
    pub quorum_threshold: u16,
    /// Execution delay after proposal passes (timelock)
    pub execution_delay: i64,
    /// Current proposal count
    pub proposal_count: u64,
    /// Active proposals count
    pub active_proposals: u32,
    /// Total governance tokens staked
    pub total_staked: u64,
    /// DAO creation timestamp
    pub created_at: i64,
    /// Last proposal timestamp
    pub last_proposal_at: i64,
    /// Emergency pause authority
    pub emergency_council: Pubkey,
    /// Whether the DAO is paused
    pub is_paused: bool,
    /// PDA bump
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct GovernanceStake {
    /// Staker's public key
    pub staker: Pubkey,
    /// Amount of governance tokens staked
    pub amount: u64,
    /// Timestamp when staked
    pub staked_at: i64,
    /// Lock duration in seconds
    pub lock_duration: i64,
    /// Voting power multiplier based on lock duration
    pub power_multiplier: u16,
    /// Delegated voting power (if any)
    pub delegated_to: Option<Pubkey>,
    /// Current voting power
    pub voting_power: u64,
    /// Rewards accumulated
    pub rewards_accumulated: u64,
    /// Last reward claim timestamp
    pub last_reward_claim: i64,
    /// PDA bump
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct VotingDelegation {
    /// Delegator (token holder)
    pub delegator: Pubkey,
    /// Delegate (receives voting power)
    pub delegate: Pubkey,
    /// Amount of voting power delegated
    pub voting_power: u64,
    /// Delegation timestamp
    pub delegated_at: i64,
    /// Delegation expiry (optional)
    pub expires_at: Option<i64>,
    /// Whether delegation is active
    pub is_active: bool,
    /// PDA bump
    pub bump: u8,
}

impl UniversalNftDAO {
    pub const INIT_SPACE: usize = 
        32 + // authority
        32 + // treasury
        32 + // governance_token
        8 +  // proposal_threshold
        8 +  // min_voting_period
        8 +  // max_voting_period
        2 +  // quorum_threshold
        8 +  // execution_delay
        8 +  // proposal_count
        4 +  // active_proposals
        8 +  // total_staked
        8 +  // created_at
        8 +  // last_proposal_at
        32 + // emergency_council
        1 +  // is_paused
        1;   // bump

    /// Initialize the DAO with governance parameters
    pub fn initialize(
        &mut self,
        authority: Pubkey,
        treasury: Pubkey,
        governance_token: Pubkey,
        emergency_council: Pubkey,
        config: DAOConfig,
        bump: u8,
    ) -> Result<()> {
        self.authority = authority;
        self.treasury = treasury;
        self.governance_token = governance_token;
        self.emergency_council = emergency_council;
        
        // Set governance parameters
        self.proposal_threshold = config.proposal_threshold;
        self.min_voting_period = config.min_voting_period;
        self.max_voting_period = config.max_voting_period;
        self.quorum_threshold = config.quorum_threshold;
        self.execution_delay = config.execution_delay;
        
        // Initialize state
        self.proposal_count = 0;
        self.active_proposals = 0;
        self.total_staked = 0;
        self.created_at = Clock::get()?.unix_timestamp;
        self.last_proposal_at = 0;
        self.is_paused = false;
        self.bump = bump;

        msg!("Universal NFT DAO initialized");
        msg!("Governance token: {}", governance_token);
        msg!("Proposal threshold: {}", config.proposal_threshold);
        msg!("Quorum threshold: {}%", config.quorum_threshold);

        Ok(())
    }

    /// Stake governance tokens for voting power
    pub fn stake_tokens(
        &mut self,
        stake_account: &mut GovernanceStake,
        amount: u64,
        lock_duration: i64,
    ) -> Result<()> {
        require!(!self.is_paused, UniversalNftError::ProgramPaused);
        
        let now = Clock::get()?.unix_timestamp;
        
        // Calculate voting power multiplier based on lock duration
        let power_multiplier = self.calculate_power_multiplier(lock_duration);
        let voting_power = amount * power_multiplier as u64 / 100;

        // Update stake account
        stake_account.amount = stake_account.amount.checked_add(amount)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        stake_account.staked_at = now;
        stake_account.lock_duration = lock_duration;
        stake_account.power_multiplier = power_multiplier;
        stake_account.voting_power = stake_account.voting_power.checked_add(voting_power)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        // Update DAO total
        self.total_staked = self.total_staked.checked_add(amount)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        msg!("Tokens staked: {} with {}x multiplier", amount, power_multiplier);
        Ok(())
    }

    /// Unstake governance tokens (after lock period)
    pub fn unstake_tokens(
        &mut self,
        stake_account: &mut GovernanceStake,
        amount: u64,
    ) -> Result<()> {
        require!(!self.is_paused, UniversalNftError::ProgramPaused);
        
        let now = Clock::get()?.unix_timestamp;
        
        // Check if lock period has expired
        require!(
            now >= stake_account.staked_at + stake_account.lock_duration,
            UniversalNftError::InvalidTransferStatus // Reusing error for lock period
        );

        // Check sufficient staked amount
        require!(
            stake_account.amount >= amount,
            UniversalNftError::ArithmeticOverflow
        );

        // Calculate voting power reduction
        let power_reduction = amount * stake_account.power_multiplier as u64 / 100;

        // Update stake account
        stake_account.amount = stake_account.amount.checked_sub(amount)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        stake_account.voting_power = stake_account.voting_power.checked_sub(power_reduction)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        // Update DAO total
        self.total_staked = self.total_staked.checked_sub(amount)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        msg!("Tokens unstaked: {}", amount);
        Ok(())
    }

    /// Delegate voting power to another address
    pub fn delegate_voting_power(
        &mut self,
        delegator_stake: &mut GovernanceStake,
        delegation: &mut VotingDelegation,
        delegate: Pubkey,
        amount: u64,
        duration: Option<i64>,
    ) -> Result<()> {
        require!(!self.is_paused, UniversalNftError::ProgramPaused);
        
        // Check delegator has sufficient voting power
        require!(
            delegator_stake.voting_power >= amount,
            UniversalNftError::ArithmeticOverflow
        );

        let now = Clock::get()?.unix_timestamp;

        // Update delegation
        delegation.delegator = delegator_stake.staker;
        delegation.delegate = delegate;
        delegation.voting_power = amount;
        delegation.delegated_at = now;
        delegation.expires_at = duration.map(|d| now + d);
        delegation.is_active = true;

        // Update delegator's available voting power
        delegator_stake.voting_power = delegator_stake.voting_power.checked_sub(amount)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        delegator_stake.delegated_to = Some(delegate);

        msg!("Voting power delegated: {} to {}", amount, delegate);
        Ok(())
    }

    /// Revoke delegated voting power
    pub fn revoke_delegation(
        &mut self,
        delegator_stake: &mut GovernanceStake,
        delegation: &mut VotingDelegation,
    ) -> Result<()> {
        require!(!self.is_paused, UniversalNftError::ProgramPaused);
        require!(delegation.is_active, UniversalNftError::InvalidTransferStatus);

        // Return voting power to delegator
        delegator_stake.voting_power = delegator_stake.voting_power
            .checked_add(delegation.voting_power)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        delegator_stake.delegated_to = None;

        // Deactivate delegation
        delegation.is_active = false;

        msg!("Delegation revoked: {}", delegation.voting_power);
        Ok(())
    }

    /// Update DAO governance parameters (requires governance vote)
    pub fn update_governance_params(&mut self, config: DAOConfig) -> Result<()> {
        // This should only be called through a successful governance proposal
        self.proposal_threshold = config.proposal_threshold;
        self.min_voting_period = config.min_voting_period;
        self.max_voting_period = config.max_voting_period;
        self.quorum_threshold = config.quorum_threshold;
        self.execution_delay = config.execution_delay;

        msg!("Governance parameters updated");
        Ok(())
    }

    /// Emergency pause (emergency council only)
    pub fn emergency_pause(&mut self, paused: bool) -> Result<()> {
        self.is_paused = paused;
        msg!("DAO emergency pause: {}", paused);
        Ok(())
    }

    /// Calculate voting power multiplier based on lock duration
    fn calculate_power_multiplier(&self, lock_duration: i64) -> u16 {
        match lock_duration {
            0..=604800 => 100,           // 1 week: 1x
            604801..=2592000 => 125,     // 1 month: 1.25x
            2592001..=7776000 => 150,    // 3 months: 1.5x
            7776001..=15552000 => 175,   // 6 months: 1.75x
            15552001..=31104000 => 200,  // 1 year: 2x
            _ => 250,                    // >1 year: 2.5x
        }
    }

    /// Check if user has sufficient voting power for proposal
    pub fn can_create_proposal(&self, voting_power: u64) -> bool {
        voting_power >= self.proposal_threshold
    }

    /// Get current governance statistics
    pub fn get_governance_stats(&self) -> GovernanceStats {
        GovernanceStats {
            total_staked: self.total_staked,
            proposal_count: self.proposal_count,
            active_proposals: self.active_proposals,
            quorum_threshold: self.quorum_threshold,
            participation_rate: if self.total_staked > 0 {
                // Calculate based on recent proposal participation
                75 // Placeholder - would calculate from actual voting history
            } else {
                0
            },
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DAOConfig {
    pub proposal_threshold: u64,
    pub min_voting_period: i64,
    pub max_voting_period: i64,
    pub quorum_threshold: u16,
    pub execution_delay: i64,
}

impl Default for DAOConfig {
    fn default() -> Self {
        Self {
            proposal_threshold: 100_000_000_000, // 100k tokens
            min_voting_period: 172800,            // 2 days
            max_voting_period: 604800,            // 7 days
            quorum_threshold: 400,                // 4% of total supply
            execution_delay: 86400,               // 1 day timelock
        }
    }
}

#[derive(Clone)]
pub struct GovernanceStats {
    pub total_staked: u64,
    pub proposal_count: u64,
    pub active_proposals: u32,
    pub quorum_threshold: u16,
    pub participation_rate: u16,
}

impl GovernanceStake {
    pub const INIT_SPACE: usize = 
        32 + // staker
        8 +  // amount
        8 +  // staked_at
        8 +  // lock_duration
        2 +  // power_multiplier
        1 + 32 + // delegated_to (Option<Pubkey>)
        8 +  // voting_power
        8 +  // rewards_accumulated
        8 +  // last_reward_claim
        1;   // bump

    pub fn initialize(
        &mut self,
        staker: Pubkey,
        amount: u64,
        lock_duration: i64,
        bump: u8,
    ) {
        self.staker = staker;
        self.amount = amount;
        self.staked_at = Clock::get().unwrap().unix_timestamp;
        self.lock_duration = lock_duration;
        self.power_multiplier = 100; // Will be calculated
        self.delegated_to = None;
        self.voting_power = 0;
        self.rewards_accumulated = 0;
        self.last_reward_claim = self.staked_at;
        self.bump = bump;
    }

    /// Check if tokens can be unstaked
    pub fn can_unstake(&self) -> bool {
        let now = Clock::get().unwrap().unix_timestamp;
        now >= self.staked_at + self.lock_duration
    }

    /// Calculate pending rewards
    pub fn calculate_pending_rewards(&self, reward_rate: u64) -> u64 {
        let now = Clock::get().unwrap().unix_timestamp;
        let time_elapsed = now - self.last_reward_claim;
        
        if time_elapsed <= 0 {
            return 0;
        }

        // Simple reward calculation: amount * rate * time / year
        let annual_seconds = 31_536_000; // seconds in a year
        (self.amount * reward_rate * time_elapsed as u64) / (annual_seconds * 10000) // rate is in basis points
    }
}

impl VotingDelegation {
    pub const INIT_SPACE: usize = 
        32 + // delegator
        32 + // delegate
        8 +  // voting_power
        8 +  // delegated_at
        1 + 8 + // expires_at (Option<i64>)
        1 +  // is_active
        1;   // bump

    pub fn initialize(
        &mut self,
        delegator: Pubkey,
        delegate: Pubkey,
        voting_power: u64,
        duration: Option<i64>,
        bump: u8,
    ) {
        let now = Clock::get().unwrap().unix_timestamp;
        
        self.delegator = delegator;
        self.delegate = delegate;
        self.voting_power = voting_power;
        self.delegated_at = now;
        self.expires_at = duration.map(|d| now + d);
        self.is_active = true;
        self.bump = bump;
    }

    /// Check if delegation is still valid
    pub fn is_valid(&self) -> bool {
        if !self.is_active {
            return false;
        }

        if let Some(expiry) = self.expires_at {
            let now = Clock::get().unwrap().unix_timestamp;
            return now <= expiry;
        }

        true
    }
}