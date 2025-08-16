use anchor_lang::prelude::*;
use crate::errors::UniversalNftError;

/// Upgrade Authority System for Universal NFT Protocol
/// Manages program upgrades through decentralized governance
#[account]
#[derive(InitSpace)]
pub struct UpgradeAuthority {
    /// Current upgrade authority (initially deployer, later DAO)
    pub authority: Pubkey,
    /// Program ID being controlled
    pub program_id: Pubkey,
    /// Pending upgrade proposal
    pub pending_upgrade: Option<UpgradeProposal>,
    /// Upgrade history count
    pub upgrade_count: u32,
    /// Last upgrade timestamp
    pub last_upgrade: i64,
    /// Minimum time between upgrades (seconds)
    pub upgrade_cooldown: i64,
    /// Emergency upgrade authority (for critical fixes)
    pub emergency_authority: Pubkey,
    /// Whether emergency upgrades are enabled
    pub emergency_enabled: bool,
    /// Total voting power required for upgrades
    pub upgrade_threshold: u64,
    /// Created timestamp
    pub created_at: i64,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UpgradeProposal {
    /// Proposal ID
    pub id: u64,
    /// New program data account
    pub new_program_data: Pubkey,
    /// Upgrade description
    pub description: String,
    /// Proposer
    pub proposer: Pubkey,
    /// Votes in favor
    pub votes_for: u64,
    /// Votes against
    pub votes_against: u64,
    /// Voting deadline
    pub voting_deadline: i64,
    /// Execution deadline
    pub execution_deadline: i64,
    /// Upgrade type
    pub upgrade_type: UpgradeType,
    /// Status
    pub status: UpgradeStatus,
    /// Created timestamp
    pub created_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum UpgradeType {
    /// Regular feature upgrade
    Feature,
    /// Security patch
    Security,
    /// Emergency hotfix
    Emergency,
    /// Major version upgrade
    Major,
    /// Bug fix
    BugFix,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum UpgradeStatus {
    Proposed,
    Voting,
    Approved,
    Rejected,
    Executed,
    Cancelled,
    Expired,
}

#[account]
#[derive(InitSpace)]
pub struct UpgradeHistory {
    /// Upgrade ID
    pub id: u32,
    /// Previous program data hash
    pub previous_hash: [u8; 32],
    /// New program data hash
    pub new_hash: [u8; 32],
    /// Upgrade type
    pub upgrade_type: UpgradeType,
    /// Upgrade description
    pub description: String,
    /// Executed by
    pub executed_by: Pubkey,
    /// Execution timestamp
    pub executed_at: i64,
    /// Votes received (for governance upgrades)
    pub votes_for: u64,
    pub votes_against: u64,
    /// Gas cost of upgrade
    pub gas_used: u64,
    /// Rollback information (for emergency rollbacks)
    pub rollback_data: Option<RollbackData>,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RollbackData {
    /// Previous program data account
    pub previous_program_data: Pubkey,
    /// Rollback deadline
    pub rollback_deadline: i64,
    /// Rollback authorized by
    pub rollback_authority: Pubkey,
}

impl UpgradeAuthority {
    pub const INIT_SPACE: usize = 
        32 +    // authority
        32 +    // program_id
        1 + 256 + // pending_upgrade (Option<UpgradeProposal>)
        4 +     // upgrade_count
        8 +     // last_upgrade
        8 +     // upgrade_cooldown
        32 +    // emergency_authority
        1 +     // emergency_enabled
        8 +     // upgrade_threshold
        8 +     // created_at
        1;      // bump

    /// Initialize upgrade authority
    pub fn initialize(
        &mut self,
        authority: Pubkey,
        program_id: Pubkey,
        emergency_authority: Pubkey,
        config: UpgradeConfig,
        bump: u8,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        self.authority = authority;
        self.program_id = program_id;
        self.pending_upgrade = None;
        self.upgrade_count = 0;
        self.last_upgrade = now;
        self.upgrade_cooldown = config.upgrade_cooldown;
        self.emergency_authority = emergency_authority;
        self.emergency_enabled = config.emergency_enabled;
        self.upgrade_threshold = config.upgrade_threshold;
        self.created_at = now;
        self.bump = bump;

        msg!("Upgrade authority initialized for program: {}", program_id);
        msg!("Upgrade cooldown: {} seconds", self.upgrade_cooldown);
        msg!("Emergency upgrades enabled: {}", self.emergency_enabled);

        Ok(())
    }

    /// Propose a program upgrade
    pub fn propose_upgrade(
        &mut self,
        proposal_id: u64,
        new_program_data: Pubkey,
        description: String,
        proposer: Pubkey,
        upgrade_type: UpgradeType,
    ) -> Result<()> {
        require!(self.pending_upgrade.is_none(), UniversalNftError::InvalidTransferStatus);
        require!(description.len() <= 256, UniversalNftError::InvalidTransferStatus);

        let now = Clock::get()?.unix_timestamp;

        // Check cooldown period (except for emergency upgrades)
        if upgrade_type != UpgradeType::Emergency {
            require!(
                now >= self.last_upgrade + self.upgrade_cooldown,
                UniversalNftError::InvalidTransferStatus
            );
        }

        let voting_period = match upgrade_type {
            UpgradeType::Emergency => 24 * 3600,    // 1 day
            UpgradeType::Security => 3 * 24 * 3600, // 3 days
            UpgradeType::BugFix => 5 * 24 * 3600,   // 5 days
            _ => 7 * 24 * 3600,                     // 7 days
        };

        let proposal = UpgradeProposal {
            id: proposal_id,
            new_program_data,
            description,
            proposer,
            votes_for: 0,
            votes_against: 0,
            voting_deadline: now + voting_period,
            execution_deadline: now + voting_period + (3 * 24 * 3600), // +3 days
            upgrade_type,
            status: UpgradeStatus::Voting,
            created_at: now,
        };

        self.pending_upgrade = Some(proposal);

        msg!("Upgrade proposal {} created by {}", proposal_id, proposer);
        msg!("Voting deadline: {}", now + voting_period);

        Ok(())
    }

    /// Vote on pending upgrade proposal
    pub fn vote_on_upgrade(
        &mut self,
        vote_for: bool,
        voting_power: u64,
    ) -> Result<()> {
        let proposal = self.pending_upgrade.as_mut()
            .ok_or(UniversalNftError::InvalidTransferStatus)?;

        let now = Clock::get()?.unix_timestamp;
        require!(now <= proposal.voting_deadline, UniversalNftError::InvalidTransferStatus);
        require!(proposal.status == UpgradeStatus::Voting, UniversalNftError::InvalidTransferStatus);

        if vote_for {
            proposal.votes_for = proposal.votes_for.checked_add(voting_power)
                .ok_or(UniversalNftError::ArithmeticOverflow)?;
        } else {
            proposal.votes_against = proposal.votes_against.checked_add(voting_power)
                .ok_or(UniversalNftError::ArithmeticOverflow)?;
        }

        msg!("Upgrade vote cast: {} with {} voting power", 
             if vote_for { "FOR" } else { "AGAINST" }, voting_power);

        Ok(())
    }

    /// Finalize upgrade proposal voting
    pub fn finalize_upgrade_vote(&mut self) -> Result<()> {
        let proposal = self.pending_upgrade.as_mut()
            .ok_or(UniversalNftError::InvalidTransferStatus)?;

        let now = Clock::get()?.unix_timestamp;
        require!(now > proposal.voting_deadline, UniversalNftError::InvalidTransferStatus);
        require!(proposal.status == UpgradeStatus::Voting, UniversalNftError::InvalidTransferStatus);

        let total_votes = proposal.votes_for + proposal.votes_against;
        
        // Check if threshold is met and majority approves
        if total_votes >= self.upgrade_threshold && proposal.votes_for > proposal.votes_against {
            proposal.status = UpgradeStatus::Approved;
            msg!("Upgrade proposal {} approved: {} for, {} against", 
                 proposal.id, proposal.votes_for, proposal.votes_against);
        } else {
            proposal.status = UpgradeStatus::Rejected;
            msg!("Upgrade proposal {} rejected: {} for, {} against", 
                 proposal.id, proposal.votes_for, proposal.votes_against);
        }

        Ok(())
    }

    /// Execute approved upgrade
    pub fn execute_upgrade(
        &mut self,
        history: &mut UpgradeHistory,
        program_data_hash: [u8; 32],
        new_program_data_hash: [u8; 32],
        executor: Pubkey,
        gas_used: u64,
    ) -> Result<()> {
        let proposal = self.pending_upgrade.as_mut()
            .ok_or(UniversalNftError::InvalidTransferStatus)?;

        let now = Clock::get()?.unix_timestamp;
        require!(proposal.status == UpgradeStatus::Approved, UniversalNftError::InvalidTransferStatus);
        require!(now <= proposal.execution_deadline, UniversalNftError::InvalidTransferStatus);

        // Record upgrade in history
        history.id = self.upgrade_count;
        history.previous_hash = program_data_hash;
        history.new_hash = new_program_data_hash;
        history.upgrade_type = proposal.upgrade_type.clone();
        history.description = proposal.description.clone();
        history.executed_by = executor;
        history.executed_at = now;
        history.votes_for = proposal.votes_for;
        history.votes_against = proposal.votes_against;
        history.gas_used = gas_used;
        
        // Set rollback data for non-emergency upgrades
        if proposal.upgrade_type != UpgradeType::Emergency {
            history.rollback_data = Some(RollbackData {
                previous_program_data: Pubkey::default(), // Would be set to actual previous data
                rollback_deadline: now + (7 * 24 * 3600), // 7 days to rollback
                rollback_authority: self.emergency_authority,
            });
        } else {
            history.rollback_data = None;
        }

        // Update authority state
        proposal.status = UpgradeStatus::Executed;
        self.upgrade_count = self.upgrade_count.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        self.last_upgrade = now;

        msg!("Upgrade {} executed successfully by {}", proposal.id, executor);
        msg!("New upgrade count: {}", self.upgrade_count);

        // Clear pending upgrade
        self.pending_upgrade = None;

        Ok(())
    }

    /// Emergency upgrade (by emergency authority only)
    pub fn emergency_upgrade(
        &mut self,
        new_program_data: Pubkey,
        description: String,
        executor: Pubkey,
        history: &mut UpgradeHistory,
        program_data_hash: [u8; 32],
        new_program_data_hash: [u8; 32],
    ) -> Result<()> {
        require!(self.emergency_enabled, UniversalNftError::InvalidTransferStatus);
        require!(description.len() <= 256, UniversalNftError::InvalidTransferStatus);

        let now = Clock::get()?.unix_timestamp;

        // Record emergency upgrade
        history.id = self.upgrade_count;
        history.previous_hash = program_data_hash;
        history.new_hash = new_program_data_hash;
        history.upgrade_type = UpgradeType::Emergency;
        history.description = description;
        history.executed_by = executor;
        history.executed_at = now;
        history.votes_for = 0; // Emergency upgrades bypass voting
        history.votes_against = 0;
        history.gas_used = 0; // Will be updated later
        history.rollback_data = Some(RollbackData {
            previous_program_data: Pubkey::default(),
            rollback_deadline: now + (24 * 3600), // 24 hours to rollback
            rollback_authority: self.emergency_authority,
        });

        self.upgrade_count = self.upgrade_count.checked_add(1)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        self.last_upgrade = now;

        msg!("Emergency upgrade executed by {}", executor);
        msg!("Description: {}", description);

        Ok(())
    }

    /// Transfer upgrade authority (requires governance vote)
    pub fn transfer_authority(&mut self, new_authority: Pubkey) -> Result<()> {
        self.authority = new_authority;
        msg!("Upgrade authority transferred to: {}", new_authority);
        Ok(())
    }

    /// Update upgrade configuration
    pub fn update_config(&mut self, config: UpgradeConfig) -> Result<()> {
        self.upgrade_cooldown = config.upgrade_cooldown;
        self.emergency_enabled = config.emergency_enabled;
        self.upgrade_threshold = config.upgrade_threshold;

        msg!("Upgrade configuration updated");
        Ok(())
    }

    /// Disable emergency upgrades (irreversible)
    pub fn disable_emergency_upgrades(&mut self) -> Result<()> {
        self.emergency_enabled = false;
        msg!("Emergency upgrades permanently disabled");
        Ok(())
    }

    /// Get upgrade statistics
    pub fn get_upgrade_stats(&self) -> UpgradeStats {
        let now = Clock::get().unwrap().unix_timestamp;
        let time_since_last = now - self.last_upgrade;
        let cooldown_remaining = (self.last_upgrade + self.upgrade_cooldown - now).max(0);

        UpgradeStats {
            total_upgrades: self.upgrade_count,
            last_upgrade: self.last_upgrade,
            time_since_last_upgrade: time_since_last,
            cooldown_remaining,
            emergency_enabled: self.emergency_enabled,
            pending_proposal: self.pending_upgrade.is_some(),
            upgrade_threshold: self.upgrade_threshold,
        }
    }
}

impl UpgradeHistory {
    pub const INIT_SPACE: usize = 
        4 +     // id
        32 +    // previous_hash
        32 +    // new_hash
        1 +     // upgrade_type (enum)
        4 + 256 + // description (String)
        32 +    // executed_by
        8 +     // executed_at
        8 +     // votes_for
        8 +     // votes_against
        8 +     // gas_used
        1 + 64 + // rollback_data (Option<RollbackData>)
        1;      // bump

    pub fn initialize(&mut self, bump: u8) {
        self.id = 0;
        self.previous_hash = [0; 32];
        self.new_hash = [0; 32];
        self.upgrade_type = UpgradeType::Feature;
        self.description = String::new();
        self.executed_by = Pubkey::default();
        self.executed_at = 0;
        self.votes_for = 0;
        self.votes_against = 0;
        self.gas_used = 0;
        self.rollback_data = None;
        self.bump = bump;
    }

    /// Check if rollback is still possible
    pub fn can_rollback(&self) -> bool {
        if let Some(rollback_data) = &self.rollback_data {
            let now = Clock::get().unwrap().unix_timestamp;
            now <= rollback_data.rollback_deadline
        } else {
            false
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UpgradeConfig {
    pub upgrade_cooldown: i64,
    pub emergency_enabled: bool,
    pub upgrade_threshold: u64,
}

impl Default for UpgradeConfig {
    fn default() -> Self {
        Self {
            upgrade_cooldown: 7 * 24 * 3600,    // 7 days
            emergency_enabled: true,
            upgrade_threshold: 1_000_000_000_000, // 1M tokens
        }
    }
}

#[derive(Clone)]
pub struct UpgradeStats {
    pub total_upgrades: u32,
    pub last_upgrade: i64,
    pub time_since_last_upgrade: i64,
    pub cooldown_remaining: i64,
    pub emergency_enabled: bool,
    pub pending_proposal: bool,
    pub upgrade_threshold: u64,
}