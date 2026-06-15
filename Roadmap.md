# Phase 1 — Stablecoin Core Foundation (MVP)

This is the REAL protocol core.

## Goal

Build a fully working decentralized stablecoin system using SOL collateral.

## Features

Vault System

Each user gets a vault PDA containing:

- deposited SOL
- minted debt
- collateral ratio
- health factor

## Collateral Management

- ✅ Deposit SOL collateral
- ✅ Withdraw SOL collateral

## Stablecoin System

- ✅ Mint stablecoin
- ✅ Burn/repay stablecoin

## Debt Accounting

### Track:

- total protocol debt
- user debt
- stablecoin supply
- Health Factor System

## Health Factor=

(Collateral Value \* Liquidation Threshold) / Debt Value
​

##### Used to determine:

- vault safety
- liquidation eligibility

## Oracle Integration

#### Use:

- Pyth Network

#### Fetch:

- SOL/USD price

#### Add:

- stale price checks
- oracle confidence checks

## Liquidation Engine

If vault becomes unhealthy:

- liquidator repays debt
- receives discounted collateral

This protects protocol solvency.

## Treasury Accounting

#### Track:

- protocol fees
- liquidation penalties
- reserve balances

## Security

- ✅ PDA authority model
- ✅ signer validation
- ✅ overflow-safe math
- ✅ access controls
- ✅ emergency pause

# Phase 2 — Protocol Improvements

Improve realism and protocol quality.

### Partial Liquidations

Instead of:

```
liquidate entire vault
```

### Allow:

```
liquidate only required debt amount
```

This is more capital efficient.

## Stability Fees

Charge borrowing fees over time.

### Example:

```
Borrow 100 stablecoins
↓
Repay 102 later
```

### Learn:

- interest accrual
- treasury revenue
- debt indexing
- Better Risk Engine

### Add:

- configurable collateral ratios
- liquidation bonus parameters
- liquidation thresholds
- Protocol Events

### Emit events/logs for:

- deposits
- minting
- liquidations
- repayments

Useful for indexers/frontend.

# Phase 3 — Yield-Bearing Collateral

This is the BEST advanced Solana-native feature.

### Add Liquid Staking Token Collateral

#### Support:

- JitoSOL
- mSOL

#### Users can:

```
earn staking yield + mint stablecoin
```

### Oracle Expansion

Add LST pricing support.

#### Example:

```
JitoSOL/SOL
SOL/USD
```

### Multi-Collateral Vault Logic

#### Vaults can hold:

- SOL
- JitoSOL
- mSOL

#### Learn:

- weighted collateral valuation
- multi-asset accounting

# Phase 4 — Production Readiness

Focus on robustness.

## Advanced Security

✅ circuit breaker
✅ protocol pause
✅ liquidation cooldowns
✅ admin multi-sig support

## Testing

- unit tests
- integration tests
- liquidation simulations
- oracle manipulation tests
- Infrastructure

## (Optional)

- liquidation bots
- backend indexer
- monitoring dashboard
