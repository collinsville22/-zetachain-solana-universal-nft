use anchor_lang::prelude::*;
use crate::errors::UniversalNftError;
use crate::governance::dao::{UniversalNftDAO, GovernanceStake};

/// Governance Proposal System for Universal NFT Protocol
/// Enables token holders to propose and vote on protocol changes
#[account]
#[derive(InitSpace)]
pub struct Proposal {
    /// Unique proposal ID
    pub id: u64,
    /// Proposer's public key
    pub proposer: Pubkey,
    /// Proposal title (max 128 chars)
    pub title: String,
    /// Proposal description (max 512 chars)
    pub description: String,
    /// Proposal type
    pub proposal_type: ProposalType,
    /// Target for execution (program ID, account, etc.)
    pub target: Option<Pubkey>,
    /// Encoded instruction data for execution
    pub instruction_data: Vec<u8>,
    /// Voting starts at this timestamp
    pub voting_start: i64,
    /// Voting ends at this timestamp
    pub voting_end: i64,
    /// Execution deadline (after voting passes)
    pub execution_deadline: i64,
    /// Current proposal status
    pub status: ProposalStatus,
    /// Total votes cast
    pub total_votes: u64,
    /// Votes in favor
    pub votes_for: u64,
    /// Votes against
    pub votes_against: u64,
    /// Votes abstained
    pub votes_abstain: u64,
    /// Minimum voting power required (quorum)
    pub quorum_threshold: u64,
    /// Proposal created timestamp
    pub created_at: i64,
    /// Proposal executed timestamp
    pub executed_at: Option<i64>,
    /// Emergency proposal flag (shorter voting period)
    pub is_emergency: bool,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ProposalType {
    /// Change governance parameters
    GovernanceUpdate,
    /// Upgrade program authority
    ProgramUpgrade,
    /// Treasury management
    TreasurySpend,
    /// Emergency pause/unpause
    EmergencyAction,
    /// Protocol parameter changes
    ProtocolUpdate,
    /// Integration with new chains
    ChainIntegration,
    /// Security configuration updates
    SecurityUpdate,
    /// Fee structure changes
    FeeUpdate,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ProposalStatus {
    /// Proposal is active and can be voted on
    Active,
    /// Proposal passed and can be executed
    Passed,
    /// Proposal failed (didn't meet quorum or majority)
    Failed,
    /// Proposal has been executed
    Executed,
    /// Proposal was cancelled
    Cancelled,
    /// Proposal expired without execution
    Expired,
}

#[account]
#[derive(InitSpace)]
pub struct Vote {
    /// Voter's public key
    pub voter: Pubkey,
    /// Proposal being voted on
    pub proposal: Pubkey,
    /// Vote choice
    pub vote_type: VoteType,
    /// Voting power used
    pub voting_power: u64,
    /// Vote timestamp
    pub voted_at: i64,
    /// Delegation source (if vote was delegated)
    pub delegation_source: Option<Pubkey>,
    /// PDA bump
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum VoteType {
    For,
    Against,
    Abstain,
}

impl Proposal {
    pub const INIT_SPACE: usize = 
        8 +     // id
        32 +    // proposer
        4 + 128 + // title (String)
        4 + 512 + // description (String)
        1 +     // proposal_type (enum)
        1 + 32 + // target (Option<Pubkey>)
        4 + 1024 + // instruction_data (Vec<u8>)
        8 +     // voting_start
        8 +     // voting_end
        8 +     // execution_deadline
        1 +     // status (enum)
        8 +     // total_votes
        8 +     // votes_for
        8 +     // votes_against
        8 +     // votes_abstain
        8 +     // quorum_threshold
        8 +     // created_at
        1 + 8 + // executed_at (Option<i64>)
        1 +     // is_emergency
        1;      // bump

    /// Initialize a new proposal
    pub fn initialize(
        &mut self,
        id: u64,
        proposer: Pubkey,
        title: String,
        description: String,
        proposal_type: ProposalType,
        target: Option<Pubkey>,
        instruction_data: Vec<u8>,
        dao: &UniversalNftDAO,
        is_emergency: bool,
        bump: u8,
    ) -> Result<()> {
        require!(title.len() <= 128, UniversalNftError::InvalidTransferStatus);
        require!(description.len() <= 512, UniversalNftError::InvalidTransferStatus);
        require!(instruction_data.len() <= 1024, UniversalNftError::InvalidTransferStatus);

        let now = Clock::get()?.unix_timestamp;
        
        // Determine voting period based on emergency status
        let voting_duration = if is_emergency {
            dao.min_voting_period
        } else {
            (dao.min_voting_period + dao.max_voting_period) / 2 // Default to middle
        };

        self.id = id;
        self.proposer = proposer;
        self.title = title;
        self.description = description;
        self.proposal_type = proposal_type;
        self.target = target;
        self.instruction_data = instruction_data;
        self.voting_start = now;
        self.voting_end = now + voting_duration;
        self.execution_deadline = self.voting_end + dao.execution_delay;
        self.status = ProposalStatus::Active;
        self.total_votes = 0;
        self.votes_for = 0;
        self.votes_against = 0;
        self.votes_abstain = 0;
        self.quorum_threshold = (dao.total_staked * dao.quorum_threshold as u64) / 10000;
        self.created_at = now;
        self.executed_at = None;
        self.is_emergency = is_emergency;
        self.bump = bump;

        msg!("Proposal {} created: {}", id, self.title);
        msg!("Voting period: {} to {}", self.voting_start, self.voting_end);
        msg!("Quorum required: {}", self.quorum_threshold);

        Ok(())
    }

    /// Cast a vote on this proposal
    pub fn cast_vote(
        &mut self,
        vote: &mut Vote,
        voter: Pubkey,
        vote_type: VoteType,
        voting_power: u64,
        delegation_source: Option<Pubkey>,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        // Validate voting period
        require!(
            now >= self.voting_start && now <= self.voting_end,
            UniversalNftError::InvalidTransferStatus
        );

        // Validate proposal is active
        require!(
            self.status == ProposalStatus::Active,
            UniversalNftError::InvalidTransferStatus
        );

        // Initialize vote account
        vote.voter = voter;
        vote.proposal = Pubkey::default(); // Will be set by caller
        vote.vote_type = vote_type.clone();
        vote.voting_power = voting_power;
        vote.voted_at = now;
        vote.delegation_source = delegation_source;

        // Update proposal vote counts
        self.total_votes = self.total_votes.checked_add(voting_power)
            .ok_or(UniversalNftError::ArithmeticOverflow)?;

        match vote_type {
            VoteType::For => {
                self.votes_for = self.votes_for.checked_add(voting_power)
                    .ok_or(UniversalNftError::ArithmeticOverflow)?;
            }
            VoteType::Against => {
                self.votes_against = self.votes_against.checked_add(voting_power)
                    .ok_or(UniversalNftError::ArithmeticOverflow)?;
            }
            VoteType::Abstain => {
                self.votes_abstain = self.votes_abstain.checked_add(voting_power)
                    .ok_or(UniversalNftError::ArithmeticOverflow)?;
            }
        }

        msg!("Vote cast: {} with {} voting power", 
             match vote_type { VoteType::For => "FOR", VoteType::Against => "AGAINST", VoteType::Abstain => "ABSTAIN" },
             voting_power);

        Ok(())
    }

    /// Finalize proposal after voting period ends
    pub fn finalize(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        // Can only finalize after voting period
        require!(now > self.voting_end, UniversalNftError::InvalidTransferStatus);
        require!(self.status == ProposalStatus::Active, UniversalNftError::InvalidTransferStatus);

        // Check if quorum was met
        if self.total_votes < self.quorum_threshold {
            self.status = ProposalStatus::Failed;
            msg!("Proposal {} failed: insufficient quorum ({} < {})", 
                 self.id, self.total_votes, self.quorum_threshold);
            return Ok(());
        }

        // Check if majority voted in favor
        if self.votes_for > self.votes_against {
            self.status = ProposalStatus::Passed;
            msg!("Proposal {} passed: {} for, {} against", 
                 self.id, self.votes_for, self.votes_against);
        } else {
            self.status = ProposalStatus::Failed;
            msg!("Proposal {} failed: {} for, {} against", 
                 self.id, self.votes_for, self.votes_against);
        }

        Ok(())
    }

    /// Execute a passed proposal
    pub fn execute(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        // Validate proposal can be executed
        require!(self.status == ProposalStatus::Passed, UniversalNftError::InvalidTransferStatus);
        require!(now <= self.execution_deadline, UniversalNftError::InvalidTransferStatus);

        self.status = ProposalStatus::Executed;
        self.executed_at = Some(now);

        msg!("Proposal {} executed successfully", self.id);
        Ok(())
    }

    /// Cancel a proposal (only by proposer or emergency council)
    pub fn cancel(&mut self) -> Result<()> {
        require!(
            self.status == ProposalStatus::Active || self.status == ProposalStatus::Passed,
            UniversalNftError::InvalidTransferStatus
        );

        self.status = ProposalStatus::Cancelled;
        msg!("Proposal {} cancelled", self.id);
        Ok(())
    }

    /// Check if proposal has expired
    pub fn check_expiry(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        if self.status == ProposalStatus::Passed && now > self.execution_deadline {
            self.status = ProposalStatus::Expired;
            msg!("Proposal {} expired", self.id);
        }

        Ok(())
    }

    /// Get voting statistics
    pub fn get_voting_stats(&self) -> VotingStats {
        let total_possible = self.quorum_threshold; // Conservative estimate
        let participation_rate = if total_possible > 0 {
            (self.total_votes * 100) / total_possible
        } else {
            0
        };

        VotingStats {
            total_votes: self.total_votes,
            votes_for: self.votes_for,
            votes_against: self.votes_against,
            votes_abstain: self.votes_abstain,
            participation_rate: participation_rate.min(100),
            for_percentage: if self.total_votes > 0 {
                (self.votes_for * 100) / self.total_votes
            } else {
                0
            },
            against_percentage: if self.total_votes > 0 {
                (self.votes_against * 100) / self.total_votes
            } else {
                0
            },
            quorum_met: self.total_votes >= self.quorum_threshold,
        }
    }

    /// Check if user can vote (not already voted)
    pub fn can_vote(&self, voter: Pubkey) -> bool {
        let now = Clock::get().unwrap().unix_timestamp;
        now >= self.voting_start && 
        now <= self.voting_end && 
        self.status == ProposalStatus::Active
    }

    /// Get time remaining for voting
    pub fn time_remaining(&self) -> i64 {
        let now = Clock::get().unwrap().unix_timestamp;
        (self.voting_end - now).max(0)
    }
}

impl Vote {
    pub const INIT_SPACE: usize = 
        32 +    // voter
        32 +    // proposal
        1 +     // vote_type (enum)
        8 +     // voting_power
        8 +     // voted_at
        1 + 32 + // delegation_source (Option<Pubkey>)
        1;      // bump

    pub fn initialize(
        &mut self,
        voter: Pubkey,
        proposal: Pubkey,
        vote_type: VoteType,
        voting_power: u64,
        delegation_source: Option<Pubkey>,
        bump: u8,
    ) {
        self.voter = voter;
        self.proposal = proposal;
        self.vote_type = vote_type;
        self.voting_power = voting_power;
        self.voted_at = Clock::get().unwrap().unix_timestamp;
        self.delegation_source = delegation_source;
        self.bump = bump;
    }
}

#[derive(Clone)]
pub struct VotingStats {
    pub total_votes: u64,
    pub votes_for: u64,
    pub votes_against: u64,
    pub votes_abstain: u64,
    pub participation_rate: u64,
    pub for_percentage: u64,
    pub against_percentage: u64,
    pub quorum_met: bool,
}

/// Proposal creation parameters
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateProposalParams {
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub target: Option<Pubkey>,
    pub instruction_data: Vec<u8>,
    pub is_emergency: bool,
}

impl Default for CreateProposalParams {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            proposal_type: ProposalType::ProtocolUpdate,
            target: None,
            instruction_data: Vec::new(),
            is_emergency: false,
        }
    }
}