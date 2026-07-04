//! state/vault.rs
//!
//! User collateral vault.
//!
//! Every borrower owns a Vault account that tracks:
//! - deposited collateral
//! - outstanding debt
//! - timestamps
//!
//! NOTE:
//! This account DOES NOT store SOL.
//!
//! Actual SOL collateral is stored inside a separate
//! Collateral Vault PDA.
//!
//! This account is only responsible for protocol accounting.

use core::time;

use anchor_lang::prelude::*;

/// Supported collateral assets.
///
/// Phase 1:
/// Only SOL is supported.
///
/// Future phases:
/// - JitoSOL
/// - mSOL
/// - Other Solana-native collateral
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, PartialEq, Eq)]
pub enum CollateralAsset {
    Sol,
}

/// User vault.
///
/// PDA:
/// seeds = [VAULT_SEED, owner]
#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub owner: Pubkey,

    /// Type of collateral deposited.
    ///
    /// Phase 1:
    /// Always CollateralAsset::Sol
    pub collateral_asset: CollateralAsset,

    /// Amount of collateral deposited.
    /// Stored in lamports.
    pub collateral_amount: u64,

    /// Outstanding stablecoin debt.
    pub debt_amount: u64,

    /// Vault identifier.
    ///
    /// Phase 1:
    /// Always 0.
    ///
    /// Future:
    /// Enables multiple vaults per user.
    pub vault_number: u64,

    /// Unix timestamp when the vault was created.
    pub created_at: i64,

    /// Unix timestamp of the most recent vault update.
    ///
    /// Future:
    /// Used for stability fee accrual.
    pub last_updated: i64,

    pub bump: u8
}

impl Vault {
    /// Total account size.
    pub const LENGTH: usize = 8 + Self::INIT_SPACE;

    /// Increase collateral.
    pub fn deposit_collateral(&mut self, amount: u64) -> Result<()> {
        self.collateral_amount = self.collateral_amount
            .checked_add(amount)
            .ok_or(crate::errors::SolStableErrors::MathOverflow)?;

        Ok(())
    }

    /// Decrease collateral.
    pub fn withdraw_collateral(&mut self, amount: u64) -> Result<()> {
        self.collateral_amount = self.collateral_amount
            .checked_sub(amount)
            .ok_or(crate::errors::SolStableErrors::MathUnderflow)?;
        
        Ok(())
    }

    pub fn increase_debt(&mut self, amount: u64) -> Result<()> {
        self.debt_amount = self.debt_amount
            .checked_add(amount)
            .ok_or(crate::errors::SolStableErrors::MathOverflow)?;

        Ok(())
    }

    pub fn decrease_debt(&mut self, amount: u64) -> Result<()> {
        self.debt_amount = self.debt_amount
            .checked_sub(amount)
            .ok_or(crate::errors::SolStableErrors::MathUnderflow)?;

        Ok(())
    }

    // update last modification timestamp
    pub fn touch(&mut self, timestamp: i64) -> Result<()> {
        self.last_updated = timestamp;

        Ok(())
    }

    /// Returns true if the vault currently has debt.
    pub fn has_debt(&self) -> bool {
        self.debt_amount > 0
    }

    /// Returns true if the vault contains collateral.
    pub fn has_collateral(&self) -> bool {
        self.collateral_amount > 0
    }
}
