//! state/protocol_config.rs
//!
//! Global protocol configuration.
//!
//! This account stores protocol-wide configuration that changes
//! infrequently. Separating configuration from mutable protocol
//! state reduces unnecessary account writes and follows the design
//! used by many production DeFi protocols.
//!
//! PDA Seed:
//!     PROTOCOL_CONFIG_SEED

use anchor_lang::prelude::*;

/// Global protocol configuration.
///
/// PDA:
/// seeds = [PROTOCOL_CONFIG_SEED]
#[account]
#[derive(InitSpace)]
pub struct ProtocolConfig {
    /// Protocol administrator.
    ///
    /// Can:
    /// - Pause/unpause the protocol
    /// - Update protocol parameters
    /// - Transfer protocol authority
    pub authority: Pubkey,

    /// Minimum collateral ratio expressed in basis points.
    ///
    /// Example:
    /// 15000 = 150%
    pub min_collateral_ratio_bps: u16,

    /// Liquidation threshold.
    ///
    /// If a vault falls below this threshold,
    /// it becomes eligible for liquidation.
    ///
    /// Example:
    /// 12000 = 120%
    pub liquidation_threshold_bps: u16,

    /// Liquidation bonus awarded to liquidators.
    ///
    /// Example:
    /// 500 = 5%
    pub liquidation_bonus_bps: u16,

    /// Annual stability fee in basis points.
    ///
    /// Phase 1:
    /// Always initialized to zero.
    ///
    /// Phase 2:
    /// Used to accrue protocol revenue.
    pub stability_fees_bps: u16,

    pub bump: u8,
}

impl ProtocolConfig {
    /// Total account space.
    ///
    /// Anchor automatically calculates the struct size using
    /// `InitSpace`; we only add the 8-byte discriminator.
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}