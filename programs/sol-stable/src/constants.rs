//! constants.rs
//!
//! Global protocol constants.
//!
//! This file contains:
//! - PDA seeds
//! - Protocol configuration constants
//! - Financial constants
//! - Time constants
//!
//! Keeping all constants here avoids magic numbers throughout the codebase.

use anchor_lang::prelude::*;

/// ----------------------------
/// PDA Seeds
/// ----------------------------

/// Global protocol configuration PDA.
pub const PROTOCOL_CONFIG_SEED: &[u8] = b"protocol_config";

/// Global protocol state PDA.
pub const PROTOCOL_STATE_SEED: &[u8] = b"protocol_state";

/// Treasury PDA.
pub const TREASURY_SEED: &[u8] = b"treasury";

/// User vault PDA.
pub const VAULT_SEED: &[u8] = b"vault";

/// PDA that stores SOL collateral.
pub const COLLATERAL_VAULT_SEED: &[u8] = b"collateral_vault";

/// Stablecoin mint authority PDA.
pub const MINT_AUTHORITY_SEED: &[u8] = b"mint_authority";

/// ----------------------------
/// Protocol Constants
/// ----------------------------

/// Stablecoin decimals.
pub const STABLECOIN_DECIMALS: u8 = 6;

/// Basis points denominator.
///
/// 100% = 10_000 BPS
pub const BPS_DENOMINATOR: u16 = 10_000;

/// Initial minimum collateral ratio.
///
/// 150%
pub const DEFAULT_MIN_COLLATERAL_RATIO_BPS: u16 = 15_000;

/// Liquidation threshold.
///
/// 120%
pub const DEFAULT_LIQUIDATION_THRESHOLD_BPS: u16 = 12_000;

/// Liquidation bonus.
///
/// 5%
pub const DEFAULT_LIQUIDATION_BONUS_BPS: u16 = 500;

/// Initial stability fee.
///
/// Disabled for Phase 1.
pub const DEFAULT_STABILITY_FEE_BPS: u16 = 0;

/// ----------------------------
/// Time Constants
/// ----------------------------

/// Seconds in one day.
pub const SECONDS_PER_DAY: i64 = 86_400;

/// Seconds in one year.
///
/// Used later for stability fee calculations.
pub const SECONDS_PER_YEAR: i64 = 31_536_000;

/// ----------------------------
/// Account Limits
/// ----------------------------

/// Maximum protocol name length.
pub const MAX_PROTOCOL_NAME_LENGTH: usize = 32;

/// Maximum symbol length.
pub const MAX_SYMBOL_LENGTH: usize = 10;

/// ----------------------------
/// Treasury
/// ----------------------------

/// Minimum lamports kept inside treasury.
///
/// Prevents accidental account deallocation.
pub const TREASURY_MIN_BALANCE: u64 = 1_000_000;

/// ----------------------------
/// Events
/// ----------------------------

/// Protocol version.
pub const PROTOCOL_VERSION: u8 = 1;