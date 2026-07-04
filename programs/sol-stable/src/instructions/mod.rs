//! Central module for all instruction handlers.
//!
//! Instructions are grouped by feature instead of keeping every
//! instruction in a single directory. As the protocol grows this
//! makes navigation much easier.
//!
//! Phase 1
//! --------
//! admin/
//!     initialize_protocol
//!
//! vault/
//!     create_vault
//!     deposit_collateral
//!     withdraw_collateral
//!
//! stablecoin/
//!     mint_stablecoin
//!     burn_stablecoin
//!
//! Future
//! ------
//! liquidation/
//! governance/
//! oracle/

pub mod admin;
pub mod stablecoin;
pub mod vault;

// Re-export instruction handlers.
pub use admin::*;
pub use stablecoin::*;
pub use vault::*;