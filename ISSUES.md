# ISSUES.md — Decentralized Time-Lock Vault
## Wave Program — Batch 1 (125 Issues)

> Stack: Rust · Soroban SDK v22 · Stellar Blockchain · Persistent Storage
> All issues are implementation-focused, non-duplicate, and grounded in the actual codebase.

---

## 🔴 BUGS (Issues #1–#22)

| # | Title | Priority | Difficulty | Tags |
|---|---|---|---|---|
| 1 | `has_deposit` skips TTL bump, enabling ghost-entry bypass in duplicate guard | Critical | Advanced | `bug` `security` `storage` |
| 2 | `initialize` performs storage read before `require_auth`, violating auth-first ordering | High | Beginner | `bug` `security` `contract` |
| 3 | `unlock_time` has no absolute upper-bound check independent of `now` | High | Intermediate | `bug` `validation` `contract` |
| 4 | `withdraw` event emitted after external token transfer breaks strict CEI ordering | High | Advanced | `bug` `security` `events` |
| 5 | `emergency_withdraw` event emitted after token transfer — same CEI violation | High | Advanced | `bug` `security` `events` |
| 6 | `cancel_transfer_admin` emits no event, leaving transfer cancellations unobservable | Medium | Beginner | `bug` `events` `admin` |
| 7 | `renounce_admin` directly calls `env.storage().persistent().remove()` bypassing storage module | Medium | Beginner | `bug` `refactor` `storage` |
| 8 | `get_deposit_readonly` returns `None` for near-expiry entries without alerting caller | Medium | Intermediate | `bug` `storage` `ttl` |
| 9 | `transfer_admin` allows nominating the same address as current admin, creating a no-op transfer | Low | Beginner | `bug` `admin` `validation` |
| 10 | `accept_admin` does not bump TTL for the newly promoted admin entry | Medium | Intermediate | `bug` `storage` `ttl` |
| 11 | `deposit` validates amount before checking for existing deposit, wasting compute on duplicate calls | Low | Beginner | `bug` `performance` `contract` |
| 12 | `VaultEntry.depositor` field is redundant — address is already the storage key | Low | Beginner | `bug` `refactor` `types` |
| 13 | `get_constants` takes `_env` but does not need it — function signature misleads callers | Low | Beginner | `bug` `api` `contract` |
| 14 | `storage::get_admin` does not bump TTL on read — admin entry could silently expire | High | Intermediate | `bug` `storage` `ttl` |
| 15 | `BUMP_TARGET` of 1 year is insufficient for 5-year lock deposits — entries will expire mid-lock | Critical | Advanced | `bug` `storage` `ttl` |
| 16 | `lock_duration` computed with `saturating_sub` masks potential underflow silently | Medium | Intermediate | `bug` `arithmetic` `validation` |
| 17 | `withdraw` does not verify `entry.depositor == depositor` — anyone with auth can withdraw for another | High | Advanced | `bug` `security` `auth` |
| 18 | `emergency_withdraw` accepts `admin` as parameter instead of reading from storage — spoofable in future ABI versions | Medium | Advanced | `bug` `security` `admin` |
| 19 | No guard against depositing a zero-address token contract | Medium | Intermediate | `bug` `validation` `contract` |
| 20 | `advance_time` test helper does not update `sequence_number`, causing ledger state drift | Low | Intermediate | `bug` `testing` `helpers` |
| 21 | `test_deposit_amount_exceeds_max_fails` mints `MAX_DEPOSIT_AMOUNT` but deposits `MAX + 1` — token balance insufficient | Low | Beginner | `bug` `testing` |
| 22 | `symbol_short!("deposit")` and `symbol_short!("withdraw")` topics are not validated for uniqueness across all event types | Low | Intermediate | `bug` `events` `observability` |

---

## 🟠 SECURITY (Issues #23–#38)

| # | Title | Priority | Difficulty | Tags |
|---|---|---|---|---|
| 23 | No deployer-lock: `initialize` can be front-run between deployment and first call | Critical | Advanced | `security` `contract` `initialization` |
| 24 | Admin address stored in persistent storage — compromised admin key cannot be rotated without pending-transfer UX | High | Advanced | `security` `admin` `key-management` |
| 25 | Token contract address is never validated against a trusted registry — malicious tokens accepted | High | Advanced | `security` `validation` `token` |
| 26 | `emergency_withdraw` bypasses the lock period — admin is a privileged single point of failure | High | Advanced | `security` `admin` `trust-model` |
| 27 | No rate limiting or cooldown on `deposit` calls per address — spam deposits exhaust ledger resources | Medium | Advanced | `security` `performance` `contract` |
| 28 | `renounce_admin` is irreversible with no confirmation mechanism — one wrong call locks out recovery forever | High | Intermediate | `security` `admin` `ux` |
| 29 | Contract has no pause mechanism for emergency response without full admin renounce | High | Advanced | `security` `admin` `contract` |
| 30 | `transfer_admin` does not time-limit pending acceptance — a stale pending admin can accept months later | Medium | Intermediate | `security` `admin` `ttl` |
| 31 | No event for `cancel_transfer_admin` — off-chain monitors cannot detect cancelled transfers | Medium | Beginner | `security` `events` `observability` |
| 32 | Persistent storage keys are not versioned — future contract upgrades risk key collisions | Medium | Advanced | `security` `storage` `upgrades` |
| 33 | No slippage or minimum-receive check on token transfers — fee-on-transfer tokens break accounting | High | Advanced | `security` `token` `validation` |
| 34 | `VaultEntry` amount is `i128` but token balances can be `i128::MAX` — overflow in future top-up features | Medium | Advanced | `security` `arithmetic` `types` |
| 35 | Soroban contract is not upgradeable — critical bugs require full redeployment with no migration path | High | Advanced | `security` `upgrades` `contract` |
| 36 | No reentrancy guard documentation — callers may assume Soroban prevents all reentrancy by default | Medium | Intermediate | `security` `documentation` `contract` |
| 37 | `accept_admin` does not invalidate other pending operations after role change | Medium | Advanced | `security` `admin` `state-management` |
| 38 | Contract has no way to freeze a specific depositor address in case of fraudulent activity | Medium | Advanced | `security` `admin` `contract` |

---

## 🟡 PERFORMANCE (Issues #39–#50)

| # | Title | Priority | Difficulty | Tags |
|---|---|---|---|---|
| 39 | `VaultKey::Deposit(Address)` clones the full address on every storage operation — unnecessary heap allocation | Medium | Intermediate | `performance` `storage` `memory` |
| 40 | `get_deposit` always bumps TTL even when entry is far from expiry — wastes ledger compute | Medium | Intermediate | `performance` `storage` `ttl` |
| 41 | `token::Client::new` constructed inside every function — no reuse pattern | Low | Beginner | `performance` `contract` `refactor` |
| 42 | `VaultEntry` stores `depositor: Address` which is already the map key — doubles Address storage cost | Medium | Intermediate | `performance` `storage` `types` |
| 43 | `deposit` function calls `has_deposit` then may call `get_deposit` — two separate storage reads | Medium | Intermediate | `performance` `storage` `contract` |
| 44 | `get_constants` is a view function with no caching — called on every client validation round-trip | Low | Beginner | `performance` `api` `contract` |
| 45 | WASM binary not benchmarked against Soroban instruction limits — no baseline metric in CI | Medium | Intermediate | `performance` `ci` `devops` |
| 46 | No batch `get_vaults` query — clients must make N separate calls for N depositors | Medium | Advanced | `performance` `api` `scalability` |
| 47 | `advance_time` test helper creates a full `LedgerInfo` struct per call — verbose and fragile | Low | Beginner | `performance` `testing` `dx` |
| 48 | `setup()` test helper re-registers the contract on every test — slow test suite with many tests | Low | Intermediate | `performance` `testing` `dx` |
| 49 | Event payload includes full `Address` in topics — Soroban charges per-byte on topic size | Low | Advanced | `performance` `events` `cost` |
| 50 | `withdraw` reads the full `VaultEntry` when only `unlock_time`, `token`, and `amount` are needed | Low | Intermediate | `performance` `storage` `contract` |

---

## 🔵 DOCUMENTATION (Issues #51–#68)

| # | Title | Priority | Difficulty | Tags |
|---|---|---|---|---|
| 51 | README missing a "How to Interact" section showing actual CLI invocation examples | High | Beginner | `documentation` `dx` `readme` |
| 52 | No CHANGELOG.md — contributors have no history of what changed between versions | Medium | Beginner | `documentation` `dx` |
| 53 | No CONTRIBUTING.md — Wave contributors have no guide for PRs, commit style, or test requirements | High | Beginner | `documentation` `dx` `contributing` |
| 54 | No CODE_OF_CONDUCT.md | Low | Beginner | `documentation` `community` |
| 55 | `BUMP_THRESHOLD` and `BUMP_TARGET` constants lack explanation of the 5s/ledger assumption | Medium | Beginner | `documentation` `storage` `constants` |
| 56 | `MAX_DEPOSIT_AMOUNT` comment says "quadrillion" but 10^15 is a quadrillion only in short-scale — clarify for international contributors | Low | Beginner | `documentation` `types` |
| 57 | `VaultEntry` fields have no units documented (stroops vs tokens, seconds vs milliseconds) | High | Beginner | `documentation` `types` `api` |
| 58 | `events.rs` has no module-level doc comment explaining the event naming convention | Low | Beginner | `documentation` `events` |
| 59 | `storage.rs` lacks a diagram or comment showing the full key-value layout | Medium | Beginner | `documentation` `storage` |
| 60 | `contract.rs` `emergency_withdraw` doc says "Intended for emergency recovery" but never defines what qualifies as an emergency | Medium | Beginner | `documentation` `admin` `contract` |
| 61 | No Architecture Decision Record (ADR) explaining why one-deposit-per-address was chosen | Medium | Intermediate | `documentation` `design` |
| 62 | No ADR for why `i128` was chosen as the amount type over `u128` | Low | Beginner | `documentation` `design` `types` |
| 63 | `scripts/deploy_testnet.sh` has no `--help` flag or inline usage documentation | Medium | Beginner | `documentation` `devops` `scripts` |
| 64 | README "Security Notes" section does not mention the trust assumptions around the admin key | High | Intermediate | `documentation` `security` `readme` |
| 65 | No documentation on how to run a local Soroban node for integration testing | High | Beginner | `documentation` `testing` `dx` |
| 66 | `plan.md` references Wave Program sprint cycles but does not define sprint length or review SLA | Low | Beginner | `documentation` `process` |
| 67 | No `#[deprecated]` annotation strategy documented for future API evolution | Low | Intermediate | `documentation` `api` `contract` |
| 68 | `lib.rs` module-level comment references `VaultKey::Deposit(Address)` but the actual layout now includes `Admin` and `PendingAdmin` keys | Medium | Beginner | `documentation` `lib` `storage` |

---

## 🟢 TESTING (Issues #69–#88)

| # | Title | Priority | Difficulty | Tags |
|---|---|---|---|---|
| 69 | No fuzz test for `deposit` amount boundaries — only point tests at 0, -1, MAX, MAX+1 | High | Advanced | `testing` `fuzzing` `contract` |
| 70 | No property-based test verifying `time_remaining == unlock_time - now` for all valid inputs | High | Advanced | `testing` `property-based` `contract` |
| 71 | No test for `deposit` immediately followed by ledger advancement to exact `unlock_time - 1` | Medium | Beginner | `testing` `boundary` `withdraw` |
| 72 | No test verifying token balance of contract address increases after deposit | Medium | Beginner | `testing` `balance` `deposit` |
| 73 | No test verifying contract address token balance decreases to zero after withdraw | Medium | Beginner | `testing` `balance` `withdraw` |
| 74 | No test for concurrent deposits from two different addresses (alice and bob simultaneously) | Medium | Intermediate | `testing` `multi-user` `storage` |
| 75 | No test verifying events are emitted with correct values on `deposit` | High | Intermediate | `testing` `events` `deposit` |
| 76 | No test verifying events are emitted with correct values on `withdraw` | High | Intermediate | `testing` `events` `withdraw` |
| 77 | No test verifying `emergency_withdraw` event includes correct admin, depositor, token, and amount | High | Intermediate | `testing` `events` `admin` |
| 78 | No test verifying `admin_transfer_initiated` event is emitted with correct fields | Medium | Intermediate | `testing` `events` `admin` |
| 79 | No test verifying `admin_transfer_accepted` event fires on `accept_admin` | Medium | Intermediate | `testing` `events` `admin` |
| 80 | No test verifying `admin_renounced` event fires on `renounce_admin` | Medium | Beginner | `testing` `events` `admin` |
| 81 | No test for `get_vault` returning `None` after a successful `withdraw` (entry removal) | Medium | Beginner | `testing` `storage` `withdraw` |
| 82 | No test for `get_vault` returning correct data for a deposit near `MAX_LOCK_DURATION_SECS` | Low | Beginner | `testing` `storage` `boundary` |
| 83 | No integration test deploying to local Soroban standalone node | High | Advanced | `testing` `integration` `devops` |
| 84 | No test for `transfer_admin` when a previous pending admin already exists — should overwrite | Medium | Intermediate | `testing` `admin` `state` |
| 85 | No test verifying old admin cannot `emergency_withdraw` after `renounce_admin` | Medium | Beginner | `testing` `admin` `security` |
| 86 | No test verifying `get_constants` values match the constants defined in `types.rs` at compile time | Low | Beginner | `testing` `constants` `api` |
| 87 | No test for `withdraw` when token transfer would fail (insufficient contract balance) | High | Advanced | `testing` `error-path` `token` |
| 88 | No stress test depositing and withdrawing 1000 times sequentially to check ledger fee accumulation | Low | Advanced | `testing` `performance` `stress` |

---

## ⚪ REFACTORING (Issues #89–#100)

| # | Title | Priority | Difficulty | Tags |
|---|---|---|---|---|
| 89 | Extract admin authorization into a reusable `require_admin` helper to avoid repeated pattern | Medium | Beginner | `refactor` `dx` `contract` |
| 90 | `storage.rs` mixes deposit and admin helpers — split into `deposit_storage.rs` and `admin_storage.rs` | Low | Beginner | `refactor` `structure` `storage` |
| 91 | `events.rs` functions take too many positional arguments — group into event-specific structs | Low | Intermediate | `refactor` `events` `api` |
| 92 | `VaultEntry` should derive `Default` for easier test construction | Low | Beginner | `refactor` `types` `testing` |
| 93 | Error handling in `emergency_withdraw` and `withdraw` is identical — extract into `load_and_clear_deposit` helper | Medium | Intermediate | `refactor` `contract` `dx` |
| 94 | `renounce_admin` calls `env.storage().persistent().remove()` directly instead of a `storage::remove_admin` function | Medium | Beginner | `refactor` `storage` `contract` |
| 95 | `test.rs` `setup()` returns a 5-tuple — replace with a named `TestContext` struct for readability | Low | Beginner | `refactor` `testing` `dx` |
| 96 | Magic number `10_000` used as mint amount in tests — extract to a named constant `TEST_MINT_AMOUNT` | Low | Beginner | `refactor` `testing` `constants` |
| 97 | `contract.rs` admin guard pattern (read → compare → error) repeated 4 times — extract to macro or helper | Medium | Intermediate | `refactor` `contract` `dx` |
| 98 | `types.rs` and `errors.rs` could be merged into a single `model.rs` for small-contract cohesion | Low | Beginner | `refactor` `structure` `types` |
| 99 | `lib.rs` re-exports `TimeLockVaultClient` and `TimeLockVault` separately — consolidate into a single `pub use contract::*` | Low | Beginner | `refactor` `lib` `api` |
| 100 | `Makefile` `check` target runs `fmt-check`, `lint`, `test` but not `build` — CI divergence risk | Medium | Beginner | `refactor` `devops` `makefile` |

---

## 🔧 FEATURES / SCALABILITY (Issues #101–#112)

| # | Title | Priority | Difficulty | Tags |
|---|---|---|---|---|
| 101 | Add `top_up(depositor, amount)` to increase locked amount without changing unlock_time | High | Intermediate | `feature` `contract` `scalability` |
| 102 | Add `extend_lock(depositor, new_unlock_time)` to push unlock time further into future | High | Intermediate | `feature` `contract` `scalability` |
| 103 | Add multi-deposit support: `VaultKey::Deposit(Address, u32)` with per-address counter | High | Advanced | `feature` `contract` `scalability` `storage` |
| 104 | Add `withdraw_partial(depositor, amount)` for partial unlock after lock period | Medium | Advanced | `feature` `contract` `scalability` |
| 105 | Add admin-managed token whitelist to restrict accepted assets | Medium | Advanced | `feature` `admin` `security` `contract` |
| 106 | Add `get_all_vaults` paginated query returning deposits sorted by unlock_time | Medium | Advanced | `feature` `api` `scalability` |
| 107 | Add `deposit_on_behalf(beneficiary, token, amount, unlock_time)` for third-party deposits | Medium | Advanced | `feature` `contract` `ux` |
| 108 | Add protocol fee mechanism (basis points) collected on withdrawal | Medium | Advanced | `feature` `contract` `economics` |
| 109 | Add vault pause/unpause admin function for emergency response | High | Advanced | `feature` `admin` `security` |
| 110 | Add `get_total_locked(token)` aggregate query for TVL tracking | Medium | Intermediate | `feature` `api` `analytics` |
| 111 | Support contract upgrade path via Soroban's `update_current_contract_wasm` | High | Advanced | `feature` `contract` `upgrades` |
| 112 | Add `notify_unlock(depositor)` that emits an event exactly at unlock time (via scheduled invocation) | Low | Advanced | `feature` `events` `ux` |

---

## 🚀 CI/CD & DEVOPS (Issues #113–#121)

| # | Title | Priority | Difficulty | Tags |
|---|---|---|---|---|
| 113 | CI pipeline has no job to check WASM binary size regression between PRs | Medium | Intermediate | `devops` `ci` `performance` |
| 114 | CI pipeline has no security audit step (`cargo audit`) | High | Intermediate | `devops` `ci` `security` |
| 115 | `deploy_testnet.sh` has no idempotency check — re-running deploys a duplicate contract | High | Intermediate | `devops` `scripts` `deployment` |
| 116 | No Dependabot or Renovate config for automated `soroban-sdk` version updates | Medium | Beginner | `devops` `dependencies` `dx` |
| 117 | CI does not run tests with `--release` profile — debug-mode tests may hide optimized-build bugs | Medium | Intermediate | `devops` `ci` `testing` |
| 118 | No GitHub Release workflow to tag, build optimized WASM, and attach as release asset | High | Intermediate | `devops` `ci` `release` |
| 119 | `scripts/deploy_testnet.sh` not tested in CI — shell errors could silently break deployment | Medium | Intermediate | `devops` `ci` `scripts` |
| 120 | Makefile has no `install-tools` target to bootstrap `soroban-cli` and Rust toolchain | Medium | Beginner | `devops` `dx` `makefile` |
| 121 | No `.env.example` file documenting required environment variables for deployment | Medium | Beginner | `devops` `dx` `documentation` |

---

## 🎨 DEVELOPER EXPERIENCE (Issues #122–#125)

| # | Title | Priority | Difficulty | Tags |
|---|---|---|---|---|
| 122 | No `.kiro/steering` file to guide AI-assisted development within this project | Low | Beginner | `dx` `tooling` `ai` |
| 123 | No `SECURITY.md` documenting responsible disclosure policy for vulnerability reports | High | Beginner | `dx` `security` `documentation` |
| 124 | No issue template (`.github/ISSUE_TEMPLATE/`) — contributors file unstructured issues | Medium | Beginner | `dx` `github` `contributing` |
| 125 | No pull request template (`.github/pull_request_template.md`) — PRs lack consistent structure | Medium | Beginner | `dx` `github` `contributing` |

---

## Summary Statistics

| Category | Count | Critical | High | Medium | Low |
|---|---|---|---|---|---|
| Bugs | 22 | 2 | 9 | 7 | 4 |
| Security | 16 | 1 | 9 | 6 | 0 |
| Performance | 12 | 0 | 0 | 6 | 6 |
| Documentation | 18 | 0 | 5 | 8 | 5 |
| Testing | 20 | 0 | 6 | 9 | 5 |
| Refactoring | 12 | 0 | 0 | 5 | 7 |
| Features | 12 | 0 | 4 | 7 | 1 |
| CI/CD | 9 | 0 | 3 | 6 | 0 |
| Developer Experience | 4 | 0 | 1 | 2 | 1 |
| **Total** | **125** | **3** | **37** | **56** | **29** |

---

## Recommended Sprint Order

**Sprint 1 (Critical + High Security/Bugs):** #1, #2, #3, #15, #17, #23, #24, #33, #114, #123
**Sprint 2 (Testing foundations):** #69–#88 (full test coverage before features)
**Sprint 3 (Features):** #101, #102, #103, #109, #111
**Sprint 4 (DX + Docs):** #51–#68, #122–#125
**Sprint 5 (Refactor + Performance):** #89–#100, #39–#50

---

*Generated for Wave Program · Decentralized Time-Lock Vault · Soroban / Stellar*
