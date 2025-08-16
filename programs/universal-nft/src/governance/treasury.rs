use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};
use crate::errors::UniversalNftError;

/// Treasury Management System for Universal NFT Protocol
/// Handles protocol funds, revenue distribution, and treasury operations
#[account]
#[derive(InitSpace)]
pub struct Treasury {
    /// Treasury authority (DAO program)
    pub authority: Pubkey,
    /// Treasury wallet for SOL
    pub sol_vault: Pubkey,
    /// Treasury wallet for governance tokens
    pub governance_vault: Pubkey,
    /// Revenue collected from protocol fees
    pub total_revenue: u64,
    /// Total funds distributed
    pub total_distributed: u64,
    /// Current treasury balance (SOL in lamports)
    pub sol_balance: u64,
    /// Current governance token balance
    pub governance_balance: u64,
    /// Last revenue distribution timestamp
    pub last_distribution: i64,
    /// Distribution frequency (seconds)
    pub distribution_frequency: i64,
    /// Treasury fee percentage (basis points)
    pub treasury_fee_bps: u16,
    /// Emergency reserve percentage
    pub emergency_reserve_bps: u16,
    /// Treasury created timestamp
    pub created_at: i64,
    /// PDA bump
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct TreasuryProposal {
    /// Proposal ID
    pub id: u64,
    /// Proposal type
    pub proposal_type: TreasuryProposalType,
    /// Recipient of funds
    pub recipient: Pubkey,
    /// Amount to transfer (in lamports for SOL)
    pub amount: u64,
    /// Token mint (None for SOL)
    pub token_mint: Option<Pubkey>,
    /// Proposal description
    pub description: String,
    /// Proposer
    pub proposer: Pubkey,
    /// Proposal status
    pub status: TreasuryProposalStatus,
    /// Votes in favor
    pub votes_for: u64,
    /// Votes against
    pub votes_against: u64,
    /// Voting deadline
    pub voting_deadline: i64,
    /// Execution deadline
    pub execution_deadline: i64,
    /// Created timestamp
    pub created_at: i64,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum TreasuryProposalType {
    /// General spending proposal
    Spend,
    /// Investment proposal
    Investment,
    /// Grant proposal
    Grant,
    /// Emergency withdrawal
    Emergency,
    /// Revenue sharing adjustment
    RevenueSharing,
    /// Fee structure change
    FeeChange,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum TreasuryProposalStatus {
    Active,
    Passed,
    Failed,
    Executed,
    Cancelled,
}

#[account]
#[derive(InitSpace)]
pub struct RevenueDistribution {
    /// Distribution ID
    pub id: u64,
    /// Total amount distributed
    pub total_amount: u64,
    /// Number of beneficiaries
    pub beneficiary_count: u32,
    /// Distribution timestamp
    pub distributed_at: i64,
    /// Distribution type
    pub distribution_type: DistributionType,
    /// Amount per token (for proportional distributions)
    pub amount_per_token: u64,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum DistributionType {
    /// Equal distribution to all token holders
    Equal,
    /// Proportional to token holdings
    Proportional,
    /// Proportional to voting power
    VotingPower,
    /// Custom distribution logic
    Custom,
}

impl Treasury {
    pub const INIT_SPACE: usize = 
        32 +    // authority
        32 +    // sol_vault
        32 +    // governance_vault
        8 +     // total_revenue
        8 +     // total_distributed
        8 +     // sol_balance
        8 +     // governance_balance
        8 +     // last_distribution
        8 +     // distribution_frequency
        2 +     // treasury_fee_bps
        2 +     // emergency_reserve_bps
        8 +     // created_at
        1;      // bump

    /// Initialize treasury
    pub fn initialize(
        &mut self,
        authority: Pubkey,
        sol_vault: Pubkey,
        governance_vault: Pubkey,
        config: TreasuryConfig,
        bump: u8,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        self.authority = authority;
        self.sol_vault = sol_vault;
        self.governance_vault = governance_vault;
        self.total_revenue = 0;
        self.total_distributed = 0;
        self.sol_balance = 0;
        self.governance_balance = 0;
        self.last_distribution = now;
        self.distribution_frequency = config.distribution_frequency;
        self.treasury_fee_bps = config.treasury_fee_bps;
        self.emergency_reserve_bps = config.emergency_reserve_bps;
        self.created_at = now;
        self.bump = bump;

        msg!("Treasury initialized with authority: {}", authority);
        msg!("Distribution frequency: {} seconds", self.distribution_frequency);
        msg!("Treasury fee: {}bps", self.treasury_fee_bps);

        Ok(())
    }

    /// Deposit revenue into treasury
    pub fn deposit_revenue(&mut self, amount: u64, is_sol: bool) -> Result<()> {
        self.total_revenue = self.total_revenue.checked_add(amount)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        if is_sol {
            self.sol_balance = self.sol_balance.checked_add(amount)
                .ok_or(UniversalNftError::ArithmeticOverflow)?;
        } else {
            self.governance_balance = self.governance_balance.checked_add(amount)
                .ok_or(UniversalNftError::ArithmeticOverflow)?;
        }

        msg!("Revenue deposited: {} {}", amount, if is_sol { "SOL" } else { "tokens" });
        Ok(())
    }

    /// Calculate available funds for distribution
    pub fn calculate_available_for_distribution(&self, is_sol: bool) -> u64 {
        let balance = if is_sol { self.sol_balance } else { self.governance_balance };
        let emergency_reserve = (balance * self.emergency_reserve_bps as u64) / 10000;
        balance.saturating_sub(emergency_reserve)
    }

    /// Execute treasury spending (after governance approval)
    pub fn execute_spend(
        &mut self,
        amount: u64,
        is_sol: bool,
    ) -> Result<()> {
        let available = self.calculate_available_for_distribution(is_sol);
        require!(amount <= available, UniversalNftError::ArithmeticOverflow);

        if is_sol {
            self.sol_balance = self.sol_balance.checked_sub(amount)
                .ok_or(UniversalNftError::ArithmeticOverflow)?;
        } else {
            self.governance_balance = self.governance_balance.checked_sub(amount)
                .ok_or(UniversalNftError::ArithmeticOverflow)?;
        }

        self.total_distributed = self.total_distributed.checked_add(amount)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        msg!("Treasury spend executed: {} {}", amount, if is_sol { "SOL" } else { "tokens" });
        Ok(())
    }

    /// Check if distribution is due
    pub fn is_distribution_due(&self) -> bool {
        let now = Clock::get().unwrap().unix_timestamp;
        now >= self.last_distribution + self.distribution_frequency
    }

    /// Execute revenue distribution
    pub fn execute_distribution(
        &mut self,
        distribution: &mut RevenueDistribution,
        amount: u64,
        beneficiary_count: u32,
        distribution_type: DistributionType,
    ) -> Result<()> {
        let available = self.calculate_available_for_distribution(true); // SOL for now
        require!(amount <= available, UniversalNftError::ArithmeticOverflow);

        // Update treasury
        self.sol_balance = self.sol_balance.checked_sub(amount)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        self.total_distributed = self.total_distributed.checked_add(amount)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;
        self.last_distribution = Clock::get()?.unix_timestamp;

        // Update distribution record
        distribution.total_amount = amount;
        distribution.beneficiary_count = beneficiary_count;
        distribution.distributed_at = self.last_distribution;
        distribution.distribution_type = distribution_type;
        distribution.amount_per_token = if beneficiary_count > 0 {
            amount / beneficiary_count as u64
        } else {
            0
        };

        msg!("Revenue distribution executed: {} SOL to {} beneficiaries", 
             amount, beneficiary_count);
        Ok(())
    }

    /// Update treasury configuration
    pub fn update_config(&mut self, config: TreasuryConfig) -> Result<()> {
        self.distribution_frequency = config.distribution_frequency;
        self.treasury_fee_bps = config.treasury_fee_bps;
        self.emergency_reserve_bps = config.emergency_reserve_bps;

        msg!("Treasury configuration updated");
        Ok(())
    }

    /// Get treasury statistics
    pub fn get_treasury_stats(&self) -> TreasuryStats {
        let total_balance = self.sol_balance + self.governance_balance;
        let utilization_rate = if self.total_revenue > 0 {
            (self.total_distributed * 100) / self.total_revenue
        } else {
            0
        };

        TreasuryStats {
            total_revenue: self.total_revenue,
            total_distributed: self.total_distributed,
            current_balance: total_balance,
            sol_balance: self.sol_balance,
            governance_balance: self.governance_balance,
            utilization_rate,
            emergency_reserve: (total_balance * self.emergency_reserve_bps as u64) / 10000,
            days_since_last_distribution: {
                let now = Clock::get().unwrap().unix_timestamp;
                (now - self.last_distribution) / 86400
            },
        }
    }
}

impl TreasuryProposal {
    pub const INIT_SPACE: usize = 
        8 +     // id
        1 +     // proposal_type (enum)
        32 +    // recipient
        8 +     // amount
        1 + 32 + // token_mint (Option<Pubkey>)
        4 + 256 + // description (String)
        32 +    // proposer
        1 +     // status (enum)
        8 +     // votes_for
        8 +     // votes_against
        8 +     // voting_deadline
        8 +     // execution_deadline
        8 +     // created_at
        1;      // bump

    pub fn initialize(
        &mut self,
        id: u64,
        proposal_type: TreasuryProposalType,
        recipient: Pubkey,
        amount: u64,
        token_mint: Option<Pubkey>,
        description: String,
        proposer: Pubkey,
        bump: u8,
    ) -> Result<()> {
        require!(description.len() <= 256, UniversalNftError::InvalidTransferStatus);

        let now = Clock::get()?.unix_timestamp;
        
        self.id = id;
        self.proposal_type = proposal_type;
        self.recipient = recipient;
        self.amount = amount;
        self.token_mint = token_mint;
        self.description = description;
        self.proposer = proposer;
        self.status = TreasuryProposalStatus::Active;
        self.votes_for = 0;
        self.votes_against = 0;
        self.voting_deadline = now + (7 * 24 * 3600); // 7 days
        self.execution_deadline = self.voting_deadline + (3 * 24 * 3600); // +3 days
        self.created_at = now;
        self.bump = bump;

        msg!("Treasury proposal {} created: {} {} to {}", 
             id, amount, 
             if token_mint.is_some() { "tokens" } else { "SOL" },
             recipient);
        Ok(())
    }

    pub fn cast_vote(&mut self, vote_for: bool, voting_power: u64) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        require!(now <= self.voting_deadline, UniversalNftError::InvalidTransferStatus);
        require!(self.status == TreasuryProposalStatus::Active, UniversalNftError::InvalidTransferStatus);

        if vote_for {
            self.votes_for = self.votes_for.checked_add(voting_power)
                .ok_or(UniversalNftError::ArithmeticOverflow)?;
        } else {
            self.votes_against = self.votes_against.checked_add(voting_power)
                .ok_or(UniversalNftError::ArithmeticOverflow)?;
        }

        Ok(())
    }

    pub fn finalize(&mut self, quorum_threshold: u64) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        require!(now > self.voting_deadline, UniversalNftError::InvalidTransferStatus);
        require!(self.status == TreasuryProposalStatus::Active, UniversalNftError::InvalidTransferStatus);

        let total_votes = self.votes_for + self.votes_against;
        
        if total_votes >= quorum_threshold && self.votes_for > self.votes_against {
            self.status = TreasuryProposalStatus::Passed;
        } else {
            self.status = TreasuryProposalStatus::Failed;
        }

        Ok(())
    }

    pub fn execute(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        require!(self.status == TreasuryProposalStatus::Passed, UniversalNftError::InvalidTransferStatus);
        require!(now <= self.execution_deadline, UniversalNftError::InvalidTransferStatus);

        self.status = TreasuryProposalStatus::Executed;
        Ok(())
    }
}

impl RevenueDistribution {
    pub const INIT_SPACE: usize = 
        8 +     // id
        8 +     // total_amount
        4 +     // beneficiary_count
        8 +     // distributed_at
        1 +     // distribution_type (enum)
        8 +     // amount_per_token
        1;      // bump

    pub fn initialize(&mut self, id: u64, bump: u8) {
        self.id = id;
        self.total_amount = 0;
        self.beneficiary_count = 0;
        self.distributed_at = 0;
        self.distribution_type = DistributionType::Proportional;
        self.amount_per_token = 0;
        self.bump = bump;
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TreasuryConfig {
    pub distribution_frequency: i64,
    pub treasury_fee_bps: u16,
    pub emergency_reserve_bps: u16,
}

impl Default for TreasuryConfig {
    fn default() -> Self {
        Self {
            distribution_frequency: 30 * 24 * 3600, // 30 days
            treasury_fee_bps: 500,                   // 5%
            emergency_reserve_bps: 1000,             // 10%
        }
    }
}

#[derive(Clone)]
pub struct TreasuryStats {
    pub total_revenue: u64,
    pub total_distributed: u64,
    pub current_balance: u64,
    pub sol_balance: u64,
    pub governance_balance: u64,
    pub utilization_rate: u64,
    pub emergency_reserve: u64,
    pub days_since_last_distribution: i64,
}