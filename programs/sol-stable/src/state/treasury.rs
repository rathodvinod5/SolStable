//! state/treasury.rs
//!
//! Protocol treasury metadata.
//!
//! IMPORTANT:
//! This account DOES NOT store SOL.
//!
//! It only stores accounting information about protocol-owned funds.
//! The actual SOL is stored inside a separate Treasury Vault PDA.
//!
//! This separation keeps accounting independent from custody and
//! mirrors the architecture used in many production DeFi protocols.

use anchor_lang::prelude::*;


/// Treasury metadata.
///
/// PDA:
/// seeds = [TREASURY_SEED]
#[account]
#[derive(InitSpace)]
pub struct Treasury {
     /// Total protocol fees collected.
    ///
    /// Phase 1:
    /// Always zero.
    ///
    /// Phase 2:
    /// Increased by stability fees.
    pub accumelated_fees: u64,

    /// Total rewards paid to liquidators.
    ///
    /// Phase 1:
    /// Always zero.
    ///
    /// Phase 2:
    /// Updated whenever a liquidation occurs.
    pub liquidation_rewards: u64,

    pub bump: u8
}

impl Treasury {
    /// Account size including Anchor discriminator.
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    pub fn add_fees(&mut self, amount: u64) -> Result<()> {
        self.accumelated_fees = self.accumelated_fees
            .checked_add(amount)
            .ok_or(crate::errors::SolStableErrors::MathOverflow)?;

        Ok(())
    }

    pub fn add_liquidation_rewards(&mut self, amount: u64) -> Result<()> {
        self.liquidation_rewards = self.liquidation_rewards
            .checked_add(amount)
            .ok_or(crate::errors::SolStableErrors::MathOverflow)?;

        Ok(())
    }
}