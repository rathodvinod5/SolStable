use anchor_lang::prelude::*;

#[error_code]
pub enum SolStableErrors {
    // Authorization
    #[msg("Unathoirzed")]
    UnAuthorized,

    #[msg("Porotocol is currently paused")]
    ProtocolPaused,

    // Vault
    #[msg("Vault Already exists")]
    VaultExists,

    #[msg("Vault not found")]
    VaultNotFound,

    #[msg("Invalid owner")]
    InvalidOwner,

    // Collateral
    #[msg("Collateral amount must be greater than zero")]
    InvalidCollateralAmount,

    #[msg("Insufficient Collateral")]
    InsufficientCollateral,

    #[msg("Withdrawal would make the vault undercollateralized")]
    InsufficientCollateralRatio,

    // StableCoin
    #[msg("Mint amount must be greater than zero")]
    InvalidMintAmount,

    #[msg("Burn amount must be greater than zero")]
    InvalidBurnAmount,

    #[msg("Insufficient debt to repay")]
    InsufficientDebt,

    // Health Factor / Risk
    #[msg("Vault is healthy and cannot be liquidated")]
    VaultHealthy,

    #[msg("Vault is undercollateralized")]
    VaultUnhealthy,

    #[msg("Health factor is below the minimum threshold")]
    HealthFactoTooLow,

    // Oracle
    #[msg("Invalid oracle account")]
    InvalidOracleAccount,

    #[msg("Oracle price is stale")]
    StaleOraclePrice,

    #[msg("nvalid oracle price")]
    InvalidOraclePrice,

    // Treasury
    #[msg("Treasury balance is insufficient")]
    TreasuryInsufficientBalance,

    // Math
     #[msg("Arithmetic overflow")]
    MathOverflow,

    #[msg("Arithmetic underflow")]
    MathUnderflow,

    #[msg("Division by zero")]
    DivisionByZero,

    #[msg("Invalid protocol configuration")]
    InvalidConfiguration,

    // Accounts
    #[msg("Invalid PDA")]
    InvalidPda,

    #[msg("Invalid account")]
    InvalidAccount,

    #[msg("Account constraint violated")]
    InvalidAccountConstraint,

    // Generic
    #[msg("Feature not implemented yet")]
    NotImplemented,
}