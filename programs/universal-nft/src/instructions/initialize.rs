use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

pub fn initialize(ctx: Context<Initialize>, gateway_authority: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    // Validate gateway authority
    require!(
        gateway_authority != Pubkey::default(),
        UniversalNftError::InvalidGatewayAuthority
    );

    // Initialize program configuration
    config.authority = ctx.accounts.authority.key();
    config.gateway_authority = gateway_authority;
    config.tss_authority = Pubkey::default(); // Will be set later via update
    config.nonce = 0;
    config.bump = ctx.bumps.config;
    config.is_paused = false;

    msg!("Universal NFT program initialized");
    msg!("Authority: {}", config.authority);
    msg!("Gateway Authority: {}", config.gateway_authority);

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + ProgramConfig::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, ProgramConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

/// Update program configuration (authority only)
pub fn update_config(
    ctx: Context<UpdateConfig>,
    new_gateway_authority: Option<Pubkey>,
    new_tss_authority: Option<Pubkey>,
    paused: Option<bool>,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    // Only authority can update configuration
    require!(
        ctx.accounts.authority.key() == config.authority,
        UniversalNftError::Unauthorized
    );

    if let Some(gateway_auth) = new_gateway_authority {
        require!(
            gateway_auth != Pubkey::default(),
            UniversalNftError::InvalidGatewayAuthority
        );
        config.gateway_authority = gateway_auth;
        msg!("Gateway authority updated to: {}", gateway_auth);
    }

    if let Some(tss_auth) = new_tss_authority {
        config.tss_authority = tss_auth;
        msg!("TSS authority updated to: {}", tss_auth);
    }

    if let Some(is_paused) = paused {
        config.is_paused = is_paused;
        msg!("Program paused status updated to: {}", is_paused);
    }

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}