//! state/protocol_state.rs
//!
//! Global mutable protocol state.
//!
//! Unlike `ProtocolConfig`, which stores protocol parameters,
//! this account stores data that changes frequently as users
//! interact with the protocol.
//!
//! PDA Seed:
//!     PROTOCOL_STATE_SEED

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProtocolState {
    /// Stablecoin mint managed by the protocol.
    ///
    /// This is the SPL Token mint that users receive when
    /// borrowing against their collateral.
    pub stable_coin_mint: Pubkey,

    /// Treasury PDA.
    ///
    /// Stores protocol-owned funds such as:
    /// - stability fees (Phase 2)
    /// - liquidation penalties (Phase 2)
    pub treasury_pda: Pubkey,

    /// Total collateral deposited into the protocol.
    ///
    /// Phase 1:
    /// Tracks SOL collateral only.
    ///
    /// Future:
    /// Tracks all supported collateral assets.
    pub total_collateral: u64,

    /// Total outstanding stablecoin debt.
    ///
    /// Every mint increases this value.
    /// Every burn decreases this value.
    pub total_debt: u64,

    /// Emergency pause flag.
    ///
    /// When true:
    /// - deposits disabled
    /// - withdrawals disabled
    /// - minting disabled
    ///
    /// Liquidations may still remain enabled depending on
    /// protocol policy (implemented in later phases).
    pub paused: bool,

    pub bump: u8,
}

impl ProtocolState {
    /// Account size including Anchor discriminator.
    pub const LENGTH: usize = 8 + Self::INIT_SPACE;

    /// Returns true if the protocol is currently paused.
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Increase protocol collateral.
    pub fn increase_collateral(&mut self, amount: u64) -> Result<()> {
        self.total_collateral = self.total_collateral
            .checked_add(amount)
            .ok_or(crate::errors::SolStableErrors::MathOverflow)?;

        Ok(())
    }

    /// Decrease protocol collateral.
    pub fn decrease_collateral(&mut self, amount: u64) -> Result<()> {
        self.total_collateral = self.total_collateral
            .checked_sub(amount)
            .ok_or(crate::errors::SolStableErrors::MathUnderflow)?;

        Ok(())
    }

    /// Increase protocol debt.
    pub fn increase_debt(&mut self, amount: u64) -> Result<()> {
        self.total_debt = self.total_debt
            .checked_add(amount)
            .ok_or(crate::errors::SolStableErrors::MathOverflow)?;
        
        Ok(())
    }

    /// Decrease protocol debt.
    pub fn decrease_debt(&mut self, amount: u64) -> Result<()> {
        self.total_debt = self.total_debt
            .checked_sub(amount)
            .ok_or(crate::errors::SolStableErrors::MathUnderflow)?;
        
        Ok(())
    }
}