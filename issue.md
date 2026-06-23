# Issue Template — Decentralized Time-Lock Vault

---

## [BUG] `deposit` allows front-running between `has_deposit` check and `set_deposit` write

**Labels:** `bug` `security` `advanced`
**Priority:** 🔴 Critical
**Difficulty:** Advanced
**Tags:** `contract` `storage` `security` `atomicity`

---

### Description

In `contract.rs`, the `deposit` function performs a read-check-write sequence that is not atomic at the application level:

```rust
if storage::has_deposit(&env, &depositor) {
    return Err(VaultError::DepositAlreadyExists);
}
// ← window here
token_client.transfer(...);
storage::set_deposit(&env, &depositor, &entry);
```

While Soroban transactions are atomic within a single ledger, a carefully timed concurrent transaction from the same address — or a contract that calls `deposit` twice in a single invoke chain — could bypass the `has_deposit` guard if Soroban's execution model does not enforce per-address mutex semantics across re-entrant invocations.

Additionally, `has_deposit` does not bump the TTL of the entry it reads. If the deposit entry is within 30 days of expiry (below `BUMP_THRESHOLD`), the entry could expire between `has_deposit` returning `false` and `set_deposit` executing, silently allowing a re-deposit on what was technically a live-but-expired entry.

---

### Reproduction Steps

1. Deploy the contract to a local Soroban testnet
2. Create a deposit for `alice` with a 1-year lock
3. Wait until the entry TTL is within the `BUMP_THRESHOLD` window (≈518,400 ledgers from expiry)
4. Call `deposit` again for `alice` without withdrawing first
5. Observe: `has_deposit` may return `false` on an expired-but-not-removed entry, allowing a second deposit to overwrite the first without error

---

### Expected Behavior

`deposit` must return `VaultError::DepositAlreadyExists` for any address that has an active or recently-expired-but-not-withdrawn entry. The TTL check and duplicate guard must be consistent.

---

### Actual Behavior

`has_deposit` does not bump TTL. An entry past `BUMP_THRESHOLD` but not yet removed from storage could return `false` from `has()` while the underlying storage slot is in an indeterminate state, allowing `set_deposit` to silently overwrite a still-valid entry whose TTL was not refreshed.

---

### Technical Notes

- Soroban's `persistent().has()` returns `false` for entries whose TTL has expired at the ledger level, even if the data has not been explicitly removed
- `BUMP_THRESHOLD = 518_400` ledgers ≈ 30 days at 5s/ledger
- `set_deposit` correctly bumps TTL, but `has_deposit` does not
- The fix is to either: (a) bump TTL inside `has_deposit`, or (b) use `get_deposit` (which already bumps TTL) and check `is_some()` instead of calling `has_deposit` separately

---

### Acceptance Criteria

- [ ] `has_deposit` bumps TTL when returning `true`, or is replaced by a `get_deposit`-based check in the `deposit` function
- [ ] No double-deposit is possible for an address with a live vault, regardless of entry TTL state
- [ ] A regression test is added that deposits, advances ledger to near-TTL-expiry, and attempts a second deposit — expecting `DepositAlreadyExists`
- [ ] All 35 existing tests continue to pass
- [ ] Inline comment explains the TTL-consistency requirement

---

### Suggested Implementation

```rust
// In contract.rs deposit():
// Replace:
if storage::has_deposit(&env, &depositor) {
    return Err(VaultError::DepositAlreadyExists);
}

// With:
if storage::get_deposit(&env, &depositor).is_some() {
    return Err(VaultError::DepositAlreadyExists);
}
```

This ensures TTL is bumped during the check, making the guard consistent with the subsequent write.

---

*Filed as part of Wave Program Sprint — Decentralized Time-Lock Vault*
